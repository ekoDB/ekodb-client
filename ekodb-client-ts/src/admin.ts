/** API-key administration. Import from @ekodb/ekodb-client/admin. */
import { EkoDBClient, type ClientConfig } from "./client";
import { MIN_SERVER_VERSION_FOR_PERMISSION_EDITS } from "./admin-version";
export { MIN_SERVER_VERSION_FOR_PERMISSION_EDITS } from "./admin-version";

const permissionEditRequirement =
  "Permission updates need a newer ekoDB server because earlier servers remove a grant's field restrictions when it is updated";

// Accept stable semantic releases only; malformed or prerelease versions fail closed.
function releaseParts(value: unknown): number[] | undefined {
  if (typeof value !== "string") return undefined;
  const match =
    /^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/.exec(
      value,
    );
  if (!match) return undefined;
  const parts = match.slice(1, 4).map(Number);
  return parts.every(Number.isSafeInteger) ? parts : undefined;
}

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
  /** Omitted/undefined keeps the restriction; null clears it; an array replaces it. */
  select_fields?: string[] | null | undefined;
  exclude_fields?: string[] | null | undefined;
  writable_fields?: string[] | null | undefined;
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
  private serverVersion?: Promise<unknown>;

  private async requirePermissionEdits(): Promise<void> {
    const minimum = releaseParts(MIN_SERVER_VERSION_FOR_PERMISSION_EDITS);
    if (!minimum) throw new Error(permissionEditRequirement);
    this.serverVersion ??= this.transport
      .send<unknown>("GET", "/api/health")
      .then((health) => (object(health) ? health.version : undefined))
      .catch(() => undefined);
    const current = releaseParts(await this.serverVersion);
    if (!current) throw new Error(permissionEditRequirement);
    for (let part = 0; part < minimum.length; part++) {
      if (current[part] > minimum[part]) return;
      if (current[part] < minimum[part])
        throw new Error(permissionEditRequirement);
    }
  }

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
      !object(request) ||
      typeof request.api_key !== "string" ||
      !request.api_key.trim() ||
      typeof request.collection !== "string" ||
      !request.collection.trim() ||
      typeof request.read !== "boolean" ||
      typeof request.write !== "boolean"
    )
      throw new Error("Invalid collection permission update");
    const body: UpdateApiKeyCollectionPermissionsRequest = {
      api_key: request.api_key,
      collection: request.collection,
      read: request.read,
      write: request.write,
    };
    for (const field of [
      "select_fields",
      "exclude_fields",
      "writable_fields",
    ] as const) {
      if (!Object.prototype.hasOwnProperty.call(request, field)) continue;
      const value = request[field];
      if (value === undefined) continue;
      if (
        value !== null &&
        (!Array.isArray(value) ||
          !Array.from(value).every(
            (item) => typeof item === "string" && item.trim().length > 0,
          ))
      )
        throw new Error("Invalid collection permission update");
      body[field] = value;
    }
    await this.requirePermissionEdits();
    return parseKey(
      await this.transport.send("PUT", "/api/auth/permissions", body),
    );
  }
}
