# 16 — Security and privacy

## Implemented
- Passwords hashed with Argon2; plaintext passwords are never stored.
- JWT access token + refresh token; refresh tokens rotate and old tokens are rejected.
- JWT carries `user_id` and `household_id`; every core function receives and filters by the household.
- Household membership/role checked for administrative household actions.
- AI provider keys are read from server environment only and never sent to the client.
- AI mutations are proposals requiring human approval; read-only tools still respect household scope.
- Price review changes are written to an audit log with actor and timestamp.
- Raw merchant post text is preserved for evidence rather than overwritten.

## Development caveats (must fix for production)
- `BELHANA_JWT_SECRET` defaults to `dev-secret-change-me`; set a strong random value in deployment.
- CORS currently allows any origin (local development convenience); restrict to known web origins.
- No rate limiting/account lockout, email verification, password reset or GDPR export/delete flow yet.
- No HTTPS termination in-process; deploy behind TLS.
- SQLite file permissions and backup encryption are the operator's responsibility.

## Privacy posture
Household food data, family member details and uploaded merchant text are private to the household.
No telemetry is collected. AI requests send only the tool payloads needed for the conversation to the
configured provider, never passwords or JWTs.
