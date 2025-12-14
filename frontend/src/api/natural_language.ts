import axios from 'axios';
import { NaturalLanguageQueryRequest } from '../types/natural_language';
import { QueryResponse } from '../types/query';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

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

