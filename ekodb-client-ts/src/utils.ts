/**
 * Utility functions for working with ekoDB records
 */

/**
 * Extract the value from an ekoDB field object.
 * ekoDB returns fields as { type: string, value: any } objects.
 * This helper safely extracts the value or returns the input if it's not a field object.
 *
 * @param field - The field value from ekoDB (may be { type, value } or a plain value)
 * @returns The extracted value
 *
 * @example
 * ```typescript
 * const user = await client.findByID('users', userId);
 * const email = getValue(user.email); // Extracts string from { type: 'String', value: 'user@example.com' }
 * const age = getValue(user.age);     // Extracts number from { type: 'Integer', value: 25 }
 * ```
 */
export function getValue<T = any>(field: any): T {
  if (field && typeof field === "object" && "value" in field) {
    return field.value as T;
  }
  return field as T;
}

/**
 * Extract values from multiple fields in a record.
 * Useful for processing entire records returned from ekoDB.
 *
 * @param record - The record object from ekoDB
 * @param fields - Array of field names to extract values from
 * @returns Object with extracted values
 *
 * @example
 * ```typescript
 * const user = await client.findByID('users', userId);
 * const { email, first_name, status } = getValues(user, ['email', 'first_name', 'status']);
 * ```
 */
export function getValues<T extends Record<string, any>>(
  record: any,
  fields: (keyof T)[],
): Partial<T> {
  const result: any = {};
  for (const field of fields) {
    result[field] = getValue(record[field]);
  }
  return result;
}

/**
 * Extract a Date value from an ekoDB DateTime field
 */
export function getDateTimeValue(field: any): Date | null {
  const val = getValue(field);
  if (val instanceof Date) return val;
  if (typeof val === "string") {
    const date = new Date(val);
    return isNaN(date.getTime()) ? null : date;
  }
  return null;
}

/**
 * Extract a UUID string from an ekoDB UUID field
 */
export function getUUIDValue(field: any): string | null {
  const val = getValue(field);
  return typeof val === "string" ? val : null;
}

/**
 * Extract a number from an ekoDB Decimal field
 */
export function getDecimalValue(field: any): number | null {
  const val = getValue(field);
  if (typeof val === "number") return val;
  if (typeof val === "string") {
    const num = parseFloat(val);
    return isNaN(num) ? null : num;
  }
  return null;
}

/**
 * Extract a number (milliseconds) from an ekoDB Duration field
 */
export function getDurationValue(field: any): number | null {
  const val = getValue(field);
  if (typeof val === "number") return val;
  if (typeof val === "object" && val.secs !== undefined) {
    return val.secs * 1000 + val.nanos / 1_000_000;
  }
  return null;
}

/**
 * Extract a Uint8Array from an ekoDB Bytes field
 */
export function getBytesValue(field: any): Uint8Array | null {
  const val = getValue(field);
  if (val instanceof Uint8Array) return val;
  if (Array.isArray(val)) return new Uint8Array(val);
  if (typeof val === "string") {
    // Assume base64 encoded
    try {
      const binary = atob(val);
      const bytes = new Uint8Array(binary.length);
      for (let i = 0; i < binary.length; i++) {
        bytes[i] = binary.charCodeAt(i);
      }
      return bytes;
    } catch {
      return null;
    }
  }
  return null;
}

/**
 * Extract a Uint8Array from an ekoDB Binary field
 */
export function getBinaryValue(field: any): Uint8Array | null {
  return getBytesValue(field);
}

/**
 * Extract an array from an ekoDB Array field
 */
export function getArrayValue<T = any>(field: any): T[] | null {
  const val = getValue(field);
  return Array.isArray(val) ? val : null;
}

/**
 * Extract an array from an ekoDB Set field
 */
export function getSetValue<T = any>(field: any): T[] | null {
  const val = getValue(field);
  return Array.isArray(val) ? val : null;
}

/**
 * Extract an array from an ekoDB Vector field
 */
export function getVectorValue(field: any): number[] | null {
  const val = getValue(field);
  if (Array.isArray(val)) {
    return val
      .map((v) => (typeof v === "number" ? v : parseFloat(v)))
      .filter((v) => !isNaN(v));
  }
  return null;
}

/**
 * Extract an object from an ekoDB Object field
 */
export function getObjectValue<T = any>(field: any): T | null {
  const val = getValue(field);
  return val && typeof val === "object" && !Array.isArray(val) ? val : null;
}

/**
 * Transform an entire record by extracting all field values.
 * Preserves the 'id' field and extracts values from all other fields.
 *
 * @param record - The record object from ekoDB
 * @returns Object with all field values extracted
 *
 * @example
 * ```typescript
 * const user = await client.findByID('users', userId);
 * const plainUser = extractRecord(user);
 * // { id: '123', email: 'user@example.com', first_name: 'John', ... }
 * ```
 */
export function extractRecord<T extends Record<string, any>>(record: any): T {
  if (!record || typeof record !== "object") {
    return record;
  }

  const result: any = {};
  for (const [key, value] of Object.entries(record)) {
    if (key === "id") {
      result[key] = value;
    } else {
      result[key] = getValue(value);
    }
  }
  return result as T;
}

// ============================================================================
// Wrapped Type Builders
// ============================================================================
// These functions create wrapped type objects for sending to ekoDB.
// Use these when inserting/updating records with special field types.
//
// Example:
//   await client.insert("orders", {
//     id: Field.uuid("550e8400-e29b-41d4-a716-446655440000"),
//     total: Field.decimal("99.99"),
//     created_at: Field.dateTime(new Date()),
//     tags: Field.set(["sale", "featured"]),
//   });

export interface WrappedFieldValue {
  type: string;
  value: unknown;
}

/**
 * Field builders for creating wrapped type values to send to ekoDB.
 * These are the inverse of the getValue* extraction functions.
 */
export const Field = {
  /**
   * Create a UUID field value
   * @param value - UUID string (e.g., "550e8400-e29b-41d4-a716-446655440000")
   */
  uuid: (value: string): WrappedFieldValue => ({
    type: "UUID",
    value,
  }),

  /**
   * Create a Decimal field value for precise numeric values
   * @param value - Decimal as string (e.g., "99.99") to preserve precision
   */
  decimal: (value: string): WrappedFieldValue => ({
    type: "Decimal",
    value,
  }),

  /**
   * Create a DateTime field value
   * @param value - Date object or RFC3339 string
   */
  dateTime: (value: Date | string): WrappedFieldValue => ({
    type: "DateTime",
    value: value instanceof Date ? value.toISOString() : value,
  }),

  /**
   * Create a Duration field value
   * @param milliseconds - Duration in milliseconds
   */
  duration: (milliseconds: number): WrappedFieldValue => ({
    type: "Duration",
    value: milliseconds,
  }),

  /**
   * Create a Number field value (flexible numeric type)
   * @param value - Integer or float
   */
  number: (value: number): WrappedFieldValue => ({
    type: "Number",
    value,
  }),

  /**
   * Create a Set field value (unique elements)
   * @param values - Array of values (duplicates will be removed by server)
   */
  set: <T>(values: T[]): WrappedFieldValue => ({
    type: "Set",
    value: values,
  }),

  /**
   * Create a Vector field value (for embeddings/similarity search)
   * @param values - Array of numbers representing the vector
   */
  vector: (values: number[]): WrappedFieldValue => ({
    type: "Vector",
    value: values,
  }),

  /**
   * Create a Binary field value
   * @param value - Base64 encoded string or Uint8Array
   */
  binary: (value: string | Uint8Array): WrappedFieldValue => ({
    type: "Binary",
    value:
      value instanceof Uint8Array ? btoa(String.fromCharCode(...value)) : value,
  }),

  /**
   * Create a Bytes field value
   * @param value - Base64 encoded string or Uint8Array
   */
  bytes: (value: string | Uint8Array): WrappedFieldValue => ({
    type: "Bytes",
    value:
      value instanceof Uint8Array ? btoa(String.fromCharCode(...value)) : value,
  }),

  /**
   * Create an Array field value
   * @param values - Array of values
   */
  array: <T>(values: T[]): WrappedFieldValue => ({
    type: "Array",
    value: values,
  }),

  /**
   * Create an Object field value
   * @param value - Object/map of key-value pairs
   */
  object: (value: Record<string, unknown>): WrappedFieldValue => ({
    type: "Object",
    value,
  }),

  /**
   * Create a String field value (explicit wrapping)
   * @param value - String value
   */
  string: (value: string): WrappedFieldValue => ({
    type: "String",
    value,
  }),

  /**
   * Create an Integer field value (explicit wrapping)
   * @param value - Integer value
   */
  integer: (value: number): WrappedFieldValue => ({
    type: "Integer",
    value: Math.floor(value),
  }),

  /**
   * Create a Float field value (explicit wrapping)
   * @param value - Float value
   */
  float: (value: number): WrappedFieldValue => ({
    type: "Float",
    value,
  }),

  /**
   * Create a Boolean field value (explicit wrapping)
   * @param value - Boolean value
   */
  boolean: (value: boolean): WrappedFieldValue => ({
    type: "Boolean",
    value,
  }),
};
