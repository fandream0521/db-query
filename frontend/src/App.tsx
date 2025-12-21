import React, { useState, useEffect } from 'react';
import { Layout } from 'antd';
import AddDatabaseForm from './components/AddDatabaseForm';
import DatabaseSidebar from './components/DatabaseSidebar';
import SchemaSidebar from './components/SchemaSidebar';
import QueryWorkspace from './components/QueryWorkspace';
import { DatabaseConnection } from './types/database';
import { useDatabaseManagement } from './hooks/useDatabaseManagement';
import { useSchemaLoading } from './hooks/useSchemaLoading';
import { useQueryExecution } from './hooks/useQueryExecution';


function App() {
  // Custom hooks for state management
  const { databases, selectedDb, loading, loadDatabases, selectDatabase } = useDatabaseManagement();
  const { schema, loading: schemaLoading, expandedKeys, loadSchema, setExpandedKeys } = useSchemaLoading();
  const {
    sql,
    setSql,
    queryResult,
    queryLoading,
    executionTime,
    executeQuery: executeQueryHandler,
    resetQuery,
  } = useQueryExecution(selectedDb?.name || null);

  // Local UI state
  const [showAddForm, setShowAddForm] = useState(false);
  const [firstColumnCollapsed, setFirstColumnCollapsed] = useState(false);

  // Load databases on mount
  useEffect(() => {
    loadDatabases();
  }, [loadDatabases]);

  // Load schema when database is selected
  useEffect(() => {
    if (selectedDb) {
      loadSchema(selectedDb.name);
    }
  }, [selectedDb, loadSchema]);

  const handleDatabaseSelect = (db: DatabaseConnection) => {
    selectDatabase(db);
    resetQuery();
  };

  const handleRefresh = () => {
    if (selectedDb) {
      loadSchema(selectedDb.name);
    }
    loadDatabases();
  };

  return (
    <Layout style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      {/* First Column - Database List */}
      <DatabaseSidebar
        databases={databases}
        selectedDb={selectedDb}
        loading={loading}
        collapsed={firstColumnCollapsed}
        onAddDatabase={() => setShowAddForm(true)}
        onSelectDatabase={handleDatabaseSelect}
        onToggleCollapse={() => setFirstColumnCollapsed(!firstColumnCollapsed)}
      />

      {/* Second Column - Tables Tree */}
      <SchemaSidebar
        selectedDb={selectedDb}
        schema={schema}
        loading={schemaLoading}
        expandedKeys={expandedKeys}
        firstColumnCollapsed={firstColumnCollapsed}
        onRefresh={handleRefresh}
        onExpandedKeysChange={setExpandedKeys}
      />

      {/* Third Column - Statistics and Query Panel */}
      <Layout style={{ marginLeft: firstColumnCollapsed ? 320 : 480, background: '#f5f5f5', transition: 'margin-left 0.3s ease' }}>
        <QueryWorkspace
          selectedDb={selectedDb}
          schema={schema}
          sql={sql}
          queryResult={queryResult}
          queryLoading={queryLoading}
          executionTime={executionTime}
          onSqlChange={setSql}
          onExecuteQuery={executeQueryHandler}
        />
      </Layout>

      {/* Add Database Modal */}
      {showAddForm && (
        <AddDatabaseForm
          onSuccess={() => {
            setShowAddForm(false);
            loadDatabases();
          }}
          onCancel={() => setShowAddForm(false)}
        />
      )}
    </Layout>
  );
}

export default App;
