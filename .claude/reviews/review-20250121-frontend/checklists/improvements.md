# Code Improvement Checklist: review-20250121-frontend

**Purpose**: Track progress on addressing code review findings
**Created**: 2025-01-21
**Review Report**: [report.md](../report.md)

---

## Critical Issues (P0) - Fix Immediately

### 1. Refactor App.tsx into Smaller Components

- [ ] **Issue**: App.tsx is 729 lines (579 over limit)
- [ ] **File**: `frontend/src/App.tsx:20-726`
- [ ] **Actions**:
  - [ ] Create `components/DatabaseSidebar.tsx` (extract lines 232-480)
  - [ ] Create `components/SchemaTreeSidebar.tsx` (extract lines 482-557)
  - [ ] Create `components/QueryWorkspace.tsx` (extract lines 560-712)
  - [ ] Create `hooks/useDatabaseManagement.ts` (database state logic)
  - [ ] Create `hooks/useSchemaLoading.ts` (schema state logic)
  - [ ] Create `hooks/useSchemaTree.ts` (extract buildTreeData)
  - [ ] Update App.tsx to use new components (target: <100 lines)
  - [ ] Test all functionality still works
- [ ] **Estimated Effort**: High (4-6 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 2. Create Shared API Client Instance

- [ ] **Issue**: API client configuration duplicated in 4 files
- [ ] **Files**:
  - `frontend/src/api/database.ts:4-11`
  - `frontend/src/api/schema.ts:4-11`
  - `frontend/src/api/query.ts:4-11`
  - `frontend/src/api/natural_language.ts:5-12`
- [ ] **Actions**:
  - [ ] Create `api/client.ts` with shared axios instance
  - [ ] Add request/response interceptors
  - [ ] Update `api/database.ts` to import shared client
  - [ ] Update `api/schema.ts` to import shared client
  - [ ] Update `api/query.ts` to import shared client
  - [ ] Update `api/natural_language.ts` to import shared client
  - [ ] Test all API calls still work
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## High Priority Issues (P1) - Address in Next Sprint

### 3. Add Error Boundary Component

- [ ] **Issue**: No error boundary - React errors cause white screen
- [ ] **File**: `frontend/src/index.tsx:6-13`
- [ ] **Actions**:
  - [ ] Create `components/ErrorBoundary.tsx`
  - [ ] Implement componentDidCatch lifecycle
  - [ ] Add fallback UI with reload button
  - [ ] Wrap App with ErrorBoundary in index.tsx
  - [ ] Test error boundary with thrown error
- [ ] **Estimated Effort**: Low (1 hour)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 4. Fix useEffect Dependency Issues

- [ ] **Issue**: loadDatabases has circular dependency on selectedDb
- [ ] **File**: `frontend/src/App.tsx:34-53`
- [ ] **Actions**:
  - [ ] Remove selectedDb from loadDatabases dependencies
  - [ ] Move auto-selection logic to useEffect callback
  - [ ] Verify no unnecessary re-renders
  - [ ] Test database loading and auto-selection
- [ ] **Estimated Effort**: Low (15 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 5. Move Inline Styles to CSS Modules

- [ ] **Issue**: 400+ inline style objects in App.tsx
- [ ] **File**: `frontend/src/App.tsx` (entire file)
- [ ] **Actions**:
  - [ ] Create `App.module.css`
  - [ ] Extract sidebar header styles
  - [ ] Extract database item styles
  - [ ] Extract schema tree styles
  - [ ] Extract query workspace styles
  - [ ] Replace inline styles with className
  - [ ] Test visual appearance matches original
- [ ] **Estimated Effort**: Medium (3-4 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 6. Add State Management (Zustand or Context)

- [ ] **Issue**: 12+ state variables in root component
- [ ] **File**: `frontend/src/App.tsx:20-32`
- [ ] **Actions**:
  - [ ] Install Zustand (or choose Context API)
  - [ ] Create `stores/useDatabaseStore.ts`
  - [ ] Create `stores/useSchemaStore.ts`
  - [ ] Create `stores/useQueryStore.ts`
  - [ ] Migrate state from App.tsx to stores
  - [ ] Update components to use stores
  - [ ] Test all state interactions work
- [ ] **Estimated Effort**: Medium (2-3 hours)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## Medium Priority Issues (P2) - Plan for Future Sprint

### 7. Replace `any` Types with Proper Types

- [ ] **Issue**: 5 instances of `any` type defeating TypeScript
- [ ] **Files**: QueryResults.tsx, SQLEditor.tsx, QueryPanel.tsx
- [ ] **Actions**:
  - [ ] Define CellValue type for query results
  - [ ] Add Monaco editor types to SQLEditor
  - [ ] Replace AxiosError `any` casts with proper types
  - [ ] Verify type checking passes
- [ ] **Estimated Effort**: Low (1 hour)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 8. Remove Unused Components

- [ ] **Issue**: 300+ lines of unused code
- [ ] **Files**: SchemaView.tsx, DatabaseList.tsx, QueryPanel.tsx
- [ ] **Actions**:
  - [ ] Verify components are truly unused
  - [ ] Delete SchemaView.tsx (or integrate into App)
  - [ ] Delete DatabaseList.tsx (or integrate into App)
  - [ ] Delete QueryPanel.tsx (or integrate into App)
  - [ ] Update imports if needed
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 9. Fix Schema View Duplication

- [ ] **Issue**: Duplicate schema loading logic
- [ ] **Files**: App.tsx, SchemaView.tsx
- [ ] **Actions**:
  - [ ] Choose: use SchemaView OR keep App's implementation
  - [ ] If using SchemaView: refactor App to use it
  - [ ] If keeping App: delete SchemaView.tsx
  - [ ] Remove duplicate logic
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 10. Clear Schema on Database Change

- [ ] **Issue**: Stale schema shown during loading
- [ ] **File**: `frontend/src/App.tsx:56-62`
- [ ] **Actions**:
  - [ ] Add `setSchema(null)` at start of useEffect
  - [ ] Add `setExpandedKeys([])` to clear tree state
  - [ ] Test that schema clears immediately on DB change
- [ ] **Estimated Effort**: Low (2 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## Low Priority Issues (P3) - Nice to Have

### 11. Add Accessibility Attributes

- [ ] **Issue**: Missing ARIA labels for screen readers
- [ ] **File**: `frontend/src/components/QueryResults.tsx:64-88`
- [ ] **Actions**:
  - [ ] Add aria-label to Table component
  - [ ] Add caption to table
  - [ ] Add aria-label to pagination
  - [ ] Test with screen reader
- [ ] **Estimated Effort**: Low (30 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

### 12. Remove Unused Imports

- [ ] **Issue**: Unused imports increase bundle size
- [ ] **Files**: ViewList.tsx, QueryPanel.tsx
- [ ] **Actions**:
  - [ ] Remove `const { Panel } = Collapse;` from ViewList.tsx
  - [ ] Remove `import { message } from 'antd';` from QueryPanel.tsx
  - [ ] Run linter to find other unused imports
- [ ] **Estimated Effort**: Low (5 minutes)
- [ ] **Assigned To**: _________
- [ ] **Completed**: _________

---

## Progress Tracking

| Priority | Total | Completed | In Progress | Not Started |
|----------|-------|-----------|-------------|-------------|
| P0       | 2     | 0         | 0           | 2           |
| P1       | 4     | 0         | 0           | 4           |
| P2       | 4     | 0         | 0           | 4           |
| P3       | 2     | 0         | 0           | 2           |
| **Total** | **12** | **0** | **0** | **12** |

---

## Estimated Timeline

**Sprint 1** (Focus on P0):
- Week 1: Refactor App.tsx + Create shared API client (6.5 hours)
- Expected health score improvement: 58 → 72 (+14 points)

**Sprint 2** (Focus on P1):
- Week 2-3: Error boundary, fix hooks, inline styles, state management (9 hours)
- Expected health score improvement: 72 → 82 (+10 points)

**Sprint 3** (P2 cleanup):
- Week 4: Type safety, remove unused code (2 hours)
- Expected health score improvement: 82 → 88 (+6 points)

**Target Health Score**: 88/100 (Excellent)

---

## Notes

- Mark items as completed when fix is implemented and tested
- Use [validation checklist](validation.md) to verify fixes
- Update progress tracking table weekly
- Re-run code review after P0+P1 fixes to measure improvement
- Consider pair programming for App.tsx refactor (complexity)
