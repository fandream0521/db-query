import { AxiosError } from 'axios';
import { message } from 'antd';

export interface ErrorResponse {
  error: string;
  code?: string;
  details?: any;
}

/**
 * Extract error message from an error object
 */
export const getErrorMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'string') {
    return error;
  }
  return 'An unknown error occurred';
};

/**
 * Extract error response from Axios error
 */
export const getErrorResponse = (error: unknown): ErrorResponse | null => {
  if (error && typeof error === 'object' && 'response' in error) {
    const axiosError = error as AxiosError<ErrorResponse>;
    if (axiosError.response?.data) {
      return axiosError.response.data;
    }
  }
  return null;
};

/**
 * Show error message using Ant Design message component
 */
export const showError = (error: unknown, defaultMessage?: string): void => {
  const errorResponse = getErrorResponse(error);
  const errorMessage =
    errorResponse?.error || getErrorMessage(error) || defaultMessage || 'An error occurred';
  message.error(errorMessage);
};

/**
 * Show success message
 */
export const showSuccess = (msg: string): void => {
  message.success(msg);
};

/**
 * Show warning message
 */
export const showWarning = (msg: string): void => {
  message.warning(msg);
};

/**
 * Show info message
 */
export const showInfo = (msg: string): void => {
  message.info(msg);
};

