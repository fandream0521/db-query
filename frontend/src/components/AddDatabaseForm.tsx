import React, { useState } from 'react';
import { Form, Input, Button, Space, Modal } from 'antd';
import { CreateDatabaseRequest } from '../types/database';
import { upsertDatabase } from '../api/database';
import { showError, showSuccess } from '../utils/error';

interface AddDatabaseFormProps {
  onSuccess?: () => void;
  onCancel?: () => void;
}

const AddDatabaseForm: React.FC<AddDatabaseFormProps> = ({ onSuccess, onCancel }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (values: { name: string; url: string }) => {
    setLoading(true);
    try {
      const request: CreateDatabaseRequest = { url: values.url };
      await upsertDatabase(values.name, request);
      showSuccess(`Database '${values.name}' added successfully`);
      form.resetFields();
      if (onSuccess) {
        onSuccess();
      }
    } catch (error: unknown) {
      showError(error, 'Failed to add database');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal
      title="Add Database Connection"
      open={true}
      onCancel={onCancel}
      footer={null}
      width={600}
    >
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
          <Space style={{ width: '100%', justifyContent: 'flex-end' }}>
            <Button onClick={onCancel}>Cancel</Button>
            <Button type="primary" htmlType="submit" loading={loading}>
              Add Database
            </Button>
          </Space>
        </Form.Item>
      </Form>
    </Modal>
  );
};

export default AddDatabaseForm;

