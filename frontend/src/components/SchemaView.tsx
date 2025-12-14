import React, { useEffect, useState } from 'react';
import { Card, Tabs, Spin, message, Button } from 'antd';
import { ReloadOutlined } from '@ant-design/icons';
import { SchemaMetadata } from '../types/schema';
import { getSchemaMetadata } from '../api/schema';
import TableList from './TableList';
import ViewList from './ViewList';

const { TabPane } = Tabs;

interface SchemaViewProps {
  dbName: string | null;
}

const SchemaView: React.FC<SchemaViewProps> = ({ dbName }) => {
  const [metadata, setMetadata] = useState<SchemaMetadata | null>(null);
  const [loading, setLoading] = useState(false);

  const loadSchema = async () => {
    if (!dbName) {
      setMetadata(null);
      return;
    }

    setLoading(true);
    try {
      const data = await getSchemaMetadata(dbName);
      setMetadata(data);
    } catch (error: any) {
      message.error(`Failed to load schema: ${error.response?.data?.error || error.message}`);
      setMetadata(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadSchema();
  }, [dbName]);

  if (!dbName) {
    return (
      <Card>
        <div style={{ textAlign: 'center', padding: '40px', color: '#999' }}>
          Please select a database to view its schema
        </div>
      </Card>
    );
  }

  if (loading) {
    return (
      <Card>
        <Spin size="large" style={{ display: 'block', textAlign: 'center', padding: '40px' }} />
      </Card>
    );
  }

  if (!metadata) {
    return (
      <Card>
        <div style={{ textAlign: 'center', padding: '40px' }}>
          <p>No schema metadata available</p>
          <Button onClick={loadSchema}>Retry</Button>
        </div>
      </Card>
    );
  }

  return (
    <Card
      title={
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <span>Schema: {metadata.dbName}</span>
          <Button icon={<ReloadOutlined />} onClick={loadSchema}>
            Refresh
          </Button>
        </div>
      }
      extra={
        <span style={{ color: '#999', fontSize: '12px' }}>
          Updated: {new Date(metadata.updatedAt).toLocaleString()}
        </span>
      }
    >
      <Tabs defaultActiveKey="tables">
        <TabPane tab={`Tables (${metadata.tables.length})`} key="tables">
          <TableList tables={metadata.tables} />
        </TabPane>
        <TabPane tab={`Views (${metadata.views.length})`} key="views">
          <ViewList views={metadata.views} />
        </TabPane>
      </Tabs>
    </Card>
  );
};

export default SchemaView;

