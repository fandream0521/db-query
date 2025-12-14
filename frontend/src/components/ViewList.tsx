import React from 'react';
import { List, Typography, Descriptions, Tag } from 'antd';
import { ViewInfo } from '../types/schema';

const { Title } = Typography;

interface ViewListProps {
  views: ViewInfo[];
}

const ViewList: React.FC<ViewListProps> = ({ views }) => {
  if (views.length === 0) {
    return <div>No views found</div>;
  }

  return (
    <List
      dataSource={views}
      renderItem={(view) => (
        <List.Item>
          <div style={{ width: '100%' }}>
            <Title level={5}>
              {view.name}
              <Tag color="green" style={{ marginLeft: 8 }}>VIEW</Tag>
            </Title>
            <Descriptions size="small" column={4} bordered>
              {view.columns.map((col) => (
                <Descriptions.Item
                  key={col.name}
                  label={
                    <span>
                      {col.name}
                      {!col.nullable && <Tag color="red" style={{ marginLeft: 4 }}>NOT NULL</Tag>}
                    </span>
                  }
                >
                  {col.dataType}
                  {col.defaultValue && (
                    <span style={{ color: '#999', marginLeft: 8 }}>
                      (default: {col.defaultValue})
                    </span>
                  )}
                </Descriptions.Item>
              ))}
            </Descriptions>
          </div>
        </List.Item>
      )}
    />
  );
};

export default ViewList;

