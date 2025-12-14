#!/bin/bash

# Script to setup plan directory structure
# Usage: setup-plan.sh [--json]

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

FEATURE_SPEC="${FEATURE_DIR}/spec.md"
IMPL_PLAN="${FEATURE_DIR}/plan.md"
BRANCH=$(basename "$FEATURE_DIR")

# Create plan.md if it doesn't exist
if [ ! -f "$IMPL_PLAN" ]; then
    # Copy template if it exists
    if [ -f ".specify/templates/plan-template.md" ]; then
        cp ".specify/templates/plan-template.md" "$IMPL_PLAN"
    else
        # Create basic template
        cat > "$IMPL_PLAN" << 'EOF'
# Implementation Plan: [FEATURE_NAME]

**Feature ID:** [FEATURE_ID]  
**Status:** Draft  
**Created:** [DATE]

## Technical Context

[To be filled]

## Constitution Check

[To be filled]

## Gates

[To be filled]

## Phase 0: Research

[To be filled]

## Phase 1: Design

[To be filled]

## Phase 2: Implementation

[To be filled]
EOF
    fi
fi

# Output JSON if requested
if [ "$JSON_OUTPUT" = true ]; then
    cat << EOF
{
  "feature_spec": "${FEATURE_SPEC}",
  "impl_plan": "${IMPL_PLAN}",
  "specs_dir": "${SPECS_DIR}",
  "branch": "${BRANCH}",
  "feature_dir": "${FEATURE_DIR}"
}
EOF
else
    echo "Feature Spec: ${FEATURE_SPEC}"
    echo "Implementation Plan: ${IMPL_PLAN}"
    echo "Specs Directory: ${SPECS_DIR}"
    echo "Branch: ${BRANCH}"
fi
