import axios, { AxiosInstance, AxiosError } from 'axios';
import {
  ClientConfig,
  FindOptions,
  TypedRecord,
  BatchInsertResponse,
  BatchUpdateResponse,
  BatchDeleteResponse,
  CollectionSchema,
  TokenResponse,
} from './types';

export class EkoDBError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
    public response?:
      | {
          error: string;
          message: string;
          status: number;
          data?: unknown;
        }
      | unknown
      | undefined
  ) {
    super(message);
    this.name = 'EkoDBError';
  }
}

export class EkoDBClient {
  private baseURL: string;
  private apiKey: string;
  private token: string | null = null;
  private axiosInstance: AxiosInstance;

  constructor(config: ClientConfig) {
    this.baseURL = config.baseURL || 'http://localhost:8080';
    this.apiKey = config.apiKey;

    this.axiosInstance = axios.create({
      baseURL: this.baseURL,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add interceptor to include auth token
    this.axiosInstance.interceptors.request.use(
      async (config) => {
        if (!this.token) {
          await this.authenticate();
        }
        config.headers.Authorization = `Bearer ${this.token}`;
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Add response interceptor for error handling
    this.axiosInstance.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => {
        if (error.response) {
          throw new EkoDBError(
            error.message,
            error.response.status,
            error.response.data
          );
        }
        throw new EkoDBError(error.message);
      }
    );
  }

  private async authenticate(): Promise<void> {
    try {
      if (!this.apiKey) {
        throw new Error('API key is required');
      }

      const response = await axios.post<TokenResponse>(
        `${this.baseURL}/api/auth/token`,
        { api_key: this.apiKey }
      );

      if (!response?.data?.token) {
        throw new Error('Invalid authentication response: missing token');
      }

      this.token = response.data.token;
    } catch (error) {
      let message = 'Authentication failed';
      let statusCode: number | undefined;
      let responseData: unknown;

      if (error instanceof Error) {
        message = `${message}: ${error.message}`;
      } else if ((error as EkoDBError).response) {
        statusCode = (error as AxiosError).response?.status;
        responseData = (error as AxiosError).response?.data;
        message = `${message}: ${(error as EkoDBError).message}`;
      }

      throw new EkoDBError(message, statusCode, responseData);
    }
  }

  async insert<T>(collection: string, record: TypedRecord<T>): Promise<string> {
    try {
      const response = await this.axiosInstance.post<{ id: string }>(
        `/api/insert/${collection}`,
        record
      );
      return response.data.id;
    } catch (error) {
      throw new EkoDBError(`Insert failed: ${(error as Error).message}`);
    }
  }

  async batchInsert<T>(
    collection: string,
    records: TypedRecord<T>[]
  ): Promise<BatchInsertResponse> {
    try {
      const response = await this.axiosInstance.post<BatchInsertResponse>(
        `/api/batch/insert/${collection}`,
        { records }
      );
      return response.data;
    } catch (error) {
      throw new EkoDBError(`Batch insert failed: ${(error as Error).message}`);
    }
  }

  async find<T>(
    collection: string,
    options: FindOptions = {}
  ): Promise<TypedRecord<T>[]> {
    try {
      const queryParams: Record<string, string> = {};

      if (options.filter) {
        queryParams.filter = JSON.stringify(options.filter);
      }
      if (options.sort) {
        queryParams.sort = JSON.stringify(options.sort);
      }
      if (options.limit !== undefined) {
        queryParams.limit = options.limit.toString();
      }
      if (options.skip !== undefined) {
        queryParams.skip = options.skip.toString();
      }

      const response = await this.axiosInstance.get<TypedRecord<T>[]>(
        `/api/find/${collection}`,
        { params: queryParams }
      );
      return response.data;
    } catch (error) {
      throw new EkoDBError(`Find failed: ${(error as Error).message}`);
    }
  }

  async findById<T>(collection: string, id: string): Promise<TypedRecord<T>> {
    try {
      const response = await this.axiosInstance.get<TypedRecord<T>>(
        `/api/find/${collection}/${id}`
      );
      return response.data;
    } catch (error) {
      throw new EkoDBError(`Find by ID failed: ${(error as Error).message}`);
    }
  }

  async update<T>(
    collection: string,
    id: string,
    updates: Partial<TypedRecord<T>>
  ): Promise<boolean> {
    try {
      const response = await this.axiosInstance.put<{ success: boolean }>(
        `/api/update/${collection}/${id}`,
        updates
      );
      return response.data.success;
    } catch (error) {
      throw new EkoDBError(`Update failed: ${(error as Error).message}`);
    }
  }

  async batchUpdate<T>(
    collection: string,
    updates: Record<string, Partial<TypedRecord<T>>>
  ): Promise<BatchUpdateResponse> {
    try {
      const response = await this.axiosInstance.put<BatchUpdateResponse>(
        `/api/batch/update/${collection}`,
        { updates }
      );
      return response.data;
    } catch (error) {
      throw new EkoDBError(`Batch update failed: ${(error as Error).message}`);
    }
  }

  async delete(collection: string, id: string): Promise<boolean> {
    try {
      const response = await this.axiosInstance.delete<{ success: boolean }>(
        `/api/delete/${collection}/${id}`
      );
      return response.data.success;
    } catch (error) {
      throw new EkoDBError(`Delete failed: ${(error as Error).message}`);
    }
  }

  async batchDelete(
    collection: string,
    ids: string[]
  ): Promise<BatchDeleteResponse> {
    try {
      const response = await this.axiosInstance.delete<BatchDeleteResponse>(
        `/api/batch/delete/${collection}`,
        { data: { ids } }
      );
      return response.data;
    } catch (error) {
      throw new EkoDBError(`Batch delete failed: ${(error as Error).message}`);
    }
  }

  async getSchema(collection: string): Promise<CollectionSchema> {
    try {
      const response = await this.axiosInstance.get<CollectionSchema>(
        `/api/schemas/${collection}`
      );
      return response.data;
    } catch (error) {
      throw new EkoDBError(`Get schema failed: ${(error as Error).message}`);
    }
  }
}

export default EkoDBClient;
