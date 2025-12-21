import { apiClient } from './client';
import { QueryRequest, QueryResponse } from '../types/query';

export const executeQuery = async (
  dbName: string,
  request: QueryRequest
): Promise<QueryResponse> => {
  const response = await apiClient.post<QueryResponse>(
    `/dbs/${dbName}/query`,
    request
  );
  return response.data;
};

