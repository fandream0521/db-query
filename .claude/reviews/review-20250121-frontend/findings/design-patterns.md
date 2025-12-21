# Design Patterns and KISS Principle Findings

## Over-Engineering

### Complex Tree Building Logic
**File**: `App.tsx:118-228`

The `buildTreeData` function is 110 lines of complex nested object construction with inline JSX. This violates KISS (Keep It Simple, Stupid).

**Issue**:
```typescript
const buildTreeData = () => {
  if (!schema) return [];

  return [
    {
      title: (
        <span style={{ fontWeight: 600, fontSize: '13px' }}>
          Tables ({schema.tables.length})
        </span>
      ),
      key: 'tables-root',
      children: schema.tables.map((table) => ({
        title: (
          <div style={{ /* 15 lines of inline styles */ }}>
            {/* Complex JSX */}
          </div>
        ),
        // ... 60 more lines of nested mapping
      })),
    },
  ];
};
```

**Simplification**:
```typescript
// Extract to separate component
const SchemaTreeBuilder = ({ schema }) => {
  if (!schema) return null;

  return (
    <Tree>
      <TreeNode title={`Tables (${schema.tables.length})`}>
        {schema.tables.map(table => (
          <TableNode key={table.name} table={table} />
        ))}
      </TreeNode>
    </Tree>
  );
};

const TableNode = ({ table }) => (
  <TreeNode title={<TableTitle table={table} />}>
    {table.columns.map(col => (
      <ColumnNode key={col.name} column={col} table={table} />
    ))}
  </TreeNode>
);
```

## Unnecessary Complexity

### Inline Event Handlers with Multiple Conditions
**File**: `App.tsx:369-380`

```typescript
onMouseEnter={(e) => {
  if (!isSelected) {
    e.currentTarget.style.background = '#fafafa';
    e.currentTarget.style.borderColor = '#d9d9d9';
  }
}}
onMouseLeave={(e) => {
  if (!isSelected) {
    e.currentTarget.style.background = 'transparent';
    e.currentTarget.style.borderColor = '#f0f0f0';
  }
}}
```

**Simpler**:
Use CSS hover pseudo-class instead of JavaScript:

```css
.database-item:hover:not(.selected) {
  background: #fafafa;
  border-color: #d9d9d9;
}
```

### Excessive State for UI Concerns
**Issue**: Separate loading states for different operations:
- `loading`
- `schemaLoading`
- `queryLoading`

**Simpler**: Use loading context or single loading state with type:
```typescript
type LoadingState =
  | { type: 'idle' }
  | { type: 'loading-databases' }
  | { type: 'loading-schema' }
  | { type: 'executing-query' };

const [loadingState, setLoadingState] = useState<LoadingState>({ type: 'idle' });
```

## Premature Optimization

### Collapse Animation Logic
**File**: `App.tsx:233-324`

Extensive collapse/expand animation logic with computed styles, transitions, and multiple state variables just for sidebar collapse. This could use a simple CSS class toggle.

## Missing Abstractions

### Error Handling Duplication
Every API call has try-catch with showError:

```typescript
try {
  const data = await listDatabases();
  setDatabases(data);
} catch (error: unknown) {
  showError(error, 'Failed to load databases');
}
```

**Better**: Create useQuery hook or wrapper:
```typescript
const { data, loading, error } = useQuery(() => listDatabases());
```

## Cognitive Load Issues

### 400+ Inline Styles
Makes JSX extremely hard to parse visually. Example:

```typescript
<div style={{
  display: 'flex',
  alignItems: 'center',
  gap: '12px',
  marginBottom: '16px',
  cursor: 'pointer',
  userSelect: 'none',
  justifyContent: firstColumnCollapsed ? 'center' : 'flex-start',
  transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)'
}}>
```

**Reduces to**:
```typescript
<div className="header-title">
```

## Recommendations

1. **Extract Components**: Break App.tsx into 8-10 focused components
2. **Use CSS**: Move 90% of inline styles to CSS/CSS modules
3. **Simplify State**: Consolidate related state variables
4. **Create Abstractions**: Custom hooks for common patterns
5. **Remove Premature Optimizations**: Use standard patterns first

