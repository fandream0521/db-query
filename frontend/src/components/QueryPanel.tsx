import React, { useState } from 'react';
import { Card, Button, Space, Alert, message, Tabs } from 'antd';
import { PlayCircleOutlined } from '@ant-design/icons';
import SQLEditor from './SQLEditor';
import QueryResults from './QueryResults';
import NaturalLanguageQuery from './NaturalLanguageQuery';
import { executeQuery } from '../api/query';
import { QueryResponse } from '../types/query';
import { showError, showSuccess, showWarning } from '../utils/error';

interface QueryPanelProps {
  dbName: string;
}

const QueryPanel: React.FC<QueryPanelProps> = ({ dbName }) => {
  const [sql, setSql] = useState<string>('');
  const [result, setResult] = useState<QueryResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleExecute = async () => {
    if (!sql.trim()) {
      showWarning('Please enter a SQL query');
      return;
    }

    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const response = await executeQuery(dbName, { sql });
      setResult(response);
      showSuccess('Query executed successfully');
    } catch (err: unknown) {
      const errorResponse = err && typeof err === 'object' && 'response' in err
        ? (err as any).response?.data?.error
        : null;
      const errorMessage = errorResponse || (err instanceof Error ? err.message : 'Failed to execute query');
      setError(errorMessage);
      showError(err, 'Failed to execute query');
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    // Ctrl+Enter or Cmd+Enter to execute
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      handleExecute();
    }
  };

  return (
    <Card style={{ marginTop: '24px' }}>
      <Tabs
        defaultActiveKey="sql"
        items={[
          {
            key: 'sql',
            label: 'SQL Query',
            children: (
              <Space direction="vertical" style={{ width: '100%' }} size="large">
                <div onKeyDown={handleKeyDown}>
                  <SQLEditor value={sql} onChange={(value) => setSql(value || '')} />
                </div>
                <div>
                  <Button
                    type="primary"
                    icon={<PlayCircleOutlined />}
                    onClick={handleExecute}
                    loading={loading}
                    size="large"
                  >
                    Execute Query (Ctrl+Enter)
                  </Button>
                </div>
                {error && (
                  <Alert
                    message="Query Error"
                    description={error}
                    type="error"
                    showIcon
                    closable
                    onClose={() => setError(null)}
                  />
                )}
                <QueryResults result={result} loading={loading} />
              </Space>
            ),
          },
          {
            key: 'natural',
            label: 'Natural Language',
            children: <NaturalLanguageQuery dbName={dbName} />,
          },
        ]}
      />
    </Card>
  );
};

export default QueryPanel;

