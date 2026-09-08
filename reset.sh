#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

get_folder() {
  case "$1" in
    01)       echo "01-variables-and-types" ;;
    02)       echo "02-functions-and-control-flow" ;;
    02-bonus) echo "02-bonus-loops-practice" ;;
    03)       echo "03-ownership-and-borrowing" ;;
    04)       echo "04-structs-and-methods" ;;
    05)       echo "05-enums-and-pattern-matching" ;;
    06)       echo "06-option-result-and-error-handling" ;;
    07)       echo "07-collections" ;;
    *)        echo "" ;;
  esac
}

ALL_KEYS="01 02 02-bonus 03 04 05 06 07"

reset_lesson() {
  local key="$1"
  local folder
  folder="$(get_folder "$key")"

  if [[ -z "$folder" ]]; then
    echo "  ❌ Unknown lesson: $key"
    return
  fi

  local template="$SCRIPT_DIR/.templates/$key/main.rs"

  # 02-bonus has no exercise/ subdirectory
  if [[ "$key" == "02-bonus" ]]; then
    local dest="$SCRIPT_DIR/$folder/src/main.rs"
  else
    local dest="$SCRIPT_DIR/$folder/exercise/src/main.rs"
  fi

  if [[ ! -f "$template" ]]; then
    echo "  ⚠  Template not found: .templates/$key/main.rs — skipping"
    return
  fi

  if [[ ! -d "$(dirname "$dest")" ]]; then
    echo "  ⚠  Destination directory missing for $folder — skipping"
    return
  fi

  cp "$template" "$dest"
  echo "  ✅ Reset $folder"
}

# Determine which lessons to reset
if [[ $# -eq 0 ]]; then
  echo "🔄 Resetting ALL exercises..."
  targets="$ALL_KEYS"
else
  echo "🔄 Resetting selected exercises..."
  targets="$*"
fi

for key in $targets; do
  reset_lesson "$key"
done

echo "Done!"
