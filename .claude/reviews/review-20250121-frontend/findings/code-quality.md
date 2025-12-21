# Code Quality Findings

## DRY Violations

### API Client Duplication (Critical)
See Architecture findings - duplicated axios configuration in 4 files.

### Schema Loading Logic Duplication (Medium)
- App.tsx and SchemaView.tsx both implement schema loading
- Consider consolidating or choosing one approach

## YAGNI Violations

### Unused Components
- **SchemaView.tsx** - Fully implemented but never imported/used in App
- **DatabaseList.tsx** - Component exists but App implements its own database list
- **QueryPanel.tsx** - Not used, App handles query panel inline

**Impact**: 300+ lines of unused code in the codebase

### Unused Imports
- ViewList.tsx: `const { Panel } = Collapse;`
- QueryPanel.tsx: `import { message } from 'antd';`

## Function Length Violations

| File | Function | Lines | Over Limit |
|------|----------|-------|------------|
| App.tsx | `App` component | 729 | +579 lines |
| App.tsx | `buildTreeData` | 110 | No (inline, but should be extracted) |

**Threshold**: 150 lines per function

## Parameter Count
All functions are within acceptable limits (< 7 parameters).

## Code Complexity

### App.tsx Complexity Metrics:
- **Cyclomatic Complexity**: Very High (~40+)
- **State Variables**: 12 (recommendation: max 5 per component)
- **useEffect Hooks**: 3 (some with dependency issues)
- **Event Handlers**: 15+ inline handlers
- **Nesting Depth**: 6-7 levels deep in JSX

### Recommendations:
1. Extract components (reduces complexity to ~5 per component)
2. Use custom hooks for state logic
3. Move styles to CSS modules
4. Extract event handlers to separate functions

## Missing Patterns

### No Custom Hooks
Despite complex state logic, no custom hooks are extracted:
- useDatabaseSelection
- useSchemaLoading
- useQueryExecution

These would significantly improve code organization.

### No Component Composition
Large monolithic components instead of composing smaller ones.

### No Memoization
QueryResults could benefit from useMemo for column/row transformations.

