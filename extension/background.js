// Background script - keeps model in memory across page reloads
let model = null;
let modelLoadPromise = null;

async function loadModel() {
  if (model) return true;
  if (modelLoadPromise) return modelLoadPromise;

  modelLoadPromise = (async () => {
    try {
      const url = chrome.runtime.getURL("model.json.gz");
      console.log("[BG] Loading model from:", url);

      const resp = await fetch(url);
      if (!resp.ok) throw new Error("HTTP " + resp.status);

      const blob = await resp.blob();
      console.log("[BG] Blob size:", blob.size);

      const ds = new DecompressionStream("gzip");
      const stream = blob.stream().pipeThrough(ds);
      const text = await new Response(stream).text();
      console.log("[BG] Decompressed size:", text.length);

      const data = JSON.parse(text);
      model = {
        version: data.version || "unknown",
        vocabulary: data.vocabulary,
        class_log_prior: data.class_log_prior,
        feature_log_prob: data.feature_log_prob,
        classes: data.classes,
      };

      console.log("[BG] Model", model.version, "loaded:", Object.keys(model.vocabulary).length, "features");
      return true;
    } catch (e) {
      console.error("[BG] Failed to load model:", e);
      return false;
    }
  })();

  return modelLoadPromise;
}

function transform(text) {
  if (!model) return null;
  const textLower = text.toLowerCase();
  const nFeatures = Object.keys(model.vocabulary).length;
  const features = new Array(nFeatures).fill(0);

  for (const token of textLower.split(/\s+/)) {
    const cleaned = token.replace(/\s/g, "");
    if (cleaned && model.vocabulary[cleaned] !== undefined) {
      features[model.vocabulary[cleaned]] = 1;
    }
  }

  const chars = [...textLower];
  for (let i = 0; i < chars.length - 2; i++) {
    const trigram = chars.slice(i, i + 3).join("");
    if (model.vocabulary[trigram] !== undefined) {
      features[model.vocabulary[trigram]] = 1;
    }
  }

  for (let i = 0; i < chars.length - 3; i++) {
    const quadgram = chars.slice(i, i + 4).join("");
    if (model.vocabulary[quadgram] !== undefined) {
      features[model.vocabulary[quadgram]] = 1;
    }
  }

  return features;
}

function predict(text) {
  if (!model) return { isSpam: false, prob: 0 };

  const features = transform(text);
  if (!features) return { isSpam: false, prob: 0 };

  const scores = model.classes.map((_, i) => {
    let s = model.class_log_prior[i];
    for (let j = 0; j < features.length; j++) {
      if (features[j] > 0) {
        s += model.feature_log_prob[i][j];
      } else {
        const prob = Math.exp(model.feature_log_prob[i][j]);
        s += Math.log(Math.max(1 - prob, 1e-300));
      }
    }
    return s;
  });

  const maxS = Math.max(...scores);
  const exps = scores.map(s => Math.exp(s - maxS));
  const sum = exps.reduce((a, b) => a + b, 0);

  const bestIdx = scores.indexOf(Math.max(...scores));
  const isSpam = model.classes[bestIdx] === 1;

  // Get probability of the PREDICTED class (not hardcoded index 1)
  const predictedProb = sum > 0 ? exps[bestIdx] / sum : 0;

  return {
    isSpam: isSpam,
    prob: predictedProb,
  };
}

// Start loading immediately
console.log("[BG] Background script started");
loadModel().then(ok => {
  console.log("[BG] Model load result:", ok);
});

// Listen for messages from content scripts
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  console.log("[BG] Received message:", request.action, "from:", sender.tab ? sender.tab.url : "unknown");

  if (request.action === "predict") {
    const result = predict(request.text);
    console.log("[BG] Predict result:", result);
    sendResponse(result);
  } else if (request.action === "status") {
    console.log("[BG] Status check - model loaded:", !!model, "version:", model ? model.version : "none");
    sendResponse({ loaded: !!model, version: model ? model.version : null });
  }
  return true;
});
