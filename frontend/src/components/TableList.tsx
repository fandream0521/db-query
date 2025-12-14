import React from 'react';
import { List, Tag, Typography, Descriptions } from 'antd';
import { TableInfo } from '../types/schema';

const { Title } = Typography;

interface TableListProps {
  tables: TableInfo[];
}

const TableList: React.FC<TableListProps> = ({ tables }) => {
  if (tables.length === 0) {
    return <div>No tables found</div>;
  }

  return (
    <List
      dataSource={tables}
      renderItem={(table) => (
        <List.Item>
          <div style={{ width: '100%' }}>
            <Title level={5}>
              {table.name}
              {table.primaryKey && table.primaryKey.length > 0 && (
                <Tag color="blue" style={{ marginLeft: 8 }}>
                  PK: {table.primaryKey.join(', ')}
                </Tag>
              )}
            </Title>
            <Descriptions size="small" column={4} bordered>
              {table.columns.map((col) => (
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

export default TableList;

