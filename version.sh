#!/bin/bash
# Bump version and create git tag for spam-labeler
# Usage:
#   ./version.sh          → show current version
#   ./version.sh patch    → bump patch (0.1.0 → 0.1.1)
#   ./version.sh minor    → bump minor (0.1.0 → 0.2.0)
#   ./version.sh major    → bump major (0.1.0 → 1.0.0)
#   ./version.sh 0.2.0    → set explicit version

set -e
cd "$(dirname "$0")"

CARGO="Cargo.toml"
CURRENT=$(grep '^version = ' "$CARGO" | head -1 | sed 's/version = "//;s/"//')

if [ $# -eq 0 ]; then
  echo "Current version: $CURRENT"
  echo ""
  echo "Usage:"
  echo "  ./version.sh          → show current version"
  echo "  ./version.sh patch    → bump patch (0.1.0 → 0.1.1)"
  echo "  ./version.sh minor    → bump minor (0.1.0 → 0.2.0)"
  echo "  ./version.sh major    → bump major (0.1.0 → 1.0.0)"
  echo "  ./version.sh 0.2.0    → set explicit version"
  exit 0
fi

NEW=""
case "$1" in
  major)
    IFS='.' read -r a b c <<< "$CURRENT"
    NEW="$((a + 1)).0.0"
    ;;
  minor)
    IFS='.' read -r a b c <<< "$CURRENT"
    NEW="$a.$((b + 1)).0"
    ;;
  patch)
    IFS='.' read -r a b c <<< "$CURRENT"
    NEW="$a.$b.$((c + 1))"
    ;;
  *)
    NEW="$1"
    ;;
esac

# Update Cargo.toml
sed -i "s/^version = \"$CURRENT\"/version = \"$NEW\"/" "$CARGO"

# Create git tag
git add "$CARGO"
git commit -m "chore: bump version to v$NEW"
git tag -a "v$NEW" -m "Release v$NEW"

echo ""
echo "Version: $CURRENT → $NEW"
echo "Tag: v$NEW"
echo ""
echo "Push with: git push origin main --tags"
