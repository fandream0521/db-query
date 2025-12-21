import { apiClient } from './client';
import { SchemaMetadata } from '../types/schema';

export const getSchemaMetadata = async (dbName: string): Promise<SchemaMetadata> => {
  const response = await apiClient.get<SchemaMetadata>(`/dbs/${dbName}`);
  return response.data;
};

