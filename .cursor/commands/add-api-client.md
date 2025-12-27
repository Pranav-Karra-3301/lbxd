# Add API Client

Create a new external API client for lbxd.

## Required Information

- API name (e.g., "imdb", "rottentomatoes")
- Base URL
- Authentication method (API key, OAuth, none)
- Main endpoints needed

## Implementation Steps

1. **Create module structure**
   ```
   src/
   └── {api_name}/
       └── mod.rs
   ```

2. **Define response types** in `models/mod.rs`
   ```rust
   #[derive(Debug, Deserialize)]
   pub struct ApiResponse {
       // fields
   }
   ```

3. **Implement client**
   ```rust
   pub struct ApiClient {
       client: reqwest::Client,
       base_url: String,
       api_key: Option<String>,
   }
   ```

4. **Add to lib.rs**
   ```rust
   pub mod api_name;
   ```

5. **Integrate with cache** if appropriate

## Safety Requirements

- Timeout on all requests (30s default)
- Proper error handling (no unwrap)
- Rate limiting awareness
- Environment variable for API key override
- HTTPS only
