import React, { useState, useEffect } from 'react';
import { Layout, Typography, Button, Space, Input, Card, Row, Col, Spin, Badge, Tree, Tooltip } from 'antd';
import { PlusOutlined, ReloadOutlined, SearchOutlined, PlayCircleOutlined, DatabaseOutlined } from '@ant-design/icons';
import AddDatabaseForm from './components/AddDatabaseForm';
import SQLEditor from './components/SQLEditor';
import QueryResults from './components/QueryResults';
import { DatabaseConnection } from './types/database';
import { SchemaMetadata } from './types/schema';
import { listDatabases } from './api/database';
import { getSchemaMetadata } from './api/schema';
import { executeQuery } from './api/query';
import { QueryResponse } from './types/query';
import { showError, showSuccess } from './utils/error';
import 'antd/dist/reset.css';
import './App.css';

const { Sider, Content } = Layout;
const { Title, Text } = Typography;
const { TreeNode } = Tree;

function App() {
  const [selectedDb, setSelectedDb] = useState<DatabaseConnection | null>(null);
  const [databases, setDatabases] = useState<DatabaseConnection[]>([]);
  const [schema, setSchema] = useState<SchemaMetadata | null>(null);
  const [loading, setLoading] = useState(false);
  const [schemaLoading, setSchemaLoading] = useState(false);
  const [showAddForm, setShowAddForm] = useState(false);
  const [searchText, setSearchText] = useState('');
  const [sql, setSql] = useState<string>('SELECT * FROM');
  const [queryResult, setQueryResult] = useState<QueryResponse | null>(null);
  const [queryLoading, setQueryLoading] = useState(false);
  const [executionTime, setExecutionTime] = useState<string>('-');
  const [expandedKeys, setExpandedKeys] = useState<React.Key[]>([]);

  // Load databases
  useEffect(() => {
    loadDatabases();
  }, []);

  // Load schema when database is selected
  useEffect(() => {
    if (selectedDb) {
      loadSchema(selectedDb.name);
    } else {
      setSchema(null);
    }
  }, [selectedDb]);

  const loadDatabases = async () => {
    setLoading(true);
    try {
      const data = await listDatabases();
      setDatabases(data);
      // Auto-select first database if none selected
      if (!selectedDb && data.length > 0) {
        setSelectedDb(data[0]);
      }
    } catch (error: unknown) {
      showError(error, 'Failed to load databases');
    } finally {
      setLoading(false);
    }
  };

  const loadSchema = async (dbName: string) => {
    setSchemaLoading(true);
    try {
      const data = await getSchemaMetadata(dbName);
      setSchema(data);
      // Auto-expand all tables
      if (data.tables.length > 0) {
        setExpandedKeys(data.tables.map((table) => `table-${table.name}`));
      }
    } catch (error: unknown) {
      showError(error, 'Failed to load schema');
      setSchema(null);
    } finally {
      setSchemaLoading(false);
    }
  };

  const handleDatabaseSelect = (db: DatabaseConnection) => {
    setSelectedDb(db);
    setSql('SELECT * FROM');
    setQueryResult(null);
    setExecutionTime('-');
  };

  const handleRefresh = () => {
    if (selectedDb) {
      loadSchema(selectedDb.name);
    }
    loadDatabases();
  };

  const handleExecute = async () => {
    if (!sql.trim() || !selectedDb) {
      return;
    }

    setQueryLoading(true);
    setQueryResult(null);
    const startTime = Date.now();

    try {
      const response = await executeQuery(selectedDb.name, { sql });
      const endTime = Date.now();
      setExecutionTime(`${((endTime - startTime) / 1000).toFixed(2)}s`);
      setQueryResult(response);
      showSuccess('Query executed successfully');
    } catch (err: unknown) {
      showError(err, 'Failed to execute query');
    } finally {
      setQueryLoading(false);
    }
  };

  // Build tree data for tables
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
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px', width: '100%', paddingRight: '8px' }}>
              <Text 
                strong 
                ellipsis
                style={{ 
                  fontSize: '12px',
                  flex: 1,
                  minWidth: 0
                }}
              >
                {table.name}
              </Text>
              <span style={{ fontSize: '10px', color: '#8c8c8c', flexShrink: 0 }}>
                ({table.rowCount || 0} rows)
              </span>
            </div>
          ),
          key: `table-${table.name}`,
          children: table.columns.map((col) => {
            // Build constraints as children
            const constraints = [];
            if (table.primaryKey?.includes(col.name)) {
              constraints.push({
                title: (
                  <span style={{ fontSize: '11px', color: '#1890ff' }}>
                    PK (Primary Key)
                  </span>
                ),
                key: `constraint-pk-${table.name}-${col.name}`,
                isLeaf: true,
              });
            }
            if (!col.nullable) {
              constraints.push({
                title: (
                  <span style={{ fontSize: '11px', color: '#ff4d4f' }}>
                    NOT NULL
                  </span>
                ),
                key: `constraint-null-${table.name}-${col.name}`,
                isLeaf: true,
              });
            }
            constraints.push({
              title: (
                <span style={{ fontSize: '11px', color: '#8c8c8c' }}>
                  Type: {col.dataType.toUpperCase()}
                </span>
              ),
              key: `constraint-type-${table.name}-${col.name}`,
              isLeaf: true,
            });
            if (col.defaultValue) {
              constraints.push({
                title: (
                  <span style={{ fontSize: '11px', color: '#8c8c8c' }}>
                    Default: {col.defaultValue}
                  </span>
                ),
                key: `constraint-default-${table.name}-${col.name}`,
                isLeaf: true,
              });
            }

            return {
              title: (
                <div style={{ 
                  display: 'flex', 
                  alignItems: 'center', 
                  width: '100%',
                  paddingRight: '8px',
                  boxSizing: 'border-box'
                }}>
                  <Text 
                    code 
                    ellipsis
                    style={{ 
                      fontSize: '12px',
                      background: 'transparent',
                      padding: 0,
                      border: 'none',
                      flex: 1,
                      minWidth: 0
                    }}
                  >
                    {col.name}
                  </Text>
                </div>
              ),
              key: `field-${table.name}-${col.name}`,
              children: constraints.length > 0 ? constraints : undefined,
              isLeaf: constraints.length === 0,
            };
          }),
        })),
      },
    ];
  };

  const totalRows = schema?.tables.reduce((sum, table) => sum + (table.rowCount || 0), 0) || 0;

  return (
    <Layout style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      {/* First Column - Database List */}
      <Sider
        width={224}
        style={{
          background: '#fff',
          borderRight: '1px solid #e8e8e8',
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
        }}
      >
        {/* Header */}
        <div style={{ padding: '20px', borderBottom: '1px solid #f0f0f0' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px' }}>
            <DatabaseOutlined style={{ fontSize: '24px', color: '#1890ff' }} />
            <Title level={4} style={{ margin: 0, fontSize: '18px', fontWeight: 600 }}>
              DB QUERY TOOL
            </Title>
          </div>
          <Button
            type="primary"
            icon={<PlusOutlined />}
            block
            onClick={() => setShowAddForm(true)}
            style={{ 
              height: '40px',
              fontSize: '14px',
              borderRadius: '6px'
            }}
          >
            ADD DATABASE
          </Button>
        </div>

        {/* Database List Body */}
        <div style={{ padding: '16px' }}>
          {loading ? (
            <Spin size="large" style={{ display: 'block', textAlign: 'center', padding: '40px' }} />
          ) : databases.length === 0 ? (
            <div style={{ textAlign: 'center', padding: '40px', color: '#999' }}>
              <Text type="secondary">No databases added</Text>
            </div>
          ) : (
            <div>
              {databases.map((db) => {
                const isSelected = selectedDb?.name === db.name;
                return (
                  <div
                    key={db.name}
                    onClick={() => handleDatabaseSelect(db)}
                    className="database-item"
                    style={{
                      padding: '14px 16px',
                      marginBottom: '8px',
                      borderRadius: '8px',
                      cursor: 'pointer',
                      background: isSelected ? '#f0f7ff' : 'transparent',
                      border: isSelected ? '1px solid #1890ff' : '1px solid #f0f0f0',
                      transition: 'all 0.2s',
                    }}
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
                  >
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: '8px' }}>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flex: 1, minWidth: 0 }}>
                        {isSelected && (
                          <div
                            style={{
                              width: '6px',
                              height: '6px',
                              borderRadius: '50%',
                              background: '#52c41a',
                              flexShrink: 0,
                            }}
                          />
                        )}
                        <div style={{ flex: 1, minWidth: 0, overflow: 'hidden' }}>
                          <Text 
                            strong={isSelected} 
                            ellipsis
                            style={{ 
                              fontSize: '13px',
                              color: isSelected ? '#1890ff' : '#262626',
                              display: 'block',
                              marginBottom: '4px',
                              width: '100%'
                            }}
                          >
                            {db.name}
                          </Text>
                          <Tooltip title={db.url} placement="right">
                            <Text 
                              type="secondary" 
                              ellipsis
                              style={{ 
                                fontSize: '11px', 
                                display: 'block',
                                width: '100%',
                                cursor: 'help'
                              }}
                            >
                              {db.url.replace(/\/\/.*@/, '//***@')}
                            </Text>
                          </Tooltip>
                        </div>
                      </div>
                      <Badge count={0} showZero style={{ backgroundColor: '#d9d9d9', flexShrink: 0 }} />
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </Sider>

      {/* Second Column - Tables Tree */}
      <Sider
        width={256}
        style={{
          background: '#fff',
          borderRight: '1px solid #e8e8e8',
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 224,
          top: 0,
          zIndex: 1,
        }}
      >
        {selectedDb ? (
          <>
            {/* Header - Yellow background */}
            <div
              style={{
                padding: '16px 20px',
                background: '#fff3cd',
                borderBottom: '1px solid #ffc107',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
              }}
            >
              <Text strong style={{ color: '#856404', fontSize: '14px' }}>
                {selectedDb.name.toUpperCase()}
              </Text>
              <Button
                icon={<ReloadOutlined />}
                onClick={handleRefresh}
                size="small"
                style={{ 
                  borderRadius: '4px',
                }}
              >
                REFRESH
              </Button>
            </div>

            {/* Body - Tables Tree */}
            <div style={{ padding: '12px 16px', overflowX: 'hidden', width: '100%', boxSizing: 'border-box' }}>
              {schemaLoading ? (
                <Spin size="large" style={{ display: 'block', textAlign: 'center', padding: '40px' }} />
              ) : schema ? (
                <div style={{ width: '100%', overflow: 'hidden' }}>
                  <Tree
                    treeData={buildTreeData()}
                    defaultExpandAll={false}
                    expandedKeys={expandedKeys}
                    onExpand={(keys) => setExpandedKeys(keys)}
                    showLine={{ showLeafIcon: false }}
                    style={{ 
                      background: 'transparent',
                      fontSize: '13px',
                      width: '100%'
                    }}
                    blockNode
                  />
                </div>
              ) : (
                <div style={{ textAlign: 'center', padding: '40px', color: '#999' }}>
                  <Text type="secondary">No schema available</Text>
                </div>
              )}
            </div>
          </>
        ) : (
          <div style={{ padding: '40px', textAlign: 'center', color: '#999' }}>
            <Text type="secondary">Please select a database</Text>
          </div>
        )}
      </Sider>

      {/* Third Column - Statistics and Query Panel */}
      <Layout style={{ marginLeft: 480, background: '#f5f5f5' }}>
        <Content style={{ padding: '12px 24px 12px 24px', background: '#f5f5f5', minHeight: '100vh', display: 'flex', flexDirection: 'column' }}>
          {selectedDb ? (
            <>
              {/* Statistics Cards */}
              <div style={{ display: 'flex', gap: '12px', marginBottom: '10px', width: '100%', flexShrink: 0 }}>
                <Card 
                  style={{ 
                    textAlign: 'center', 
                    flex: 1,
                    borderRadius: '6px',
                    boxShadow: '0 1px 3px rgba(0,0,0,0.1)'
                  }}
                  bodyStyle={{ padding: '10px 12px' }}
                >
                  <div style={{ marginBottom: '2px' }}>
                    <Text style={{ fontSize: '11px', color: '#8c8c8c', fontWeight: 500 }}>TABLES</Text>
                  </div>
                  <div>
                    <Text style={{ fontSize: '18px', fontWeight: 600, color: '#262626' }}>
                      {schema?.tables.length || 0}
                    </Text>
                  </div>
                </Card>
                <Card 
                  style={{ 
                    textAlign: 'center', 
                    flex: 1,
                    borderRadius: '6px',
                    boxShadow: '0 1px 3px rgba(0,0,0,0.1)'
                  }}
                  bodyStyle={{ padding: '10px 12px' }}
                >
                  <div style={{ marginBottom: '2px' }}>
                    <Text style={{ fontSize: '11px', color: '#8c8c8c', fontWeight: 500 }}>VIEWS</Text>
                  </div>
                  <div>
                    <Text style={{ fontSize: '18px', fontWeight: 600, color: '#262626' }}>
                      {schema?.views.length || 0}
                    </Text>
                  </div>
                </Card>
                <Card 
                  style={{ 
                    textAlign: 'center', 
                    flex: 1,
                    borderRadius: '6px',
                    boxShadow: '0 1px 3px rgba(0,0,0,0.1)'
                  }}
                  bodyStyle={{ padding: '10px 12px' }}
                >
                  <div style={{ marginBottom: '2px' }}>
                    <Text style={{ fontSize: '11px', color: '#8c8c8c', fontWeight: 500 }}>ROWS</Text>
                  </div>
                  <div>
                    <Text style={{ fontSize: '18px', fontWeight: 600, color: '#262626' }}>
                      {totalRows}
                    </Text>
                  </div>
                </Card>
                <Card 
                  style={{ 
                    textAlign: 'center', 
                    flex: 1,
                    borderRadius: '6px',
                    boxShadow: '0 1px 3px rgba(0,0,0,0.1)'
                  }}
                  bodyStyle={{ padding: '10px 12px' }}
                >
                  <div style={{ marginBottom: '2px' }}>
                    <Text style={{ fontSize: '11px', color: '#8c8c8c', fontWeight: 500 }}>TIME</Text>
                  </div>
                  <div>
                    <Text style={{ fontSize: '18px', fontWeight: 600, color: '#262626' }}>
                      {executionTime}
                    </Text>
                  </div>
                </Card>
              </div>

              {/* SQL Query Panel */}
              <Card
                title={
                  <Title level={4} style={{ margin: 0, fontSize: '16px', fontWeight: 600 }}>
                    SQL EDITOR
                  </Title>
                }
                extra={
                  <Button
                    type="primary"
                    icon={<PlayCircleOutlined />}
                    onClick={handleExecute}
                    loading={queryLoading}
                    size="large"
                    style={{ 
                      height: '40px', 
                      paddingLeft: '20px', 
                      paddingRight: '20px',
                      borderRadius: '6px',
                      fontSize: '14px',
                      fontWeight: 500
                    }}
                  >
                    EXECUTE
                  </Button>
                }
                style={{ 
                  marginBottom: '10px',
                  borderRadius: '8px',
                  flexShrink: 0
                }}
                bodyStyle={{ padding: '20px' }}
              >
                <div style={{ marginBottom: '16px', minHeight: '280px', borderRadius: '4px', overflow: 'hidden' }}>
                  <SQLEditor 
                    value={sql} 
                    onChange={(value) => setSql(value || '')} 
                    height="280px"
                    onExecute={handleExecute}
                  />
                </div>
              </Card>

              {/* Query Results */}
              {queryResult && (
                <Card
                  style={{ 
                    borderRadius: '8px',
                    flex: 1,
                    minHeight: '300px',
                    marginBottom: '0',
                    display: 'flex',
                    flexDirection: 'column'
                  }}
                  bodyStyle={{ padding: '20px', flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}
                >
                  <div style={{ flex: 1, overflow: 'auto' }}>
                    <QueryResults result={queryResult} loading={queryLoading} />
                  </div>
                </Card>
              )}
            </>
          ) : (
            <Card style={{ borderRadius: '8px' }}>
              <div style={{ textAlign: 'center', padding: '60px', color: '#999' }}>
                <Title level={3} type="secondary">
                  Please select a database to start querying
                </Title>
              </div>
            </Card>
          )}
        </Content>
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
