#!/bin/bash

# Script to update agent-specific context files
# Usage: update-agent-context.sh [agent-name]

set -e

AGENT_NAME="${1:-cursor-agent}"
CONTEXT_FILE=".specify/context/${AGENT_NAME}.md"

# Create context directory if it doesn't exist
mkdir -p "$(dirname "$CONTEXT_FILE")"

# Create or update context file
if [ ! -f "$CONTEXT_FILE" ]; then
    cat > "$CONTEXT_FILE" << EOF
# Agent Context: ${AGENT_NAME}

**Last Updated:** $(date +%Y-%m-%d)

## Technology Stack

<!-- AUTO-GENERATED START: Technology Stack -->
<!-- Add technologies used in current implementation plan -->
<!-- AUTO-GENERATED END: Technology Stack -->

## Project-Specific Patterns

<!-- AUTO-GENERATED START: Patterns -->
<!-- Add project-specific patterns and conventions -->
<!-- AUTO-GENERATED END: Patterns -->

## Manual Additions

<!-- Manual additions below this line will be preserved -->

EOF
fi

echo "Context file: ${CONTEXT_FILE}"
echo "Update this file with technologies from the implementation plan"
