import axios from 'axios';
import { SchemaMetadata } from '../types/schema';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const getSchemaMetadata = async (dbName: string): Promise<SchemaMetadata> => {
  const response = await apiClient.get<SchemaMetadata>(`/dbs/${dbName}`);
  return response.data;
};

