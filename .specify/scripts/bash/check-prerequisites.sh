#!/bin/bash

# Script to check prerequisites and list available documents
# Usage: check-prerequisites.sh [--json]

set -e

JSON_OUTPUT=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --json|-Json|-JsonOutput)
            JSON_OUTPUT=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Find the feature spec directory
SPECS_DIR="specs"
FEATURE_DIR=$(find "$SPECS_DIR" -maxdepth 1 -type d -name "*-*" | head -n 1)

if [ -z "$FEATURE_DIR" ]; then
    echo "Error: No feature directory found in $SPECS_DIR" >&2
    exit 1
fi

# Convert to absolute path
FEATURE_DIR=$(cd "$FEATURE_DIR" && pwd)
SPECS_DIR=$(cd "$SPECS_DIR" && pwd)

# Check available documents
AVAILABLE_DOCS=()

if [ -f "${FEATURE_DIR}/spec.md" ]; then
    AVAILABLE_DOCS+=("spec.md")
fi

if [ -f "${FEATURE_DIR}/plan.md" ]; then
    AVAILABLE_DOCS+=("plan.md")
fi

if [ -f "${FEATURE_DIR}/data-model.md" ]; then
    AVAILABLE_DOCS+=("data-model.md")
fi

if [ -f "${FEATURE_DIR}/research.md" ]; then
    AVAILABLE_DOCS+=("research.md")
fi

if [ -f "${FEATURE_DIR}/quickstart.md" ]; then
    AVAILABLE_DOCS+=("quickstart.md")
fi

if [ -d "${FEATURE_DIR}/contracts" ]; then
    AVAILABLE_DOCS+=("contracts/")
fi

# Output JSON if requested
if [ "$JSON_OUTPUT" = true ]; then
    # Convert array to JSON array
    DOCS_JSON=$(printf '%s\n' "${AVAILABLE_DOCS[@]}" | jq -R . | jq -s .)
    cat << EOF
{
  "feature_dir": "${FEATURE_DIR}",
  "available_docs": ${DOCS_JSON}
}
EOF
else
    echo "Feature Directory: ${FEATURE_DIR}"
    echo "Available Documents:"
    for doc in "${AVAILABLE_DOCS[@]}"; do
        echo "  - ${doc}"
    done
fi
