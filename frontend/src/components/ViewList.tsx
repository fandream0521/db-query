import React from 'react';
import { Collapse, Tag, Typography, Space } from 'antd';
import { ViewInfo } from '../types/schema';

const { Text } = Typography;

interface ViewListProps {
  views: ViewInfo[];
}

const ViewList: React.FC<ViewListProps> = ({ views }) => {
  if (views.length === 0) {
    return <div style={{ color: '#999', padding: '16px', textAlign: 'center' }}>No views found</div>;
  }

  return (
    <Collapse
      ghost
      items={views.map((view) => ({
        key: view.name,
        label: (
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', width: '100%' }}>
            <Space>
              <Text strong>{view.name}</Text>
              <Tag color="green">VIEW</Tag>
            </Space>
          </div>
        ),
        children: (
          <div style={{ padding: '8px 0' }}>
            {view.columns.map((col, index) => (
              <div
                key={col.name}
                style={{
                  padding: '8px 12px',
                  background: index % 2 === 0 ? '#fafafa' : 'transparent',
                  borderRadius: '4px',
                  marginBottom: '4px',
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                }}
              >
                <Space>
                  <Text code style={{ fontSize: '13px' }}>
                    {col.name}
                  </Text>
                  {!col.nullable && (
                    <Tag color="red" style={{ margin: 0 }}>NOT NULL</Tag>
                  )}
                </Space>
                <Text type="secondary" style={{ fontSize: '12px' }}>
                  {col.dataType}
                </Text>
              </div>
            ))}
          </div>
        ),
      }))}
    />
  );
};

export default ViewList;
