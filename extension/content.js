(function () {
  "use strict";

  if (window.__spamLabelerInjected) return;
  window.__spamLabelerInjected = true;

  const checkedItems = new Map();
  let toolbarCreated = false;
  const TOOLBAR_HEIGHT = 42;
  let modelLoaded = false;
  let modelVersion = "";

  // ── ML Model communication with background ───────────────────────

  function sendMessage(action, data) {
    return new Promise((resolve) => {
      try {
        const runtime = typeof browser !== "undefined" ? browser : chrome;
        const msg = Object.assign({ action: action }, data || {});
        console.log("[CS] Sending message:", msg);
        runtime.runtime.sendMessage(msg, function(response) {
          console.log("[CS] Received response:", response);
          resolve(response);
        });
      } catch (e) {
        console.error("[CS] sendMessage error:", e);
        resolve(null);
      }
    });
  }

  async function checkModelStatus() {
    const resp = await sendMessage("status");
    console.log("[CS] Status response:", resp);
    modelLoaded = resp && resp.loaded;
    if (modelLoaded && resp.version) {
      modelVersion = resp.version;
    }
    return modelLoaded;
  }

  async function predict(text) {
    const result = await sendMessage("predict", { text: text });
    return result || { isSpam: false, prob: 0 };
  }

  // ── Toolbar ──────────────────────────────────────────────────────

  function createToolbar() {
    if (toolbarCreated) return;
    toolbarCreated = true;

    const bar = document.createElement("div");
    bar.id = "spam-labeler-toolbar";
    bar.style.cssText = "position:fixed;top:0;left:0;right:0;z-index:999999;display:flex;align-items:center;gap:8px;padding:8px 16px;background:#1a1a2e;border-bottom:2px solid #333;font-family:system-ui,sans-serif;box-shadow:0 2px 8px rgba(0,0,0,0.3);";

    const label = document.createElement("span");
    label.textContent = "🏷️ Label:";
    label.style.cssText = "color:#fff;font-size:14px;font-weight:600;";

    const spamBtn = document.createElement("button");
    spamBtn.textContent = "🔴 SPAM";
    spamBtn.style.cssText = "padding:6px 16px;border:none;border-radius:6px;font-size:14px;font-weight:600;cursor:pointer;color:#fff;background:#e74c3c;";
    spamBtn.addEventListener("click", () => downloadBoth("spam"));

    const safeBtn = document.createElement("button");
    safeBtn.textContent = "🟢 SAFE";
    safeBtn.style.cssText = "padding:6px 16px;border:none;border-radius:6px;font-size:14px;font-weight:600;cursor:pointer;color:#fff;background:#27ae60;";
    safeBtn.addEventListener("click", () => downloadBoth("safe"));

    const countEl = document.createElement("span");
    countEl.id = "sl-count";
    countEl.textContent = "(0 selected)";
    countEl.style.cssText = "color:#aaa;font-size:13px;";

    const status = document.createElement("span");
    status.id = "sl-status";
    status.style.cssText = "color:#fff;font-size:13px;margin-left:auto;";

    // Model status indicator - square, near buttons
    const statusDot = document.createElement("span");
    statusDot.id = "sl-status-dot";
    statusDot.style.cssText = "display:inline-flex;align-items:center;justify-content:center;width:44px;height:26px;border-radius:4px;background:#ddd;margin-left:8px;flex-shrink:0;";
    statusDot.title = "Model not loaded";
    const dotText = document.createElement("span");
    dotText.style.cssText = "color:#000;font-size:11px;font-weight:700;";
    dotText.textContent = "?";
    statusDot.appendChild(dotText);

    bar.appendChild(label);
    bar.appendChild(spamBtn);
    bar.appendChild(safeBtn);
    bar.appendChild(statusDot);
    bar.appendChild(countEl);
    bar.appendChild(status);
    document.body.insertBefore(bar, document.body.firstChild);

    document.body.style.paddingTop = TOOLBAR_HEIGHT + "px";
  }

  // ── Add checkboxes ───────────────────────────────────────────────

  function addCheckboxes() {
    const results = document.querySelectorAll(".MjjYud");
    let added = 0;

    results.forEach((result) => {
      if (result.querySelector(".sl-cb-wrap")) return;
      const h3 = result.querySelector("h3");
      if (!h3) return;

      result.style.position = "relative";

      const wrap = document.createElement("div");
      wrap.className = "sl-cb-wrap";
      wrap.style.cssText = "position:absolute!important;right:8px!important;top:8px!important;z-index:999999!important;display:block!important;opacity:1!important;visibility:visible!important;pointer-events:auto!important;transform:none!important;-webkit-transform:none!important;";

      const cb = document.createElement("input");
      cb.type = "checkbox";
      cb.title = "Select to label as OPPOSITE";
      cb.style.cssText = "display:block!important;width:22px!important;height:22px!important;margin:0!important;padding:0!important;cursor:pointer!important;opacity:1!important;visibility:visible!important;position:relative!important;z-index:999999!important;accent-color:#3498db!important;border:2px solid #666!important;background:#fff!important;pointer-events:auto!important;box-shadow:0 1px 4px rgba(0,0,0,0.4)!important;transform:none!important;-webkit-transform:none!important;";

      cb.addEventListener("change", () => {
        if (cb.checked) {
          checkedItems.set(result, cb);
          result.classList.add("sl-selected");
        } else {
          checkedItems.delete(result);
          result.classList.remove("sl-selected");
        }
        updateCount();
      });

      wrap.appendChild(cb);
      result.appendChild(wrap);
      added++;
    });

    return added;
  }

  // ── Highlight spam results ───────────────────────────────────────

  async function highlightSpamDescriptions() {
    if (!modelLoaded) {
      console.log("[CS] Model not loaded yet, skipping highlight");
      return;
    }

    const results = document.querySelectorAll(".MjjYud");
    console.log("[CS] Found", results.length, "results to check");

    let highlighted = 0;
    let checked = 0;
    const summary = [];

    for (const result of results) {
      if (result.dataset.slHighlighted) continue;

      const descEls = result.querySelectorAll(".VwiC3b");
      for (const el of descEls) {
        const text = el.textContent.trim();
        if (text.length < 5) continue;

        checked++;
        const { isSpam, prob } = await predict(text);
        const label = isSpam ? "SPAM" : "SAFE";
        const icon = isSpam ? "🔴" : "🟢";

        summary.push({
          label: label,
          icon: icon,
          prob: (prob * 100).toFixed(0) + "%",
          text: text.substring(0, 80),
        });

        if (isSpam && prob > 0.5) {
          // Highlight the h3 title yellow
          const h3 = result.querySelector("h3");
          if (h3) {
            h3.style.color = "#d4a017";
            h3.style.fontWeight = "700";
          }

          highlighted++;
        }
      }

      result.dataset.slHighlighted = "1";
    }

    // Print full summary table
    console.log("[CS] === SCAN RESULTS ===");
    console.table(summary);
    console.log("[CS] Total:", checked, "| Spam:", highlighted, "| Safe:", checked - highlighted);
  }

  function updateCount() {
    const el = document.getElementById("sl-count");
    if (el) {
      const count = checkedItems.size;
      el.textContent = count > 0 ? "(" + count + " selected → opposite)" : "(0 selected)";
    }
  }

  // ── Extract descriptions ─────────────────────────────────────────

  function getDescriptions(resultEl) {
    const descriptions = [];
    resultEl.querySelectorAll(".VwiC3b").forEach((el) => {
      const text = el.textContent.trim();
      if (text.length > 5 && !descriptions.includes(text)) {
        descriptions.push(text);
      }
    });
    return descriptions;
  }

  // ── Download helper ──────────────────────────────────────────────

  function downloadFile(filename, content) {
    const blob = new Blob([content], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.style.display = "none";
    document.body.appendChild(a);
    a.click();
    setTimeout(() => { document.body.removeChild(a); URL.revokeObjectURL(url); }, 500);
  }

  // ── Download BOTH files ──────────────────────────────────────────

  function downloadBoth(primaryLabel) {
    const allResults = document.querySelectorAll(".MjjYud");
    if (allResults.length === 0) {
      const el = document.getElementById("sl-status");
      if (el) { el.textContent = "No results found"; el.style.color = "#e74c3c"; }
      return;
    }

    const primaryLines = [];
    const oppositeLines = [];

    allResults.forEach((result) => {
      const descs = getDescriptions(result);
      if (descs.length === 0) return;

      if (checkedItems.has(result)) {
        oppositeLines.push(...descs);
      } else {
        primaryLines.push(...descs);
      }
    });

    const timestamp = Math.floor(Date.now() / 1000);
    const oppositeLabel = primaryLabel === "spam" ? "safe" : "spam";

    if (primaryLines.length > 0) {
      downloadFile(primaryLabel + "." + timestamp + ".txt", primaryLines.join("\n"));
    }

    if (oppositeLines.length > 0) {
      setTimeout(() => {
        downloadFile(oppositeLabel + "." + timestamp + ".txt", oppositeLines.join("\n"));
      }, 300);
    }

    const el = document.getElementById("sl-status");
    if (el) {
      el.textContent = "✓ " + primaryLabel + ": " + primaryLines.length + " lines | " + oppositeLabel + ": " + oppositeLines.length + " lines";
      el.style.color = "#27ae60";
    }

    checkedItems.forEach((cb) => {
      cb.checked = false;
      const parent = cb.closest(".MjjYud");
      if (parent) parent.classList.remove("sl-selected");
    });
    checkedItems.clear();
    updateCount();
  }

  // ── Init ─────────────────────────────────────────────────────────

  async function waitForResults() {
    for (let i = 0; i < 30; i++) {
      const results = document.querySelectorAll(".MjjYud");
      if (results.length > 0) {
        console.log("[CS] Found", results.length, "results after", (i + 1) * 500, "ms");
        return true;
      }
      await new Promise(r => setTimeout(r, 500));
    }
    console.log("[CS] No results found after 15 seconds");
    return false;
  }

  function updateStatusDot(loaded) {
    const dot = document.getElementById("sl-status-dot");
    if (!dot) {
      console.log("[CS] Status dot not found yet");
      return;
    }
    if (loaded && modelVersion) {
      dot.style.background = "#4caf50";
      dot.title = "Model " + modelVersion;
      const txt = dot.querySelector("span");
      if (txt) {
        txt.style.color = "#000";
        txt.style.fontSize = "11px";
        txt.textContent = modelVersion;
      }
    } else {
      dot.style.background = "#e74c3c";
      dot.title = "Model not loaded";
      const txt = dot.querySelector("span");
      if (txt) {
        txt.style.color = "#fff";
        txt.style.fontSize = "11px";
        txt.textContent = "✗";
      }
    }
  }

  async function init() {
    console.log("[CS] Content script initialized");

    // Step 1: Create toolbar immediately
    createToolbar();
    console.log("[CS] Toolbar created");

    // Step 2: Wait for model to load in background
    let loaded = false;
    for (let i = 0; i < 10; i++) {
      console.log("[CS] Checking model status... attempt", i + 1);
      loaded = await checkModelStatus();
      if (loaded) break;
      await new Promise(r => setTimeout(r, 500));
    }

    if (loaded) {
      modelLoaded = true;
      console.log("[CS] Model is ready");
      updateStatusDot(true);
    } else {
      console.error("[CS] Model failed to load after retries");
      updateStatusDot(false);
    }

    // Step 3: Wait for Google results to load
    console.log("[CS] Waiting for search results...");
    const resultsReady = await waitForResults();

    if (!resultsReady) {
      console.log("[CS] No search results found, skipping scan");
      return;
    }

    // Step 4: Add checkboxes
    addCheckboxes();

    // Step 5: Scan and highlight spam
    console.log("[CS] Starting spam detection...");
    await highlightSpamDescriptions();
    console.log("[CS] Scan complete");
  }

  init();

  new MutationObserver(() => {
    addCheckboxes();
  }).observe(document.body, { childList: true, subtree: true });
})();
