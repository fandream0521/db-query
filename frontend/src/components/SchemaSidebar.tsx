import React from 'react';
import { Layout, Typography, Button, Spin, Tree } from 'antd';
import { ReloadOutlined } from '@ant-design/icons';
import { DatabaseConnection } from '../types/database';
import { SchemaMetadata } from '../types/schema';
import { buildSchemaTreeData } from '../utils/schemaTree';

const { Sider } = Layout;
const { Text } = Typography;

interface SchemaSidebarProps {
  selectedDb: DatabaseConnection | null;
  schema: SchemaMetadata | null;
  loading: boolean;
  expandedKeys: React.Key[];
  firstColumnCollapsed: boolean;
  onRefresh: () => void;
  onExpandedKeysChange: (keys: React.Key[]) => void;
}

const SchemaSidebar: React.FC<SchemaSidebarProps> = ({
  selectedDb,
  schema,
  loading,
  expandedKeys,
  firstColumnCollapsed,
  onRefresh,
  onExpandedKeysChange,
}) => {
  return (
    <Sider
      width={256}
      style={{
        background: '#fff',
        borderRight: '1px solid #e8e8e8',
        overflow: 'auto',
        height: '100vh',
        position: 'fixed',
        left: firstColumnCollapsed ? 64 : 224,
        top: 0,
        zIndex: 1,
        transition: 'left 0.3s ease',
      }}
    >
      {selectedDb ? (
        <>
          {/* Header - Yellow background */}
          <div
            style={{
              padding: '16px 20px',
              background: '#fff3cd',
              borderBottom: '1px solid #ffc107',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
            }}
          >
            <Text strong style={{ color: '#856404', fontSize: '14px' }}>
              {selectedDb.name.toUpperCase()}
            </Text>
            <Button
              icon={<ReloadOutlined />}
              onClick={onRefresh}
              size="small"
              style={{
                borderRadius: '4px',
              }}
            >
              REFRESH
            </Button>
          </div>

          {/* Body - Tables Tree */}
          <div style={{ padding: '12px 16px', overflowX: 'hidden', width: '100%', boxSizing: 'border-box' }}>
            {loading ? (
              <Spin size="large" style={{ display: 'block', textAlign: 'center', padding: '40px' }} />
            ) : schema ? (
              <div style={{ width: '100%', overflow: 'hidden' }}>
                <Tree
                  treeData={buildSchemaTreeData(schema)}
                  defaultExpandAll={false}
                  expandedKeys={expandedKeys}
                  onExpand={(keys) => onExpandedKeysChange(keys)}
                  showLine={{ showLeafIcon: false }}
                  style={{
                    background: 'transparent',
                    fontSize: '13px',
                    width: '100%',
                  }}
                  blockNode
                />
              </div>
            ) : (
              <div style={{ textAlign: 'center', padding: '40px', color: '#999' }}>
                <Text type="secondary">No schema available</Text>
              </div>
            )}
          </div>
        </>
      ) : (
        <div style={{ padding: '40px', textAlign: 'center', color: '#999' }}>
          <Text type="secondary">Please select a database</Text>
        </div>
      )}
    </Sider>
  );
};

export default SchemaSidebar;
