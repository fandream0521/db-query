import { useState, useCallback } from 'react';
import { QueryResponse } from '../types/query';
import { executeQuery } from '../api/query';
import { showError, showSuccess } from '../utils/error';

export const useQueryExecution = (dbName: string | null) => {
  const [sql, setSql] = useState<string>('SELECT * FROM');
  const [queryResult, setQueryResult] = useState<QueryResponse | null>(null);
  const [queryLoading, setQueryLoading] = useState(false);
  const [executionTime, setExecutionTime] = useState<string>('-');

  const executeQueryHandler = useCallback(async () => {
    if (!sql.trim() || !dbName) {
      return;
    }

    setQueryLoading(true);
    setQueryResult(null);
    const startTime = Date.now();

    try {
      const response = await executeQuery(dbName, { sql });
      const endTime = Date.now();
      setExecutionTime(`${((endTime - startTime) / 1000).toFixed(2)}s`);
      setQueryResult(response);
      showSuccess('Query executed successfully');
    } catch (err: unknown) {
      showError(err, 'Failed to execute query');
    } finally {
      setQueryLoading(false);
    }
  }, [sql, dbName]);

  const resetQuery = useCallback(() => {
    setSql('SELECT * FROM');
    setQueryResult(null);
    setExecutionTime('-');
  }, []);

  return {
    sql,
    setSql,
    queryResult,
    queryLoading,
    executionTime,
    executeQuery: executeQueryHandler,
    resetQuery,
  };
};
