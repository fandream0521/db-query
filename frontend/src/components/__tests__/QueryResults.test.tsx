import React from 'react';
import { render, screen } from '@testing-library/react';
import QueryResults from '../QueryResults';
import { QueryResponse } from '../../types/query';

describe('QueryResults', () => {
  it('displays loading state', () => {
    render(<QueryResults result={null} loading={true} />);
    expect(screen.getByText('Executing query...')).toBeInTheDocument();
  });

  it('displays empty state when no result', () => {
    render(<QueryResults result={null} loading={false} />);
    expect(screen.getByText(/No query results/)).toBeInTheDocument();
  });

  it('displays empty result set message', () => {
    const emptyResult: QueryResponse = {
      columns: ['id', 'name'],
      rows: [],
      rowCount: 0,
      executionTimeMs: 10,
    };
    render(<QueryResults result={emptyResult} loading={false} />);
    expect(screen.getByText(/returned no rows/)).toBeInTheDocument();
    expect(screen.getByText(/Execution time: 10ms/)).toBeInTheDocument();
  });

  it('displays query results with data', () => {
    const result: QueryResponse = {
      columns: ['id', 'name'],
      rows: [
        [1, 'Alice'],
        [2, 'Bob'],
      ],
      rowCount: 2,
      executionTimeMs: 15,
    };
    render(<QueryResults result={result} loading={false} />);
    expect(screen.getByText('2 rows returned')).toBeInTheDocument();
    expect(screen.getByText('Execution time: 15ms')).toBeInTheDocument();
    expect(screen.getByText('id')).toBeInTheDocument();
    expect(screen.getByText('name')).toBeInTheDocument();
  });

  it('handles null values', () => {
    const result: QueryResponse = {
      columns: ['id', 'name'],
      rows: [[1, null]],
      rowCount: 1,
      executionTimeMs: 5,
    };
    render(<QueryResults result={result} loading={false} />);
    // NULL values should be displayed
    expect(screen.getByText('1 row returned')).toBeInTheDocument();
  });
});

