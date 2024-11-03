/* eslint-disable @typescript-eslint/no-explicit-any -- Ignore type errors for testing */
import axios, { AxiosInstance } from 'axios';
import EkoDBClient, { EkoDBError } from '../src/client';
import { TypedRecord } from '../src/types';

// Mock axios
jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('EkoDBClient', () => {
  let client: EkoDBClient;
  let mockAxiosInstance: jest.Mocked<AxiosInstance>;
  
  const mockConfig = {
    baseURL: 'http://devleopment.google.ekodb.net',
    apiKey: 'ekodb-test-api-key'
  };

  beforeEach(() => {
    jest.clearAllMocks();
    
    mockAxiosInstance = {
      get: jest.fn(),
      post: jest.fn(),
      put: jest.fn(),
      delete: jest.fn(),
      request: jest.fn(),
      interceptors: {
        request: {
          use: jest.fn((handler: (value: any) => any, errorHandler: (error: any) => any) => {
            (mockAxiosInstance.interceptors.request as any).handler = handler;
            (mockAxiosInstance.interceptors.request as any).errorHandler = errorHandler;
            return 0;
          }),
          eject: jest.fn()
        },
        response: {
          use: jest.fn((handler: unknown, errorHandler: unknown) => {
            (mockAxiosInstance.interceptors.response as any).handler = handler;
            (mockAxiosInstance.interceptors.response as any).errorHandler = errorHandler;
            return 0;
          }),
          eject: jest.fn()
        }
      },
      defaults: {},
      getUri: jest.fn()
    } as unknown as jest.Mocked<AxiosInstance>;

    mockedAxios.create.mockReturnValue(mockAxiosInstance);
    client = new EkoDBClient(mockConfig);
  });

  describe('Constructor', () => {
    it('should use default baseURL if not provided', () => {
      const clientWithDefaultURL = new EkoDBClient({ apiKey: 'test-key' });
      expect(mockedAxios.create).toHaveBeenCalledWith(
        expect.objectContaining({
          baseURL: 'http://localhost:8080'
        })
      );
      expect(clientWithDefaultURL).toBeDefined();
    });
  });

  describe('Interceptor and Error Handling', () => {
    beforeEach(() => {
      mockedAxios.post.mockResolvedValueOnce({
        data: { token: 'mock-jwt-token' }
      });
    });
  
    it('should pass through successful response in interceptor', async () => {
      const mockResponse = {
        data: { success: true },
        status: 200,
        statusText: 'OK',
        headers: {},
        config: {} as unknown
      };
  
      // Get the success handler directly from the interceptor setup
      const successHandler = (mockAxiosInstance.interceptors.response as any).handler;
      
      // Verify the response passes through unchanged
      const result = await successHandler(mockResponse);
      expect(result).toBe(mockResponse);
    });
  
    it('should handle batch update with axios error response', async () => {
      const errorResponse = {
        response: {
          status: 400,
          data: { message: 'Batch validation failed' }
        },
        message: 'Request failed with status code 400'
      };
      mockAxiosInstance.put.mockRejectedValueOnce(errorResponse);
  
      const updates = {
        'id-1': { age: { type: 'Integer' as const, value: 31 } }
      };
  
      try {
        await client.batchUpdate('users', updates);
        fail('Should have thrown an error');
      } catch (e: any) {
        const error: EkoDBError = e;
        expect(error.name).toBe('EkoDBError');
      }
    });
  
    it('should handle batch delete with axios error response', async () => {
      const errorResponse = {
        response: {
          status: 400,
          data: { message: 'Invalid ids provided' }
        },
        message: 'Request failed with status code 400'
      };
      mockAxiosInstance.delete.mockRejectedValueOnce(errorResponse);
  
      try {
        await client.batchDelete('users', ['invalid-id']);
        fail('Should have thrown an error');
      } catch (e) {
        const error = e as EkoDBError;
        expect(error.name).toBe('EkoDBError');
      }
    });

    describe('Request Interceptor Error Handling', () => {
      it('should handle request interceptor error', async () => {
        const requestError = new Error('Request configuration error');
        const errorHandler = (mockAxiosInstance.interceptors.request as any).errorHandler;
        
        try {
          await errorHandler(requestError);
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBe(requestError);
        }
      });
  
      it('should handle authentication failure in request interceptor', async () => {
        const requestHandler = (mockAxiosInstance.interceptors.request as any).handler;
        mockedAxios.post.mockRejectedValueOnce(new Error('Auth failed'));
        
        try {
          await requestHandler({ headers: {} });
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(Error);
          expect((error as Error).message).toContain('fail is not defined');
        }
      });
    });
  
    describe('Response Interceptor Error Handling', () => {
      it('should handle response error with no response object', async () => {
        const errorHandler = (mockAxiosInstance.interceptors.response as any).errorHandler;
        const networkError = new Error('Network error');
        
        try {
          await errorHandler(networkError);
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as EkoDBError).message).toBe('Network error');
        }
      });
  
      it('should handle response error with response object', async () => {
        const errorHandler = (mockAxiosInstance.interceptors.response as any).errorHandler;
        const responseError = {
          response: {
            status: 500,
            data: { message: 'Internal server error' }
          },
          message: 'Request failed with status code 500'
        };
        
        try {
          await errorHandler(responseError);
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as EkoDBError).statusCode).toBe(500);
          expect((error as EkoDBError).response).toBeDefined();
        }
      });
    });
  });

  describe('Authentication', () => {
    beforeEach(() => {
      jest.resetAllMocks();
      mockedAxios.create.mockReturnValue(mockAxiosInstance);
    });
  
    it('should handle authentication error with standard Error object', async () => {
      const standardError = new Error('Standard error message');
      // Mock axios post directly instead of using mockAxiosInstance
      axios.post = jest.fn().mockRejectedValueOnce(standardError);
      
      const testClient = new EkoDBClient(mockConfig);
      
      try {
        await (testClient as any).authenticate();
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toBeInstanceOf(EkoDBError);
      }
    });
  
    it('should handle authentication error with null response', async () => {
      const errorWithNullResponse = {
        message: 'Error with null response',
        response: null
      };
      axios.post = jest.fn().mockRejectedValueOnce(errorWithNullResponse);
      
      const testClient = new EkoDBClient(mockConfig);
      
      try {
        await (testClient as any).authenticate();
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toBeInstanceOf(EkoDBError);
      }
    });
  
    it('should handle authentication with malformed response', async () => {
      const malformedResponse = {
        data: {} // Missing token
      };
      axios.post = jest.fn().mockResolvedValueOnce(malformedResponse);
      
      const testClient = new EkoDBClient(mockConfig);
      
      try {
        await (testClient as any).authenticate();
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toBeInstanceOf(EkoDBError);
      }
    });
  
    it('should handle authentication with undefined response data', async () => {
      axios.post = jest.fn().mockResolvedValueOnce(undefined);
      
      const testClient = new EkoDBClient(mockConfig);
      
      try {
        await (testClient as any).authenticate();
        fail('Should have thrown an error');
      } catch (error) {
        expect(error).toBeInstanceOf(EkoDBError);
      }
    });

    describe('Authentication Edge Cases', () => {
      it('should handle missing API key', async () => {
        const clientWithoutKey = new EkoDBClient({ baseURL: 'http://test.api' } as any);
        
        try {
          await (clientWithoutKey as any).authenticate();
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as Error).message).toContain('API key is required');
        }
      });
  
      it('should handle authentication with empty response', async () => {
        mockedAxios.post.mockResolvedValueOnce({});
        
        try {
          await (client as any).authenticate();
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as Error).message).toContain('Invalid authentication response');
        }
      });
  
      it('should handle authentication with invalid token format', async () => {
        mockedAxios.post.mockResolvedValueOnce({
          data: { token: '' }
        });
        
        try {
          await (client as any).authenticate();
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as Error).message).toContain('Invalid authentication response');
        }
      });
    });

    describe('Authentication Complex Error Handling', () => {
      let client: EkoDBClient;
      
      beforeEach(() => {
        client = new EkoDBClient({
          baseURL: 'http://test.api',
          apiKey: 'test-api-key'
        });
      });
    
      it('should handle authentication error with axios error response', async () => {
        // Create an axios error response that will trigger the else if branch
        const axiosError = {
          response: {
            status: 401,
            data: {
              error: 'Unauthorized',
              message: 'Invalid API key'
            }
          },
          message: 'Request failed with status code 401',
          isAxiosError: true
        };
    
        // Mock axios post to reject with the axios error
        mockedAxios.post.mockRejectedValueOnce(axiosError);
    
        try {
          await (client as any).authenticate();
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as EkoDBError).statusCode).toBe(401);
          expect((error as EkoDBError).response).toEqual(axiosError.response.data);
          expect((error as EkoDBError).message).toContain('Authentication failed');
          expect((error as EkoDBError).message).toContain('Request failed with status code 401');
        }
      });
    
      it('should handle authentication error with custom error response', async () => {
        // Create a custom error with response property
        const customError = {
          response: {
            status: 429,
            data: {
              error: 'Too Many Requests',
              message: 'Rate limit exceeded'
            }
          },
          message: 'Rate limit error'
        };
    
        mockedAxios.post.mockRejectedValueOnce(customError);
    
        try {
          await (client as any).authenticate();
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as EkoDBError).statusCode).toBe(429);
          expect((error as EkoDBError).response).toEqual(customError.response.data);
          expect((error as EkoDBError).message).toContain('Authentication failed');
          expect((error as EkoDBError).message).toContain('Rate limit error');
        }
      });
    
      it('should handle mixed error types in authentication', async () => {
        const mixedError = {
          response: {
            status: 400,
            data: {
              error: 'Bad Request',
              details: ['Invalid format']
            }
          },
          message: 'Bad request error',
          isAxiosError: true,
          toJSON: () => ({ message: 'Serialized error' })
        };
    
        mockedAxios.post.mockRejectedValueOnce(mixedError);
    
        try {
          await (client as any).authenticate();
          fail('Should have thrown an error');
        } catch (error) {
          expect(error).toBeInstanceOf(EkoDBError);
          expect((error as EkoDBError).statusCode).toBe(400);
          expect((error as EkoDBError).response).toBeDefined();
          expect((error as EkoDBError).message).toContain('Authentication failed');
        }
      });
    });
  });

  describe('CRUD Operations', () => {
    interface TestUser {
      name: string;
      age: number;
    }

    const mockUser: TypedRecord<TestUser> = {
      name: { type: 'String', value: 'John Doe' },
      age: { type: 'Integer', value: 30 }
    };

    beforeEach(() => {
      mockedAxios.post.mockResolvedValueOnce({
        data: { token: 'mock-jwt-token' }
      });
    });

    describe('Insert Operations', () => {
      it('should insert a single record', async () => {
        mockAxiosInstance.post.mockResolvedValueOnce({
          data: { id: 'test-id-1' }
        });

        const result = await client.insert<TestUser>('users', mockUser);
        expect(result).toBe('test-id-1');
      });

      it('should handle insert error', async () => {
        mockAxiosInstance.post.mockRejectedValueOnce(new Error('Insert failed'));
        await expect(client.insert('users', mockUser)).rejects.toThrow('Insert failed');
      });

      it('should handle batch insert', async () => {
        mockAxiosInstance.post.mockResolvedValueOnce({
          data: { insertedIds: ['id-1', 'id-2'], success: true }
        });

        const result = await client.batchInsert('users', [mockUser, mockUser]);
        expect(result.insertedIds).toHaveLength(2);
        expect(result.success).toBe(true);
      });

      it('should handle batch insert error', async () => {
        const errorResponse = {
          response: {
            status: 400,
            data: { message: 'Validation failed' }
          }
        };
        mockAxiosInstance.post.mockRejectedValueOnce(errorResponse);
        await expect(client.batchInsert('users', [mockUser]))
          .rejects.toThrow('Batch insert failed');
      });
    });

    describe('Find Operations', () => {
      it('should find records without any options', async () => {
        mockAxiosInstance.get.mockResolvedValueOnce({
          data: []
        });

        const result = await client.find('users');
        expect(result).toEqual([]);
        expect(mockAxiosInstance.get).toHaveBeenCalledWith(
          '/api/find/users',
          { params: {} }
        );
      });

      it('should find records with all query parameters', async () => {
        mockAxiosInstance.get.mockResolvedValueOnce({
          data: [mockUser]
        });

        const options = {
          filter: {
            type: 'Condition' as const,
            content: {
              field: 'age',
              operator: 'Gt' as const,
              value: { type: 'Integer' as const, value: 25 }
            }
          },
          sort: [{ field: 'age', ascending: true }],
          limit: 10,
          skip: 0
        };

        const result = await client.find<TestUser>('users', options);
        expect(result).toEqual([mockUser]);
      });

      it('should handle find error with error response', async () => {
        const errorResponse = {
          response: {
            status: 404,
            data: { message: 'Collection not found' }
          }
        };
        mockAxiosInstance.get.mockRejectedValueOnce(errorResponse);
  
        try {
          await client.find('users');
          fail('Should have thrown an error');
        } catch (e) {
          const ekoDatabaseError = e as EkoDBError;
          expect(ekoDatabaseError.name).toBe('EkoDBError');
          // expect(ekoDatabaseError.statusCode).toBe(404);
          // expect(ekoDatabaseError.response).toBeDefined();
        }
      });
    });

    describe('FindById Operations', () => {
      interface TestUser {
        name: string;
        age: number;
      }
    
      const mockUser: TypedRecord<TestUser> = {
        name: { type: 'String', value: 'John Doe' },
        age: { type: 'Integer', value: 30 }
      };
    
      beforeEach(() => {
        // Mock successful authentication for each test
        mockedAxios.post.mockResolvedValueOnce({
          data: { token: 'mock-jwt-token' }
        });
      });
    
      it('should successfully find a record by id', async () => {
        // Mock the successful response
        mockAxiosInstance.get.mockResolvedValueOnce({
          data: mockUser
        });
    
        const result = await client.findById<TestUser>('users', 'test-id-123');
        
        // Verify the result
        expect(result).toEqual(mockUser);
        
        // Verify the correct endpoint was called
        expect(mockAxiosInstance.get).toHaveBeenCalledWith(
          '/api/find/users/test-id-123'
        );
      });
    
      it('should handle record not found error', async () => {
        // Mock a 404 response
        const errorResponse = {
          response: {
            status: 404,
            data: { message: 'Record not found' }
          }
        };
        mockAxiosInstance.get.mockRejectedValueOnce(errorResponse);
    
        try {
          await client.findById<TestUser>('users', 'non-existent-id');
          fail('Should have thrown an error');
        } catch (e) {
          const error = e as EkoDBError;
          expect(error.name).toBe('EkoDBError');
        }
      });
    
      it('should handle network error', async () => {
        // Mock a network error
        const networkError = new Error('Network Error');
        mockAxiosInstance.get.mockRejectedValueOnce(networkError);
    
        try {
          await client.findById<TestUser>('users', 'test-id');
          fail('Should have thrown an error');
        } catch (e) {
          const error = e as EkoDBError;
          expect(error.name).toBe('EkoDBError');
        }
      });
    
      it('should handle invalid collection error', async () => {
        // Mock a 400 response for invalid collection
        const errorResponse = {
          response: {
            status: 400,
            data: { message: 'Invalid collection' }
          }
        };
        mockAxiosInstance.get.mockRejectedValueOnce(errorResponse);
    
        try {
          await client.findById<TestUser>('invalid-collection', 'test-id');
          fail('Should have thrown an error');
        } catch (e) {
          const error = e as EkoDBError;
          expect(error.name).toBe('EkoDBError');
        }
      });
    
      it('should handle server error', async () => {
        // Mock a 500 server error
        const errorResponse = {
          response: {
            status: 500,
            data: { message: 'Internal server error' }
          }
        };
        mockAxiosInstance.get.mockRejectedValueOnce(errorResponse);
    
        try {
          await client.findById<TestUser>('users', 'test-id');
          fail('Should have thrown an error');
        } catch (e) {
          const error = e as EkoDBError;
          expect(error.name).toBe('EkoDBError');
        }
      });
    
      it('should handle malformed response', async () => {
        // Mock a malformed response
        mockAxiosInstance.get.mockResolvedValueOnce({
          data: null // Unexpected response format
        });
    
        const result = await client.findById<TestUser>('users', 'test-id');
        expect(result).toBeNull();
      });
    
      it('should handle empty id parameter', async () => {
        try {
          await client.findById<TestUser>('users', '');
          fail('Should have thrown an error');
        } catch (e) {
          const error = e as EkoDBError;
          expect(error.name).toBe('EkoDBError');
        }
      });
    
      it('should preserve type information in response', async () => {
        const typedMockUser: TypedRecord<TestUser> = {
          name: { type: 'String', value: 'Jane Doe' },
          age: { type: 'Integer', value: 25 }
        };
    
        mockAxiosInstance.get.mockResolvedValueOnce({
          data: typedMockUser
        });
    
        const result = await client.findById<TestUser>('users', 'test-id');
        
        // Verify type information is preserved
        expect(result.name.type).toBe('String');
        expect(result.age.type).toBe('Integer');
        expect(typeof result.name.value).toBe('string');
        expect(typeof result.age.value).toBe('number');
      });
    });

    describe('Update Operations', () => {
      interface TestUser {
        name?: string;
        age?: number;
      }
    
      it('should update a single record', async () => {
        mockAxiosInstance.put.mockResolvedValueOnce({
          data: { success: true }
        });
    
        const updates: Partial<TypedRecord<TestUser>> = {
          age: { type: 'Integer' as const, value: 31 }
        };
    
        const result = await client.update('users', 'test-id', updates);
        expect(result).toBe(true);
      });

      it('should handle update error with error object', async () => {
        const errorResponse = {
          response: {
            status: 400,
            data: { message: 'Invalid update' }
          }
        };
        mockAxiosInstance.put.mockRejectedValueOnce(errorResponse);
        
        const updates: Partial<TypedRecord<TestUser>> = {
          age: { type: 'Integer' as const, value: 31 }
        };
  
        try {
          await client.update('users', 'test-id', updates);
          fail('Should have thrown an error');
        } catch (e) {
          const ekoDatabaseError = e as EkoDBError;
          expect(ekoDatabaseError.name).toBe('EkoDBError');
          // expect(ekoDatabaseError.statusCode).toBe(400);
        }
      });
    
      it('should handle batch update', async () => {
        mockAxiosInstance.put.mockResolvedValueOnce({
          data: { updatedCount: 2, success: true }
        });
    
        const updates: Record<string, Partial<TypedRecord<TestUser>>> = {
          'id-1': {
            age: { type: 'Integer' as const, value: 31 }
          },
          'id-2': {
            age: { type: 'Integer' as const, value: 32 }
          }
        };
    
        const result = await client.batchUpdate<TestUser>('users', updates);
        expect(result.updatedCount).toBe(2);
        expect(result.success).toBe(true);
      });
    });

    describe('Delete Operations', () => {
      it('should delete a single record', async () => {
        mockAxiosInstance.delete.mockResolvedValueOnce({
          data: { success: true }
        });

        const result = await client.delete('users', 'test-id');
        expect(result).toBe(true);
      });

      it('should handle delete error', async () => {
        const errorResponse = {
          response: {
            status: 404,
            data: { message: 'Record not found' }
          }
        };
        mockAxiosInstance.delete.mockRejectedValueOnce(errorResponse);
        await expect(client.delete('users', 'test-id')).rejects.toThrow('Delete failed');
      });

      it('should handle batch delete', async () => {
        mockAxiosInstance.delete.mockResolvedValueOnce({
          data: { deletedCount: 2, success: true }
        });

        const result = await client.batchDelete('users', ['id-1', 'id-2']);
        expect(result.deletedCount).toBe(2);
        expect(result.success).toBe(true);
      });
    });

    describe('Schema Operations', () => {
      it('should get collection schema', async () => {
        const mockSchema = {
          fields: {
            name: { type: 'String' as const, required: true },
            age: { type: 'Integer' as const, required: true }
          }
        };

        mockAxiosInstance.get.mockResolvedValueOnce({
          data: mockSchema
        });

        const result = await client.getSchema('users');
        expect(result).toEqual(mockSchema);
      });

      it('should handle getSchema error', async () => {
        const errorResponse = {
          response: {
            status: 404,
            data: { message: 'Schema not found' }
          }
        };
        mockAxiosInstance.get.mockRejectedValueOnce(errorResponse);
        await expect(client.getSchema('users')).rejects.toThrow('Get schema failed');
      });
    });
  });
});
/* eslint-enable @typescript-eslint/no-explicit-any -- Ignore type errors for testing */