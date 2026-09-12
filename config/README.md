# Codex relay configuration

The relay-supplied credentials JSON is represented by the `OPENAI_API_KEY`
environment variable. `requires_openai_auth = true` makes the pinned Codex
release read that variable for the OpenAI-compatible Responses endpoint.

Use the two complete templates as separate `CODEX_HOME/config.toml` files:

- `codex-relay-shell.toml.example` keeps Codex's default Shell launcher.
- `codex-relay-transparent.toml.example` adds the trusted `mbtx exec --`
  launcher while preserving the original command for approval and policy.

The online collector creates equivalent run-local copies automatically and
does not write the API key into the run directory. The examples contain only a
placeholder and must not be edited in place with a real secret.

If the relay gives you the JSON form, keep a private copy outside Git and
export its value before collecting:

```bash
export OPENAI_API_KEY="$(jq -r '.OPENAI_API_KEY' /private/path/credentials.json)"
```

The relay recommendation's `network_access = "enabled"` is translated to
`[sandbox_workspace_write] network_access = true` for the pinned Codex release.
The top-level `disable_response_storage` and
`windows_wsl_setup_acknowledged` fields are omitted because this pinned release
does not accept them under `--strict-config`.
