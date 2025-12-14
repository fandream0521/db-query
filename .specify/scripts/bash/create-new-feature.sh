#!/bin/bash

# Script to create a new feature branch and specification
# Usage: create-new-feature.sh [--number N] [--short-name NAME] "Feature description"

set -e

NUMBER=""
SHORT_NAME=""
DESCRIPTION=""
JSON_OUTPUT=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --number)
            NUMBER="$2"
            shift 2
            ;;
        --short-name)
            SHORT_NAME="$2"
            shift 2
            ;;
        --json|-Json|-JsonOutput)
            JSON_OUTPUT=true
            shift
            ;;
        *)
            if [ -z "$DESCRIPTION" ]; then
                DESCRIPTION="$1"
            fi
            shift
            ;;
    esac
done

# Validate required parameters
if [ -z "$NUMBER" ] || [ -z "$SHORT_NAME" ] || [ -z "$DESCRIPTION" ]; then
    echo "Error: Missing required parameters" >&2
    echo "Usage: create-new-feature.sh --number N --short-name NAME \"Description\"" >&2
    exit 1
fi

# Create branch name
BRANCH_NAME="${NUMBER}-${SHORT_NAME}"
FEATURE_DIR="specs/${BRANCH_NAME}"
SPEC_FILE="${FEATURE_DIR}/spec.md"

# Create feature directory
mkdir -p "$FEATURE_DIR"
mkdir -p "${FEATURE_DIR}/checklists"

# Create branch (if git is available)
if command -v git &> /dev/null; then
    git checkout -b "$BRANCH_NAME" 2>/dev/null || git checkout "$BRANCH_NAME" 2>/dev/null || true
fi

# Create spec file placeholder
cat > "$SPEC_FILE" << EOF
# Feature Specification: [FEATURE_NAME]

**Feature ID:** ${BRANCH_NAME}  
**Status:** Draft  
**Created:** $(date +%Y-%m-%d)  
**Last Updated:** $(date +%Y-%m-%d)

## Overview

[To be filled]

## User Scenarios & Testing

[To be filled]

## Functional Requirements

[To be filled]

## Success Criteria

[To be filled]

## Key Entities

[To be filled]

## Assumptions

[To be filled]

## Dependencies

[To be filled]

## Out of Scope

[To be filled]

## Notes

[To be filled]
EOF

# Output JSON if requested
if [ "$JSON_OUTPUT" = true ]; then
    cat << EOF
{
  "branch_name": "${BRANCH_NAME}",
  "feature_dir": "${FEATURE_DIR}",
  "spec_file": "${SPEC_FILE}",
  "number": "${NUMBER}",
  "short_name": "${SHORT_NAME}",
  "description": "${DESCRIPTION}"
}
EOF
else
    echo "Created feature branch: ${BRANCH_NAME}"
    echo "Feature directory: ${FEATURE_DIR}"
    echo "Spec file: ${SPEC_FILE}"
fi
