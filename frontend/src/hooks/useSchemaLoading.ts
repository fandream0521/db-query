import { useState, useCallback } from 'react';
import { SchemaMetadata } from '../types/schema';
import { getSchemaMetadata } from '../api/schema';
import { showError } from '../utils/error';

export const useSchemaLoading = () => {
  const [schema, setSchema] = useState<SchemaMetadata | null>(null);
  const [loading, setLoading] = useState(false);
  const [expandedKeys, setExpandedKeys] = useState<React.Key[]>([]);

  const loadSchema = useCallback(async (dbName: string) => {
    setLoading(true);
    setSchema(null); // Clear old schema immediately
    setExpandedKeys([]); // Clear expanded state

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
      setLoading(false);
    }
  }, []);

  const clearSchema = useCallback(() => {
    setSchema(null);
    setExpandedKeys([]);
  }, []);

  return {
    schema,
    loading,
    expandedKeys,
    loadSchema,
    clearSchema,
    setExpandedKeys,
  };
};
