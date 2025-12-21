import { apiClient } from './client';
import { NaturalLanguageQueryRequest } from '../types/natural_language';
import { QueryResponse } from '../types/query';

export const executeNaturalLanguageQuery = async (
  dbName: string,
  request: NaturalLanguageQueryRequest
): Promise<QueryResponse> => {
  const response = await apiClient.post<QueryResponse>(
    `/dbs/${dbName}/query/natural`,
    request
  );
  return response.data;
};

