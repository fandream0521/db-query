import React, { useState } from 'react';
import { Form, Input, Button, Card, Typography, message, Space } from 'antd';
import { CreateDatabaseRequest } from '../types/database';
import { upsertDatabase } from '../api/database';

const { Title } = Typography;

interface AddDatabaseFormProps {
  onSuccess?: () => void;
}

const AddDatabaseForm: React.FC<AddDatabaseFormProps> = ({ onSuccess }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (values: { name: string; url: string }) => {
    setLoading(true);
    try {
      const request: CreateDatabaseRequest = { url: values.url };
      await upsertDatabase(values.name, request);
      message.success(`Database '${values.name}' added successfully`);
      form.resetFields();
      if (onSuccess) {
        onSuccess();
      }
    } catch (error: any) {
      message.error(`Failed to add database: ${error.response?.data?.error || error.message}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card>
      <Space direction="vertical" style={{ width: '100%' }} size="large">
        <Title level={4}>Add Database Connection</Title>
        <Form
          form={form}
          layout="vertical"
          onFinish={handleSubmit}
          autoComplete="off"
        >
          <Form.Item
            label="Database Name"
            name="name"
            rules={[
              { required: true, message: 'Please enter a database name' },
              { pattern: /^[a-zA-Z0-9_-]+$/, message: 'Name can only contain letters, numbers, dashes, and underscores' },
            ]}
          >
            <Input placeholder="e.g., my-database" />
          </Form.Item>

          <Form.Item
            label="Connection URL"
            name="url"
            rules={[
              { required: true, message: 'Please enter a database URL' },
              {
                pattern: /^(postgres|postgresql|mysql|sqlite):\/\/.+/,
                message: 'Invalid database URL format',
              },
            ]}
          >
            <Input
              placeholder="e.g., postgres://user:password@localhost:5432/dbname"
            />
          </Form.Item>

          <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading} block>
              Add Database
            </Button>
          </Form.Item>
        </Form>
      </Space>
    </Card>
  );
};

export default AddDatabaseForm;

