import { useState, useCallback } from 'react';
import { DatabaseConnection } from '../types/database';
import { listDatabases } from '../api/database';
import { showError } from '../utils/error';

export const useDatabaseManagement = () => {
  const [databases, setDatabases] = useState<DatabaseConnection[]>([]);
  const [selectedDb, setSelectedDb] = useState<DatabaseConnection | null>(null);
  const [loading, setLoading] = useState(false);

  const loadDatabases = useCallback(async () => {
    setLoading(true);
    try {
      const data = await listDatabases();
      setDatabases(data);
      return data;
    } catch (error: unknown) {
      showError(error, 'Failed to load databases');
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  const selectDatabase = useCallback((db: DatabaseConnection | null) => {
    setSelectedDb(db);
  }, []);

  return {
    databases,
    selectedDb,
    loading,
    loadDatabases,
    selectDatabase,
  };
};
