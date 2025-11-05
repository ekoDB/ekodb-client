# HTTP Compression Support

## Overview

ekoDB uses **content-negotiation based compression** to optimize network
performance while maintaining broad API compatibility.

## How It Works

### Server Behavior

The server checks the `Accept-Encoding` header in client requests:

- **Client sends `Accept-Encoding: gzip`** → Server compresses response with
  gzip
- **No Accept-Encoding header** → Server sends plain JSON (for curl, Postman,
  simple clients)

### Implementation

Server-side compression logic in `ekodb_server/src/compression.rs`:

```rust
pub fn maybe_compress_response(data: Vec<u8>, headers: &HeaderMap) -> (Vec<u8>, bool) {
    if accepts_gzip(headers) {
        match compress_gzip(&data) {
            Ok(compressed) => (compressed, true),
            Err(_) => (data, false), // Fallback to uncompressed
        }
    } else {
        (data, false)
    }
}
```

## Client Library Support

### ✅ Rust Client (`ekodb_client`)

**Status**: Fully supported with `reqwest` gzip feature

```toml
reqwest = { version = "0.12", features = ["json", "rustls-tls", "gzip"] }
```

- Automatically sends `Accept-Encoding: gzip` header
- Automatically decompresses gzip responses
- No code changes needed in applications

### ✅ Kotlin Client (`ekodb-client-kt`)

**Status**: Fully supported with Ktor ContentEncoding plugin

```kotlin
install(ContentEncoding) {
    gzip()
    deflate()
}
```

- Sends `Accept-Encoding: gzip, deflate` header
- Automatically decompresses responses
- No code changes needed in applications

### ✅ TypeScript/JavaScript Client (`ekodb-client-ts`)

**Status**: Automatically supported by modern `fetch` API

- Browser and Node.js `fetch` handle gzip decompression automatically
- Sends `Accept-Encoding` header automatically
- No configuration needed

### ✅ Python Client (`ekodb-client-py`)

**Status**: Automatically supported via Rust bindings

- Uses Rust client underneath (PyO3 bindings)
- Inherits gzip support from Rust client
- No configuration needed

### ✅ Go Client (`ekodb-client-go`)

**Status**: Automatically supported by Go's `net/http` package

- Go's default HTTP client/transport handles gzip automatically
- Sends `Accept-Encoding` header automatically
- Automatically decompresses gzip responses
- No configuration needed

## Testing Compression

### With Compression (Client Libraries)

All official clients automatically request and handle compression:

```bash
# Request will include: Accept-Encoding: gzip
# Response will include: Content-Encoding: gzip
```

### Without Compression (curl, Postman)

Simple HTTP clients work without any special configuration:

```bash
curl https://your-ekodb.com/api/health
# Returns plain JSON (no compression)
```

### Force Compression with curl

```bash
curl -H "Accept-Encoding: gzip" --compressed https://your-ekodb.com/api/health
# Returns compressed response, curl decompresses it
```

## Performance Benefits

### Compression Ratios

Typical JSON responses see **40-70% size reduction**:

- Uncompressed: 10 KB
- Gzip compressed: 3-6 KB
- **Bandwidth saved**: 40-70%

### When Compression Helps Most

- ✅ Large query results (arrays of records)
- ✅ Batch operations with many records
- ✅ Search results with text fields
- ✅ Schema and metadata responses

### When Compression Helps Less

- ❌ Single small records (<1 KB)
- ❌ Already compressed data (binary fields)
- ❌ Very fast local networks

## API Compatibility

### ✅ Works With

- Official ekoDB client libraries (Rust, Kotlin, TypeScript, Python)
- Modern web browsers
- Node.js applications
- HTTP clients with gzip support (curl --compressed, HTTPie, etc.)

### ✅ Also Works With

- curl (plain)
- Postman
- Insomnia
- Any HTTP client without compression support
- Legacy applications

**No breaking changes** - compression is purely optional and negotiated
per-request.

## Migration Guide

### For Application Developers

**No changes needed!** All official clients automatically handle compression:

```rust
// Rust - No changes needed
let client = EkoDBClient::new("https://api.ekodb.net", "api_key_xxx").await?;
let records = client.find("users", None).await?; // Automatically compressed

```

```kotlin
// Kotlin - No changes needed
val client = EkoDBClient.builder()
    .baseUrl("https://api.ekodb.net")
    .apiKey("api_key_xxx")
    .build()
val records = client.find("users") // Automatically compressed
```

```typescript
// TypeScript - No changes needed
const client = new EkoDBClient({
  baseURL: "https://api.ekodb.net",
  apiKey: "api_key_xxx",
});
const records = await client.find("users"); // Automatically compressed
```

```python
# Python - No changes needed
client = EkoDBClient("https://api.ekodb.net", "api_key_xxx")
records = client.find("users")  # Automatically compressed
```

```go
// Go - No changes needed
client, err := ekodb.NewClient("https://api.ekodb.net", "api_key_xxx")
if err != nil {
    log.Fatal(err)
}
records, err := client.Find("users", nil) // Automatically compressed
```

### For Manual HTTP Requests

If you're using a custom HTTP client, ensure it supports gzip decompression or
don't send the `Accept-Encoding` header:

```bash
# Option 1: Don't request compression (plain JSON)
curl https://api.ekodb.net/api/health

# Option 2: Request compression and decompress
curl -H "Accept-Encoding: gzip" --compressed https://api.ekodb.net/api/health
```

## Troubleshooting

### Issue: Garbled Response (Binary Data)

**Symptom**: Response looks like `���\u001F�\u0008\u0000...`

**Cause**: Your HTTP client sent `Accept-Encoding: gzip` but isn't decompressing
the response

**Solution**:

- Use an official ekoDB client library (recommended)
- Enable gzip decompression in your HTTP client
- Or remove the `Accept-Encoding` header to get plain JSON

### Issue: Slow Performance

**Check**: Is compression enabled?

```bash
# Check response headers
curl -I -H "Accept-Encoding: gzip" https://api.ekodb.net/api/health

# Should see:
Content-Encoding: gzip
```

If not seeing `Content-Encoding: gzip`, your client isn't requesting
compression.

## Implementation Details

### Server-Side

- **Location**: `ekodb_server/src/compression.rs`
- **Algorithm**: gzip (via `flate2` crate)
- **Compression Level**: Default (balance of speed and ratio)
- **Fallback**: Returns uncompressed on compression errors

### Client-Side

- **Rust**: `reqwest` with `gzip` feature
- **Kotlin**: Ktor `ContentEncoding` plugin
- **TypeScript**: Native `fetch` API
- **Go**: Standard `net/http` default transport
- **Python**: Inherits from Rust client

## Future Enhancements

Potential future improvements:

- **Brotli compression**: Higher compression ratios for modern browsers
- **Configurable compression levels**: Trade speed for size
- **Selective compression**: Compression hints per endpoint
- **Compression metrics**: Monitor bandwidth savings

## References

- [RFC 7230 - HTTP/1.1 Message Syntax](https://tools.ietf.org/html/rfc7230#section-3.3.1)
- [MDN: Accept-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Encoding)
- [MDN: Content-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding)
