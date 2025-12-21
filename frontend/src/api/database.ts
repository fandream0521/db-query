import { apiClient } from './client';
import { DatabaseConnection, CreateDatabaseRequest } from '../types/database';

export const listDatabases = async (): Promise<DatabaseConnection[]> => {
  const response = await apiClient.get<DatabaseConnection[]>('/dbs');
  return response.data;
};

export const getDatabase = async (name: string): Promise<DatabaseConnection> => {
  const response = await apiClient.get<DatabaseConnection>(`/dbs/${name}`);
  return response.data;
};

export const upsertDatabase = async (
  name: string,
  request: CreateDatabaseRequest
): Promise<DatabaseConnection> => {
  const response = await apiClient.put<DatabaseConnection>(`/dbs/${name}`, request);
  return response.data;
};

export const deleteDatabase = async (name: string): Promise<void> => {
  await apiClient.delete(`/dbs/${name}`);
};

