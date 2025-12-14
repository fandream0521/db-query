import React from 'react';
import { Collapse, Tag, Typography, Space } from 'antd';
import { TableInfo } from '../types/schema';

const { Text } = Typography;

interface TableListProps {
  tables: TableInfo[];
}

const TableList: React.FC<TableListProps> = ({ tables }) => {
  if (tables.length === 0) {
    return <div style={{ color: '#999', padding: '16px', textAlign: 'center', fontSize: '13px' }}>No tables found</div>;
  }

  return (
    <Collapse
      ghost
      items={tables.map((table) => ({
        key: table.name,
        label: (
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', width: '100%' }}>
            <Space size="small">
              <Text strong style={{ fontSize: '13px' }}>{table.name}</Text>
              <Tag color="blue" style={{ margin: 0, fontSize: '11px', padding: '0 6px', height: '20px', lineHeight: '20px' }}>TABLE</Tag>
              <Text type="secondary" style={{ fontSize: '11px' }}>
                {table.rowCount || 0} ROWS
              </Text>
            </Space>
          </div>
        ),
        children: (
          <div style={{ padding: '6px 0' }}>
            {table.columns.map((col, index) => (
              <div
                key={col.name}
                className="table-column-item"
                style={{
                  padding: '6px 10px',
                  background: index % 2 === 0 ? '#fafafa' : 'transparent',
                  borderRadius: '3px',
                  marginBottom: '2px',
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  fontSize: '12px',
                }}
              >
                <Space size="small">
                  <Text code style={{ fontSize: '12px', background: 'transparent', padding: 0, border: 'none' }}>
                    {col.name}
                  </Text>
                  {table.primaryKey?.includes(col.name) && (
                    <Tag color="blue" style={{ margin: 0, fontSize: '10px', padding: '0 4px', height: '18px', lineHeight: '18px' }}>PK</Tag>
                  )}
                  {!col.nullable && (
                    <Tag color="red" style={{ margin: 0, fontSize: '10px', padding: '0 4px', height: '18px', lineHeight: '18px' }}>NOT NULL</Tag>
                  )}
                </Space>
                <Space size="small">
                  <Text type="secondary" style={{ fontSize: '11px' }}>
                    {col.dataType.toUpperCase()}
                  </Text>
                  {col.defaultValue && (
                    <Text type="secondary" style={{ fontSize: '10px' }}>
                      default: {col.defaultValue}
                    </Text>
                  )}
                </Space>
              </div>
            ))}
          </div>
        ),
      }))}
    />
  );
};

export default TableList;
