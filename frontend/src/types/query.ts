export type CellValue = string | number | boolean | null | Record<string, unknown>;

export interface QueryRequest {
  sql: string;
}

export interface QueryResponse {
  columns: string[];
  rows: CellValue[][];
  rowCount: number;
  executionTimeMs: number;
}

