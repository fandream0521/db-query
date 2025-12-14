import React, { useEffect, useState } from 'react';
import { List, Card, Typography, Button, Space } from 'antd';
import { DatabaseConnection } from '../types/database';
import { listDatabases, deleteDatabase } from '../api/database';
import { showError, showSuccess } from '../utils/error';

const { Title } = Typography;

interface DatabaseListProps {
  onSelect?: (db: DatabaseConnection) => void;
  onRefresh?: () => void;
}

const DatabaseList: React.FC<DatabaseListProps> = ({ onSelect, onRefresh }) => {
  const [databases, setDatabases] = useState<DatabaseConnection[]>([]);
  const [loading, setLoading] = useState(false);

  const loadDatabases = async () => {
    setLoading(true);
    try {
      const data = await listDatabases();
      setDatabases(data);
    } catch (error: unknown) {
      showError(error, 'Failed to load databases');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDatabases();
  }, []);

  const handleDelete = async (name: string) => {
    try {
      await deleteDatabase(name);
      showSuccess(`Database '${name}' deleted successfully`);
      await loadDatabases();
      if (onRefresh) {
        onRefresh();
      }
    } catch (error: unknown) {
      showError(error, 'Failed to delete database');
    }
  };

  return (
    <Card>
      <Space direction="vertical" style={{ width: '100%' }} size="large">
        <Title level={4}>Database Connections</Title>
        <List
          loading={loading}
          dataSource={databases}
          renderItem={(item) => (
            <List.Item
              actions={[
                <Button
                  key="select"
                  type="link"
                  onClick={() => onSelect && onSelect(item)}
                >
                  Select
                </Button>,
                <Button
                  key="delete"
                  type="link"
                  danger
                  onClick={() => handleDelete(item.name)}
                >
                  Delete
                </Button>,
              ]}
            >
              <List.Item.Meta
                title={item.name}
                description={item.url}
              />
            </List.Item>
          )}
          locale={{ emptyText: 'No databases configured' }}
        />
      </Space>
    </Card>
  );
};

export default DatabaseList;

