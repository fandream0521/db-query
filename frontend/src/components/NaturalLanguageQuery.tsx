import React, { useState } from 'react';
import { Card, Input, Button, Space, Alert } from 'antd';
import { ThunderboltOutlined } from '@ant-design/icons';
import QueryResults from './QueryResults';
import { executeNaturalLanguageQuery } from '../api/natural_language';
import { QueryResponse } from '../types/query';
import { showError, showSuccess, showWarning } from '../utils/error';

const { TextArea } = Input;

interface NaturalLanguageQueryProps {
  dbName: string;
}

const NaturalLanguageQuery: React.FC<NaturalLanguageQueryProps> = ({ dbName }) => {
  const [prompt, setPrompt] = useState<string>('');
  const [result, setResult] = useState<QueryResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [generatedSql, setGeneratedSql] = useState<string | null>(null);

  const handleExecute = async () => {
    if (!prompt.trim()) {
      showWarning('Please enter a natural language query');
      return;
    }

    setLoading(true);
    setError(null);
    setResult(null);
    setGeneratedSql(null);

    try {
      const response = await executeNaturalLanguageQuery(dbName, { prompt });
      setResult(response);
      showSuccess('Query executed successfully');
    } catch (err: unknown) {
      const errorResponse = err && typeof err === 'object' && 'response' in err
        ? (err as any).response?.data
        : null;
      const errorMessage =
        errorResponse?.error || (err instanceof Error ? err.message : 'Failed to execute natural language query');
      setError(errorMessage);
      showError(err, 'Failed to execute natural language query');
      
      // Try to extract generated SQL from error if available
      if (errorResponse?.details?.sql) {
        setGeneratedSql(errorResponse.details.sql);
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card title="Natural Language Query" style={{ marginTop: '24px' }}>
      <Space direction="vertical" style={{ width: '100%' }} size="large">
        <div>
          <TextArea
            rows={4}
            placeholder="Enter your question in natural language, e.g., 'Show me all users from the users table'"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            disabled={loading}
          />
        </div>
        <div>
          <Button
            type="primary"
            icon={<ThunderboltOutlined />}
            onClick={handleExecute}
            loading={loading}
            size="large"
          >
            Generate & Execute Query
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
        {generatedSql && (
          <Alert
            message="Generated SQL"
            description={<pre style={{ margin: 0, whiteSpace: 'pre-wrap' }}>{generatedSql}</pre>}
            type="info"
            showIcon
            closable
            onClose={() => setGeneratedSql(null)}
          />
        )}
        <QueryResults result={result} loading={loading} />
      </Space>
    </Card>
  );
};

export default NaturalLanguageQuery;

