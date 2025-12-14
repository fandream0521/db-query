export interface SchemaMetadata {
  dbName: string;
  tables: TableInfo[];
  views: ViewInfo[];
  updatedAt: string;
}

export interface TableInfo {
  name: string;
  columns: ColumnInfo[];
  primaryKey?: string[];
  rowCount?: number;
}

export interface ViewInfo {
  name: string;
  columns: ColumnInfo[];
}

export interface ColumnInfo {
  name: string;
  dataType: string;
  nullable: boolean;
  defaultValue?: string;
}

