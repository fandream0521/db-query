import axios from 'axios';
import { QueryRequest, QueryResponse } from '../types/query';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1';

const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

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

