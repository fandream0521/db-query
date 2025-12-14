import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import AddDatabaseForm from '../AddDatabaseForm';
import * as databaseApi from '../../api/database';

// Mock the API
jest.mock('../../api/database');

describe('AddDatabaseForm', () => {
  const mockOnSuccess = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders form fields', () => {
    render(<AddDatabaseForm onSuccess={mockOnSuccess} />);
    expect(screen.getByLabelText(/Database Name/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/Connection URL/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Add Database/i })).toBeInTheDocument();
  });

  it('validates required fields', async () => {
    render(<AddDatabaseForm onSuccess={mockOnSuccess} />);
    const submitButton = screen.getByRole('button', { name: /Add Database/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText(/Please enter a database name/i)).toBeInTheDocument();
    });
  });

  it('validates database name format', async () => {
    render(<AddDatabaseForm onSuccess={mockOnSuccess} />);
    const nameInput = screen.getByLabelText(/Database Name/i);
    const urlInput = screen.getByLabelText(/Connection URL/i);

    fireEvent.change(nameInput, { target: { value: 'invalid name with spaces' } });
    fireEvent.change(urlInput, { target: { value: 'postgres://user:pass@localhost:5432/db' } });

    const submitButton = screen.getByRole('button', { name: /Add Database/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText(/Name can only contain/i)).toBeInTheDocument();
    });
  });

  it('validates database URL format', async () => {
    render(<AddDatabaseForm onSuccess={mockOnSuccess} />);
    const nameInput = screen.getByLabelText(/Database Name/i);
    const urlInput = screen.getByLabelText(/Connection URL/i);

    fireEvent.change(nameInput, { target: { value: 'testdb' } });
    fireEvent.change(urlInput, { target: { value: 'invalid-url' } });

    const submitButton = screen.getByRole('button', { name: /Add Database/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText(/Invalid database URL format/i)).toBeInTheDocument();
    });
  });

  it('submits form with valid data', async () => {
    const mockUpsertDatabase = databaseApi.upsertDatabase as jest.MockedFunction<typeof databaseApi.upsertDatabase>;
    mockUpsertDatabase.mockResolvedValue({
      name: 'testdb',
      url: 'postgres://user:pass@localhost:5432/db',
      createdAt: '2025-01-01T00:00:00Z',
      updatedAt: '2025-01-01T00:00:00Z',
    });

    render(<AddDatabaseForm onSuccess={mockOnSuccess} />);
    const nameInput = screen.getByLabelText(/Database Name/i);
    const urlInput = screen.getByLabelText(/Connection URL/i);

    fireEvent.change(nameInput, { target: { value: 'testdb' } });
    fireEvent.change(urlInput, { target: { value: 'postgres://user:pass@localhost:5432/db' } });

    const submitButton = screen.getByRole('button', { name: /Add Database/i });
    fireEvent.click(submitButton);

    await waitFor(() => {
      expect(mockUpsertDatabase).toHaveBeenCalledWith('testdb', {
        url: 'postgres://user:pass@localhost:5432/db',
      });
      expect(mockOnSuccess).toHaveBeenCalled();
    });
  });
});

