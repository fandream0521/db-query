import React from 'react';
import { Layout, Typography, Button, Spin, Badge, Tooltip } from 'antd';
import { PlusOutlined, DatabaseOutlined } from '@ant-design/icons';
import { DatabaseConnection } from '../types/database';

const { Sider } = Layout;
const { Title, Text } = Typography;

interface DatabaseSidebarProps {
  databases: DatabaseConnection[];
  selectedDb: DatabaseConnection | null;
  loading: boolean;
  collapsed: boolean;
  onAddDatabase: () => void;
  onSelectDatabase: (db: DatabaseConnection) => void;
  onToggleCollapse: () => void;
}

const DatabaseSidebar: React.FC<DatabaseSidebarProps> = ({
  databases,
  selectedDb,
  loading,
  collapsed,
  onAddDatabase,
  onSelectDatabase,
  onToggleCollapse,
}) => {
  return (
    <Sider
      width={collapsed ? 64 : 224}
      style={{
        background: '#fff',
        borderRight: '1px solid #e8e8e8',
        overflow: 'hidden',
        height: '100vh',
        position: 'fixed',
        left: 0,
        top: 0,
        transition: 'width 0.3s ease',
        zIndex: 2,
      }}
    >
      {/* Header */}
      <div
        style={{
          padding: collapsed ? '20px 12px' : '20px',
          borderBottom: '1px solid #f0f0f0',
          position: 'relative',
          display: 'flex',
          flexDirection: 'column',
          alignItems: collapsed ? 'center' : 'stretch',
          transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
        }}
      >
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '12px',
            marginBottom: '16px',
            cursor: 'pointer',
            userSelect: 'none',
            justifyContent: collapsed ? 'center' : 'flex-start',
            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
          }}
          onClick={onToggleCollapse}
        >
          {!collapsed && (
            <Title
              level={4}
              style={{
                margin: 0,
                fontSize: '18px',
                fontWeight: 600,
                whiteSpace: 'nowrap',
                overflow: 'hidden',
                transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
              }}
            >
              DB QUERY TOOL
            </Title>
          )}
          <DatabaseOutlined
            style={{
              fontSize: '24px',
              color: '#1890ff',
              flexShrink: 0,
              transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
              transform: collapsed ? 'scale(0.9)' : 'scale(1)',
            }}
          />
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          block={!collapsed}
          onClick={onAddDatabase}
          style={{
            height: '40px',
            fontSize: '14px',
            borderRadius: '6px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: collapsed ? '0' : '4px 15px',
            width: collapsed ? '40px' : '100%',
            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
            overflow: 'hidden',
            whiteSpace: 'nowrap',
          }}
          title={collapsed ? 'Add Database' : undefined}
        >
          {!collapsed && (
            <span
              style={{
                transition: 'opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                display: 'inline-block',
              }}
            >
              ADD DATABASE
            </span>
          )}
        </Button>
      </div>

      {/* Database List Body */}
      <div
        style={{
          padding: collapsed ? '16px 12px' : '16px',
          overflow: 'auto',
          height: 'calc(100vh - 120px)',
          display: 'flex',
          flexDirection: 'column',
          alignItems: collapsed ? 'center' : 'stretch',
          transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
        }}
      >
        {loading ? (
          <Spin size="large" style={{ display: 'block', textAlign: 'center', padding: '40px' }} />
        ) : databases.length === 0 ? (
          <div style={{ textAlign: 'center', padding: '40px', color: '#999' }}>
            {!collapsed && <Text type="secondary">No databases added</Text>}
          </div>
        ) : (
          <div>
            {databases.map((db) => {
              const isSelected = selectedDb?.name === db.name;
              return (
                <Tooltip key={db.name} title={collapsed ? db.name : undefined} placement="right">
                  <div
                    onClick={() => onSelectDatabase(db)}
                    className="database-item"
                    style={{
                      padding: collapsed ? '12px' : '14px 16px',
                      marginBottom: '8px',
                      borderRadius: '8px',
                      cursor: 'pointer',
                      background: isSelected ? '#f0f7ff' : 'transparent',
                      border: isSelected ? '1px solid #1890ff' : '1px solid #f0f0f0',
                      transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: collapsed ? 'center' : 'space-between',
                      width: collapsed ? '40px' : '100%',
                      boxSizing: 'border-box',
                    }}
                    onMouseEnter={(e) => {
                      if (!isSelected) {
                        e.currentTarget.style.background = '#fafafa';
                        e.currentTarget.style.borderColor = '#d9d9d9';
                      }
                    }}
                    onMouseLeave={(e) => {
                      if (!isSelected) {
                        e.currentTarget.style.background = 'transparent';
                        e.currentTarget.style.borderColor = '#f0f0f0';
                      }
                    }}
                  >
                    {collapsed ? (
                      <div
                        style={{
                          display: 'flex',
                          flexDirection: 'column',
                          alignItems: 'center',
                          gap: '4px',
                          width: '100%',
                          transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                        }}
                      >
                        {isSelected && (
                          <div
                            style={{
                              width: '6px',
                              height: '6px',
                              borderRadius: '50%',
                              background: '#52c41a',
                              transition: 'opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                            }}
                          />
                        )}
                        <DatabaseOutlined
                          style={{
                            fontSize: '20px',
                            color: isSelected ? '#1890ff' : '#262626',
                            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                          }}
                        />
                      </div>
                    ) : (
                      <>
                        <div
                          style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: '8px',
                            flex: 1,
                            minWidth: 0,
                            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                          }}
                        >
                          {isSelected && (
                            <div
                              style={{
                                width: '6px',
                                height: '6px',
                                borderRadius: '50%',
                                background: '#52c41a',
                                flexShrink: 0,
                                transition: 'opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                              }}
                            />
                          )}
                          <div style={{ flex: 1, minWidth: 0, overflow: 'hidden' }}>
                            <Text
                              strong={isSelected}
                              ellipsis
                              style={{
                                fontSize: '13px',
                                color: isSelected ? '#1890ff' : '#262626',
                                display: 'block',
                                marginBottom: '4px',
                                width: '100%',
                              }}
                            >
                              {db.name}
                            </Text>
                            <Tooltip title={db.url} placement="right">
                              <Text
                                type="secondary"
                                ellipsis
                                style={{
                                  fontSize: '11px',
                                  display: 'block',
                                  width: '100%',
                                  cursor: 'help',
                                }}
                              >
                                {db.url.replace(/\/\/.*@/, '//***@')}
                              </Text>
                            </Tooltip>
                          </div>
                        </div>
                        <Badge
                          count={0}
                          showZero
                          style={{
                            backgroundColor: '#d9d9d9',
                            flexShrink: 0,
                            transition: 'opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                          }}
                        />
                      </>
                    )}
                  </div>
                </Tooltip>
              );
            })}
          </div>
        )}
      </div>
    </Sider>
  );
};

export default DatabaseSidebar;
