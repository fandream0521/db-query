import React, { useState } from 'react';
import { Layout, Typography, Row, Col } from 'antd';
import DatabaseList from './components/DatabaseList';
import AddDatabaseForm from './components/AddDatabaseForm';
import SchemaView from './components/SchemaView';
import { DatabaseConnection } from './types/database';
import 'antd/dist/reset.css';
import './App.css';

const { Header, Content } = Layout;
const { Title } = Typography;

function App() {
  const [selectedDb, setSelectedDb] = useState<DatabaseConnection | null>(null);
  const [refreshKey, setRefreshKey] = useState(0);

  const handleDatabaseSelect = (db: DatabaseConnection) => {
    setSelectedDb(db);
  };

  const handleRefresh = () => {
    setRefreshKey((prev) => prev + 1);
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header style={{ background: '#001529', padding: '0 24px' }}>
        <Title level={3} style={{ color: '#fff', margin: '16px 0' }}>
          Database Query Tool
        </Title>
      </Header>
      <Content style={{ padding: '24px' }}>
        <Row gutter={[24, 24]}>
          <Col xs={24} lg={12}>
            <AddDatabaseForm onSuccess={handleRefresh} />
          </Col>
          <Col xs={24} lg={12}>
            <DatabaseList
              key={refreshKey}
              onSelect={handleDatabaseSelect}
              onRefresh={handleRefresh}
            />
          </Col>
        </Row>
        {selectedDb && (
          <Row style={{ marginTop: '24px' }}>
            <Col span={24}>
              <SchemaView dbName={selectedDb.name} />
            </Col>
          </Row>
        )}
      </Content>
    </Layout>
  );
}

export default App;

