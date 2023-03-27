# spin-token-auth

[![Release](https://github.com/jpflueger/spin-token-auth/actions/workflows/release.yml/badge.svg)](https://github.com/jpflueger/spin-token-auth/actions/workflows/release.yml)

A re-usable spin component that performs JWT token authorization

## Configuration

| Name                  | Type                | Source                               | Purpose                                                                          |
| --------------------- | ------------------- | ------------------------------------ | -------------------------------------------------------------------------------- |
| `jwks_uri`            | URI                 | `component.config.jwks_uri`          | URI pointing to a hosted JWKS file like http://example.com/.well-known/jwks.json |
| `audiences`           | Comma-delimited set | `component.config.audiences`         | Will reject tokens whose audience is not in this supplied set.                   |
| `issuers`             | Comma-delimited set | `component.config.issuers`           | Will reject tokens whose issuer is not in this supplied set.                     |
| `max_header_length`   | unsigned 64-bit int | `component.config.max_header_length` | Maximum unsafe, untrusted, unverified JWT header length to accept.               |
| `max_token_length`    | unsigned 64-bit int | `component.config.max_token_length`  | Maximum token length to accept.                                                  |
| `accept_future`       | boolean             | `component.config.accept_future`     | Accept tokens created with a date in the future.                                 |
| `max_validity_secs`   | unsigned 64-bit int | `component.config.max_validity_secs` | Reject tokens created more than `max_validity_secs` seconds ago.                 |
| `time_tolerance_secs` | unsigned 64-bit int | `component.config.time_tolerance`    | How much clock drift to tolerate when verifying token timestamps.                |

## Auth0 Test

1. Create an Auth0 account
2. Select an Auth0 environment
3. Create an API (remember URL for audience)
4. Go to API page and select the "Machine to Machine Applicatons" tab
5. Locally run `cp .envrc.template .envrc`
6. Locally open `.envrc` and substitute values for domain, client_id, client_secret and audience from the opened web page
7. Locally run the test script `./scripts/test-auth0.sh`

## Todos

-   [ ] Can we use oxide-auth?
-   [ ] Config parsing
    -   [ ] Should we log when parsing fails or panic?
-   [ ] Better approach to logging / error response
-   [ ] Allow request to override some verification options
-   [ ] Cache the JWKS in key-value
-   [ ] Implement custom claims
-   [ ] Design for Verification API
-   [ ] Design for Issuance API
