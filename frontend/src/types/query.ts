export interface QueryRequest {
  sql: string;
}

export interface QueryResponse {
  columns: string[];
  rows: any[][];
  rowCount: number;
  executionTimeMs: number;
}

