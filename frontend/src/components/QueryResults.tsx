import React from 'react';
import { Table, Typography, Empty } from 'antd';
import { QueryResponse, CellValue } from '../types/query';

const { Text } = Typography;

interface QueryResultsProps {
  result: QueryResponse | null;
  loading?: boolean;
}

const QueryResults: React.FC<QueryResultsProps> = ({ result, loading = false }) => {
  if (loading) {
    return (
      <div className="p-4 md:p-6 text-center">
        <Text>Executing query...</Text>
      </div>
    );
  }

  if (!result) {
    return (
      <div className="p-4 md:p-6">
        <Empty description="No query results. Execute a query to see results here." />
      </div>
    );
  }

  if (result.rowCount === 0) {
    return (
      <div className="p-4 md:p-6">
        <Empty description="Query executed successfully but returned no rows." />
        <Text type="secondary" className="mt-2 block text-xs sm:text-sm">
          Execution time: {result.executionTimeMs}ms
        </Text>
      </div>
    );
  }

  // Convert columns and rows to Ant Design Table format
  const columns = result.columns.map((col) => ({
    title: col,
    dataIndex: col,
    key: col,
    render: (text: CellValue) => {
      if (text === null || text === undefined) {
        return <Text type="secondary">NULL</Text>;
      }
      if (typeof text === 'object') {
        return JSON.stringify(text);
      }
      return String(text);
    },
  }));

  const dataSource = result.rows.map((row, index) => {
    const record: Record<string, CellValue> = { key: index };
    result.columns.forEach((col, colIndex) => {
      record[col] = row[colIndex];
    });
    return record;
  });

  return (
    <div className="p-4 md:p-6">
      <div className="mb-4 flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4">
        <Text strong className="text-sm sm:text-base">
          {result.rowCount} row{result.rowCount !== 1 ? 's' : ''} returned
        </Text>
        <Text type="secondary" className="text-xs sm:text-sm">
          Execution time: {result.executionTimeMs}ms
        </Text>
      </div>
      <div className="overflow-x-auto">
        <Table
          columns={columns}
          dataSource={dataSource}
          pagination={{
            pageSize: 50,
            showSizeChanger: true,
            showTotal: (total) => `Total ${total} rows`,
            responsive: true,
          }}
          scroll={{ x: 'max-content' }}
          size="small"
          className="w-full"
        />
      </div>
    </div>
  );
};

export default QueryResults;

