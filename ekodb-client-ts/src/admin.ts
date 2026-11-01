/** API-key administration. Import from @ekodb/ekodb-client/admin. */
import { EkoDBClient, type ClientConfig } from "./client";
import { MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS } from "./admin-version";
export {
  MIN_SERVER_VERSION_FOR_PERMISSION_EDITS,
  MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS,
} from "./admin-version";

const permissionEditRequirement =
  "Permission updates need a newer ekoDB server because earlier servers remove a grant's field restrictions when it is updated";

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
export interface KvPermission {
  prefix: string;
  read: boolean;
  write: boolean;
  [field: string]: unknown;
}
export interface KvGrant {
  prefix?: string;
  read: boolean;
  write: boolean;
  [field: string]: unknown;
}
interface PermissionSet {
  is_admin: boolean;
  collection_permissions: { [collection: string]: CollectionPermissions };
  [field: string]: unknown;
}
export interface ApiKeyPermissions extends PermissionSet {
  kv_permissions: KvPermission[];
}
/** Contains a secret key; store securely and never log the returned object. */
export interface ApiKey extends PermissionSet {
  kv_permissions?: KvPermission[] | null;
  /** Absent on legacy server responses. Never derive this from the secret. */
  id?: string;
  key: string;
  label?: string | null;
  description?: string | null;
  created_at?: number | null;
  expires_at?: number | null;
  ripple_warning?: string;
}
export interface CreateApiKeyRequest {
  label?: string;
  description?: string;
  is_admin?: boolean;
  /** Registration accepts action arrays, not field restrictions. */
  collection_permissions?: { [collection: string]: ApiKeyAction[] };
}
export interface PutApiKeyCollectionPermissionsRequest {
  key_id: string;
  collection: string;
  read: boolean;
  write: boolean;
  select_fields?: string[] | null | undefined;
  exclude_fields?: string[] | null | undefined;
  writable_fields?: string[] | null | undefined;
}
export interface PatchApiKeyCollectionPermissionsRequest {
  key_id: string;
  collection: string;
  read?: boolean;
  write?: boolean;
  /** Omitted/undefined keeps; null clears; an array (including []) replaces. */
  select_fields?: string[] | null | undefined;
  exclude_fields?: string[] | null | undefined;
  writable_fields?: string[] | null | undefined;
}
export type UpdateApiKeyCollectionPermissionsRequest =
  PutApiKeyCollectionPermissionsRequest;
export interface LegacyUpdateApiKeyCollectionPermissionsRequest {
  api_key: string;
  collection: string;
  read: boolean;
  write: boolean;
  select_fields?: string[] | null | undefined;
  exclude_fields?: string[] | null | undefined;
  writable_fields?: string[] | null | undefined;
}
export interface PutApiKeyKvPermissionsRequest {
  key_id: string;
  prefix: string;
  read: boolean;
  write: boolean;
}
export interface PatchApiKeyKvPermissionsRequest {
  key_id: string;
  prefix: string;
  read?: boolean;
  write?: boolean;
}
export interface PermissionChange<T> {
  before: T | null;
  after: T | null;
  [field: string]: unknown;
}
export type CollectionPermissionChange =
  PermissionChange<CollectionPermissions>;
export type KvPermissionChange = PermissionChange<KvGrant>;

function invalid(): never {
  throw new Error("Invalid admin response");
}
function object(value: unknown): value is { [field: string]: unknown } {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}
const limitFields = [
  "select_fields",
  "exclude_fields",
  "writable_fields",
] as const;
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
  for (const field of limitFields) {
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
function kvGrant(value: unknown, requirePrefix = false): KvGrant {
  if (
    !object(value) ||
    typeof value.read !== "boolean" ||
    typeof value.write !== "boolean" ||
    (requirePrefix && (typeof value.prefix !== "string" || !value.prefix)) ||
    (value.prefix !== undefined && typeof value.prefix !== "string")
  )
    invalid();
  return { ...value } as KvGrant;
}
function parsePermissions(value: unknown, modern = false): ApiKeyPermissions {
  if (!object(value)) invalid();
  const admin = modern ? value.admin : value.is_admin;
  const collections = modern ? value.collections : value.collection_permissions;
  const kv = modern ? value.kv : value.kv_permissions;
  if (typeof admin !== "boolean" || !object(collections)) invalid();
  if ((modern || (kv !== undefined && kv !== null)) && !Array.isArray(kv))
    invalid();
  return {
    ...value,
    is_admin: admin,
    collection_permissions: Object.fromEntries(
      Object.entries(collections).map(([name, grant]) => [
        name,
        permissions(grant),
      ]),
    ),
    kv_permissions: Array.isArray(kv)
      ? kv.map((grant) => kvGrant(grant, true) as KvPermission)
      : [],
  };
}
function parseKey(value: unknown): ApiKey {
  const result = parsePermissions(value) as ApiKey;
  if (object(value) && value.kv_permissions === null)
    result.kv_permissions = null;
  else if (object(value) && value.kv_permissions === undefined)
    delete result.kv_permissions;
  if (typeof result.key !== "string" || !result.key) invalid();
  if (
    result.id !== undefined &&
    (typeof result.id !== "string" || !/^[0-9a-f]{32}$/.test(result.id))
  )
    invalid();
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
function change<T>(
  value: unknown,
  parse: (entry: unknown) => T,
): PermissionChange<T> {
  if (!object(value) || !("before" in value) || !("after" in value)) invalid();
  return {
    ...value,
    before: value.before === null ? null : parse(value.before),
    after: value.after === null ? null : parse(value.after),
  };
}
function segment(value: unknown): string {
  if (
    typeof value !== "string" ||
    !value.length ||
    value === "." ||
    value === ".." ||
    /[\p{Cc}\p{Cf}]/u.test(value)
  )
    throw new Error("Invalid permission resource name");
  return encodedPermissionValue(value);
}
function encodedPermissionValue(value: unknown): string {
  if (typeof value !== "string")
    throw new Error("Invalid permission resource name");
  try {
    return encodeURIComponent(value).replace(
      /[!'()*]/g,
      (character) => `%${character.charCodeAt(0).toString(16).toUpperCase()}`,
    );
  } catch {
    throw new Error("Invalid permission resource name");
  }
}
function resource(keyId: string): string {
  if (
    typeof keyId !== "string" ||
    (keyId !== "me" && !/^[0-9a-f]{32}$/.test(keyId))
  )
    throw new Error("A key id or me is required; never pass a key value");
  return `/api/auth/keys/${keyId}/permissions`;
}
function editBody(
  request: unknown,
  facet: "collections" | "kv",
  patch: boolean,
  legacy = false,
): { [field: string]: unknown } {
  const noun = facet === "collections" ? "collection" : "prefix";
  const error =
    facet === "collections"
      ? "Invalid collection permission update"
      : "Invalid KV permission update";
  if (!object(request)) throw new Error(error);
  const idField = legacy ? "api_key" : "key_id";
  if (legacy) {
    if (typeof request.api_key !== "string" || !request.api_key.trim())
      throw new Error(error);
  } else resource(request.key_id as string);
  try {
    if (
      legacy &&
      (typeof request[noun] !== "string" || !(request[noun] as string).trim())
    )
      throw new Error(error);
    segment(request[noun]);
  } catch {
    throw new Error(error);
  }
  const allowed = [
    idField,
    noun,
    "read",
    "write",
    ...(facet === "collections" ? limitFields : []),
  ];
  if (!legacy && Object.keys(request).some((field) => !allowed.includes(field)))
    throw new Error(error);
  const body: { [field: string]: unknown } = {};
  for (const field of ["read", "write"] as const) {
    if (patch && !Object.prototype.hasOwnProperty.call(request, field))
      continue;
    if (request[field] === undefined && patch) continue;
    if (typeof request[field] !== "boolean") throw new Error(error);
    body[field] = request[field];
  }
  if (facet === "kv" && !patch && !body.read && !body.write)
    throw new Error(error);
  if (facet === "collections")
    for (const field of limitFields) {
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
        throw new Error(error);
      body[field] = value;
    }
  return body;
}

// Only this unexported adapter accesses the protected transport.
class AdminTransport extends EkoDBClient {
  protected override redactErrors = true;
  async send<T>(
    method: string,
    path: string,
    body?: unknown,
    readDeleteResponse = false,
  ): Promise<T> {
    await this.getToken();
    return this.makeRequest<T>(method, path, body, 0, true, readDeleteResponse);
  }
}

export class EkoDBAdminClient {
  private readonly transport: AdminTransport;
  private readonly apiKey: string;
  private serverVersion?: Promise<unknown>;
  private permissionRoutes?: Promise<boolean>;
  private probePermissions?: ApiKeyPermissions;

  constructor(config: AdminClientConfig) {
    this.apiKey = config.apiKey;
    this.transport = new AdminTransport({ ...config, shouldRetry: false });
  }
  /** Detect actual routes, including prerelease servers reporting an older version. */
  async usesKeyPermissionRoutes(): Promise<boolean> {
    if (!this.permissionRoutes) {
      this.permissionRoutes = (async () => {
        try {
          this.probePermissions = parsePermissions(
            await this.transport.send("GET", resource("me")),
            true,
          );
          return true;
        } catch (error) {
          if (
            error instanceof Error &&
            /^Admin request failed \(HTTP (404|405)\)$/.test(error.message)
          )
            return false;
          throw error;
        }
      })().catch((error) => {
        this.permissionRoutes = undefined;
        throw error;
      });
    }
    return this.permissionRoutes;
  }
  private async requireKeyPermissionRoutes(): Promise<void> {
    if (!(await this.usesKeyPermissionRoutes()))
      throw new Error(
        "This operation requires per-key permission routes (server 0.80.0)",
      );
  }
  private async requireLegacyPermissionEdits(): Promise<void> {
    const minimum = releaseParts(
      MIN_LEGACY_SERVER_VERSION_FOR_PERMISSION_EDITS,
    );
    if (!minimum) throw new Error(permissionEditRequirement);
    if (await this.usesKeyPermissionRoutes())
      throw new Error("Use key-id permission methods for this server");
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
  /** Prefer the server-generated id; legacy servers also accept a key value. */
  async revokeApiKey(keyIdOrValue: string): Promise<void> {
    if (!keyIdOrValue || typeof keyIdOrValue !== "string")
      throw new Error("An API key is required");
    let encoded: string;
    try {
      encoded = encodeURIComponent(keyIdOrValue);
    } catch {
      throw new Error("Invalid API key encoding");
    }
    await this.transport.send("DELETE", `/api/auth/keys/${encoded}`);
  }
  async getPermissions(): Promise<ApiKeyPermissions> {
    const first = !this.permissionRoutes;
    if (await this.usesKeyPermissionRoutes()) {
      if (first && this.probePermissions) return this.probePermissions;
      return parsePermissions(
        await this.transport.send("GET", resource("me")),
        true,
      );
    }
    return parsePermissions(
      await this.transport.send("GET", "/api/auth/permissions"),
    );
  }
  async getApiKeyPermissions(keyId: string): Promise<ApiKeyPermissions> {
    resource(keyId);
    if (keyId === "me") return this.getPermissions();
    await this.requireKeyPermissionRoutes();
    return parsePermissions(
      await this.transport.send("GET", resource(keyId)),
      true,
    );
  }
  async listApiKeyCollectionPermissions(
    keyId: string,
  ): Promise<{ [collection: string]: CollectionPermissions }> {
    const path = `${resource(keyId)}/collections`;
    await this.requireKeyPermissionRoutes();
    const result = await this.transport.send<unknown>("GET", path);
    if (!object(result)) invalid();
    return Object.fromEntries(
      Object.entries(result).map(([name, grant]) => [name, permissions(grant)]),
    );
  }
  async getApiKeyCollectionPermissions(
    keyId: string,
    collection: string,
  ): Promise<CollectionPermissions> {
    const path = `${resource(keyId)}/collections/${segment(collection)}`;
    await this.requireKeyPermissionRoutes();
    return permissions(await this.transport.send("GET", path));
  }
  async listApiKeyKvPermissions(
    keyId: string,
    prefix?: string,
  ): Promise<KvPermission[]> {
    let path = `${resource(keyId)}/kv`;
    if (prefix !== undefined)
      path += `?prefix=${encodedPermissionValue(prefix)}`;
    await this.requireKeyPermissionRoutes();
    const result = await this.transport.send<unknown>("GET", path);
    if (!Array.isArray(result)) invalid();
    return result.map((grant) => kvGrant(grant, true) as KvPermission);
  }
  async getApiKeyKvPermissions(
    keyId: string,
    prefix: string,
  ): Promise<KvGrant> {
    const path = `${resource(keyId)}/kv/${segment(prefix)}`;
    await this.requireKeyPermissionRoutes();
    return kvGrant(await this.transport.send("GET", path));
  }
  async updateApiKeyCollectionPermissions(
    request: UpdateApiKeyCollectionPermissionsRequest,
  ): Promise<CollectionPermissionChange> {
    return this.patchApiKeyCollectionPermissions(request);
  }
  async putApiKeyCollectionPermissions(
    request: PutApiKeyCollectionPermissionsRequest,
  ): Promise<CollectionPermissionChange> {
    const body = editBody(request, "collections", false);
    const path = `${resource(request.key_id)}/collections/${segment(request.collection)}`;
    await this.requireKeyPermissionRoutes();
    return change(await this.transport.send("PUT", path, body), permissions);
  }
  async patchApiKeyCollectionPermissions(
    request: PatchApiKeyCollectionPermissionsRequest,
  ): Promise<CollectionPermissionChange> {
    const body = editBody(request, "collections", true);
    const path = `${resource(request.key_id)}/collections/${segment(request.collection)}`;
    await this.requireKeyPermissionRoutes();
    return change(await this.transport.send("PATCH", path, body), permissions);
  }
  async deleteApiKeyCollectionPermissions(
    keyId: string,
    collection: string,
  ): Promise<CollectionPermissionChange> {
    const path = `${resource(keyId)}/collections/${segment(collection)}`;
    await this.requireKeyPermissionRoutes();
    return change(
      await this.transport.send("DELETE", path, undefined, true),
      permissions,
    );
  }
  async putApiKeyKvPermissions(
    request: PutApiKeyKvPermissionsRequest,
  ): Promise<KvPermissionChange> {
    const body = editBody(request, "kv", false);
    const path = `${resource(request.key_id)}/kv/${segment(request.prefix)}`;
    await this.requireKeyPermissionRoutes();
    return change(await this.transport.send("PUT", path, body), kvGrant);
  }
  async patchApiKeyKvPermissions(
    request: PatchApiKeyKvPermissionsRequest,
  ): Promise<KvPermissionChange> {
    const body = editBody(request, "kv", true);
    const path = `${resource(request.key_id)}/kv/${segment(request.prefix)}`;
    await this.requireKeyPermissionRoutes();
    return change(await this.transport.send("PATCH", path, body), kvGrant);
  }
  async deleteApiKeyKvPermissions(
    keyId: string,
    prefix: string,
  ): Promise<KvPermissionChange> {
    const path = `${resource(keyId)}/kv/${segment(prefix)}`;
    await this.requireKeyPermissionRoutes();
    return change(
      await this.transport.send("DELETE", path, undefined, true),
      kvGrant,
    );
  }
  /** Only for detected legacy routes on servers >= 0.79.2. */
  async legacyUpdateApiKeyCollectionPermissions(
    request: LegacyUpdateApiKeyCollectionPermissionsRequest,
  ): Promise<ApiKey> {
    const body = editBody(request, "collections", false, true);
    await this.requireLegacyPermissionEdits();
    return parseKey(
      await this.transport.send("PUT", "/api/auth/permissions", {
        api_key: request.api_key,
        collection: request.collection,
        ...body,
      }),
    );
  }
}
