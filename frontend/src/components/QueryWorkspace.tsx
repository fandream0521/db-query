import React from 'react';
import { Layout, Typography, Card, Button } from 'antd';
import { PlayCircleOutlined } from '@ant-design/icons';
import { DatabaseConnection } from '../types/database';
import { SchemaMetadata } from '../types/schema';
import { QueryResponse } from '../types/query';
import SQLEditor from './SQLEditor';
import QueryResults from './QueryResults';

const { Content } = Layout;
const { Title, Text } = Typography;

interface QueryWorkspaceProps {
  selectedDb: DatabaseConnection | null;
  schema: SchemaMetadata | null;
  sql: string;
  queryResult: QueryResponse | null;
  queryLoading: boolean;
  executionTime: string;
  onSqlChange: (value: string) => void;
  onExecuteQuery: () => void;
}

const QueryWorkspace: React.FC<QueryWorkspaceProps> = ({
  selectedDb,
  schema,
  sql,
  queryResult,
  queryLoading,
  executionTime,
  onSqlChange,
  onExecuteQuery,
}) => {
  return (
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
                boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
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
                boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
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
                boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
              }}
              bodyStyle={{ padding: '10px 12px' }}
            >
              <div style={{ marginBottom: '2px' }}>
                <Text style={{ fontSize: '11px', color: '#8c8c8c', fontWeight: 500 }}>ROWS</Text>
              </div>
              <div>
                <Text style={{ fontSize: '18px', fontWeight: 600, color: '#262626' }}>
                  {queryResult?.rowCount ?? 0}
                </Text>
              </div>
            </Card>
            <Card
              style={{
                textAlign: 'center',
                flex: 1,
                borderRadius: '6px',
                boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
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
                onClick={onExecuteQuery}
                loading={queryLoading}
                size="large"
                style={{
                  height: '40px',
                  paddingLeft: '20px',
                  paddingRight: '20px',
                  borderRadius: '6px',
                  fontSize: '14px',
                  fontWeight: 500,
                }}
              >
                EXECUTE
              </Button>
            }
            style={{
              marginBottom: '10px',
              borderRadius: '8px',
              flexShrink: 0,
            }}
            bodyStyle={{ padding: '20px' }}
          >
            <div style={{ marginBottom: '16px', minHeight: '280px', borderRadius: '4px', overflow: 'hidden' }}>
              <SQLEditor
                value={sql}
                onChange={(value) => onSqlChange(value || '')}
                height="280px"
                onExecute={onExecuteQuery}
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
                flexDirection: 'column',
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
  );
};

export default QueryWorkspace;
