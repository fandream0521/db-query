export interface DatabaseConnection {
  name: string;
  url: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateDatabaseRequest {
  url: string;
}

