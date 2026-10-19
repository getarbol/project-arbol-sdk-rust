# Security

Report a vulnerability privately to the repository's owner, through GitHub's private vulnerability
reporting (**Security → Report a vulnerability**). Do not open a public issue for it.

What this repository holds, and what it never holds:

- **Here:** the `arbol-sdk` crate: its transport, and the module a Cloudflare Forge plugin generates from
  the platform's public OpenAPI.
- **Never here:** credentials, tokens, API keys or client secrets (CI reads them from GitHub secrets,
  each only in the step that needs it), and patient or customer data.

Authority is decided by each service, never by the client: every call reaches its owner with the
caller's own credential, and the owner verifies it. A method in the SDK is not a grant.
