import React from 'react';
import { Typography } from 'antd';
import { SchemaMetadata } from '../types/schema';

const { Text } = Typography;

export const buildSchemaTreeData = (schema: SchemaMetadata | null) => {
  if (!schema) return [];

  return [
    {
      title: (
        <span style={{ fontWeight: 600, fontSize: '13px' }}>
          Tables ({schema.tables.length})
        </span>
      ),
      key: 'tables-root',
      children: schema.tables.map((table) => ({
        title: (
          <div style={{ display: 'flex', alignItems: 'center', gap: '6px', width: '100%', paddingRight: '8px' }}>
            <Text
              strong
              ellipsis
              style={{
                fontSize: '12px',
                flex: 1,
                minWidth: 0
              }}
            >
              {table.name}
            </Text>
            <span style={{ fontSize: '10px', color: '#8c8c8c', flexShrink: 0 }}>
              ({table.rowCount || 0} rows)
            </span>
          </div>
        ),
        key: `table-${table.name}`,
        children: table.columns.map((col) => {
          const constraints = [];

          if (table.primaryKey?.includes(col.name)) {
            constraints.push({
              title: (
                <span style={{ fontSize: '11px', color: '#1890ff' }}>
                  PK (Primary Key)
                </span>
              ),
              key: `constraint-pk-${table.name}-${col.name}`,
              isLeaf: true,
            });
          }

          if (!col.nullable) {
            constraints.push({
              title: (
                <span style={{ fontSize: '11px', color: '#ff4d4f' }}>
                  NOT NULL
                </span>
              ),
              key: `constraint-null-${table.name}-${col.name}`,
              isLeaf: true,
            });
          }

          constraints.push({
            title: (
              <span style={{ fontSize: '11px', color: '#8c8c8c' }}>
                Type: {col.dataType.toUpperCase()}
              </span>
            ),
            key: `constraint-type-${table.name}-${col.name}`,
            isLeaf: true,
          });

          if (col.defaultValue) {
            constraints.push({
              title: (
                <span style={{ fontSize: '11px', color: '#8c8c8c' }}>
                  Default: {col.defaultValue}
                </span>
              ),
              key: `constraint-default-${table.name}-${col.name}`,
              isLeaf: true,
            });
          }

          return {
            title: (
              <div style={{
                display: 'flex',
                alignItems: 'center',
                width: '100%',
                paddingRight: '8px',
                boxSizing: 'border-box'
              }}>
                <Text
                  code
                  ellipsis
                  style={{
                    fontSize: '12px',
                    background: 'transparent',
                    padding: 0,
                    border: 'none',
                    flex: 1,
                    minWidth: 0
                  }}
                >
                  {col.name}
                </Text>
              </div>
            ),
            key: `field-${table.name}-${col.name}`,
            children: constraints.length > 0 ? constraints : undefined,
            isLeaf: constraints.length === 0,
          };
        }),
      })),
    },
  ];
};
