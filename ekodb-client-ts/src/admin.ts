/** API-key administration. Import from @ekodb/ekodb-client/admin. */
import { EkoDBClient, type ClientConfig } from "./client";

export type AdminClientConfig = Pick<
  ClientConfig,
  "baseURL" | "apiKey" | "timeout"
>;
export type ApiKeyAction = "read" | "write";

export interface CollectionPermissions {
  read: boolean;
  write: boolean;
  select_fields?: string[] | null;
  exclude_fields?: string[] | null;
  writable_fields?: string[] | null;
  [field: string]: unknown;
}

export interface ApiKeyPermissions {
  is_admin: boolean;
  collection_permissions: { [collection: string]: CollectionPermissions };
  [field: string]: unknown;
}

/** Contains a secret key; store securely and never log the returned object. */
export interface ApiKey extends ApiKeyPermissions {
  key: string;
  label?: string | null;
  description?: string | null;
  created_at?: number | null;
  expires_at?: number | null;
  /** Registration succeeded locally but may not have propagated to peers. */
  ripple_warning?: string;
}

export interface CreateApiKeyRequest {
  label?: string;
  description?: string;
  is_admin?: boolean;
  /** Registration accepts action arrays, not field restrictions. */
  collection_permissions?: { [collection: string]: ApiKeyAction[] };
}

export interface UpdateApiKeyCollectionPermissionsRequest {
  api_key: string;
  collection: string;
  read: boolean;
  write: boolean;
}

function invalid(): never {
  throw new Error("Invalid admin response");
}
function object(value: unknown): value is { [field: string]: unknown } {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}
function permissions(value: unknown): CollectionPermissions {
  if (Array.isArray(value)) {
    if (!value.every((action) => action === "read" || action === "write"))
      invalid();
    return { read: value.includes("read"), write: value.includes("write") };
  }
  if (
    !object(value) ||
    typeof value.read !== "boolean" ||
    typeof value.write !== "boolean"
  )
    invalid();
  for (const field of ["select_fields", "exclude_fields", "writable_fields"]) {
    const restriction = value[field];
    if (
      restriction !== undefined &&
      restriction !== null &&
      (!Array.isArray(restriction) ||
        !restriction.every((item) => typeof item === "string"))
    )
      invalid();
  }
  return { ...value } as CollectionPermissions;
}
function parsePermissions(value: unknown): ApiKeyPermissions {
  if (
    !object(value) ||
    typeof value.is_admin !== "boolean" ||
    !object(value.collection_permissions)
  )
    invalid();
  return {
    ...value,
    is_admin: value.is_admin,
    collection_permissions: Object.fromEntries(
      Object.entries(value.collection_permissions).map(([name, grant]) => [
        name,
        permissions(grant),
      ]),
    ),
  };
}
function parseKey(value: unknown): ApiKey {
  const result = parsePermissions(value);
  if (typeof result.key !== "string" || !result.key) invalid();
  if (
    result.ripple_warning !== undefined &&
    typeof result.ripple_warning !== "string"
  )
    invalid();
  for (const field of ["label", "description"]) {
    if (
      result[field] !== undefined &&
      result[field] !== null &&
      typeof result[field] !== "string"
    )
      invalid();
  }
  for (const field of ["created_at", "expires_at"]) {
    if (
      result[field] !== undefined &&
      result[field] !== null &&
      !Number.isSafeInteger(result[field])
    )
      invalid();
  }
  return result as ApiKey;
}

// Only this unexported adapter can access the protected transport. The public
// admin client exposes the five supported operations, never arbitrary routes.
class AdminTransport extends EkoDBClient {
  protected override redactErrors = true;
  async send<T>(method: string, path: string, body?: unknown): Promise<T> {
    await this.getToken();
    return this.makeRequest<T>(method, path, body, 0, true);
  }
}

export class EkoDBAdminClient {
  private readonly transport: AdminTransport;
  private readonly apiKey: string;

  constructor(config: AdminClientConfig) {
    this.apiKey = config.apiKey;
    // No automatic retries after ambiguous failures, including registration.
    this.transport = new AdminTransport({ ...config, shouldRetry: false });
  }

  async createApiKey(request: CreateApiKeyRequest = {}): Promise<ApiKey> {
    if (request.is_admin !== undefined && typeof request.is_admin !== "boolean")
      throw new Error("Invalid admin key request");
    const grants = request.collection_permissions ?? {};
    if (
      !object(grants) ||
      !Object.values(grants).every(
        (actions) =>
          Array.isArray(actions) &&
          actions.every((action) => action === "read" || action === "write"),
      )
    )
      throw new Error("Registration requires read/write action arrays");
    return parseKey(
      await this.transport.send("POST", "/api/auth/register", {
        api_key: this.apiKey,
        label: request.label,
        description: request.description,
        is_admin: request.is_admin === true,
        collection_permissions: grants,
      }),
    );
  }

  async listApiKeys(): Promise<ApiKey[]> {
    const result = await this.transport.send<unknown>("GET", "/api/auth/keys");
    if (!Array.isArray(result)) invalid();
    return result.map(parseKey);
  }

  async revokeApiKey(apiKey: string): Promise<void> {
    if (!apiKey || typeof apiKey !== "string")
      throw new Error("An API key is required");
    let encoded: string;
    try {
      encoded = encodeURIComponent(apiKey);
    } catch {
      throw new Error("Invalid API key encoding");
    }
    await this.transport.send("DELETE", `/api/auth/keys/${encoded}`);
  }

  async getPermissions(): Promise<ApiKeyPermissions> {
    return parsePermissions(
      await this.transport.send("GET", "/api/auth/permissions"),
    );
  }

  async updateApiKeyCollectionPermissions(
    request: UpdateApiKeyCollectionPermissionsRequest,
  ): Promise<ApiKey> {
    if (
      ["select_fields", "exclude_fields", "writable_fields"].some(
        (field) => field in request,
      )
    )
      throw new Error(
        "The server does not support field restrictions on key permission updates",
      );
    if (
      typeof request.api_key !== "string" ||
      !request.api_key ||
      typeof request.collection !== "string" ||
      !request.collection ||
      typeof request.read !== "boolean" ||
      typeof request.write !== "boolean"
    )
      throw new Error("Invalid collection permission update");
    return parseKey(
      await this.transport.send("PUT", "/api/auth/permissions", {
        api_key: request.api_key,
        collection: request.collection,
        read: request.read,
        write: request.write,
      }),
    );
  }
}
