# Codex relay configuration

The relay-supplied credentials JSON is represented by the `OPENAI_API_KEY`
environment variable. `env_key = "OPENAI_API_KEY"` explicitly selects that
variable for the provider's Bearer authentication, with
`requires_openai_auth = false`. An isolated `CODEX_HOME` has no saved login;
`requires_openai_auth = true` alone does not load this variable in the pinned
release. Do not write a key into the config or copy `auth.json` into evidence.

Use the two complete templates as separate `CODEX_HOME/config.toml` files:

- `codex-relay-shell.toml.example` keeps Codex's default Shell launcher.
- `codex-relay-transparent.toml.example` adds the trusted `mbtx exec --`
  launcher while preserving the original command for approval and policy.

The online collector creates equivalent run-local copies automatically and
does not write the API key into the run directory. The examples contain only a
placeholder and must not be edited in place with a real secret.

Both arms enable `features.unified_exec` and `features.code_mode_host`, and
disable `features.plugins` so each arm does not perform unrelated curated-plugin
network synchronization.
The pinned model catalog selects `code_mode_only` for `gpt-5.6-terra`, so its
`exec_command` tool is called through Code Mode. The collector builds the CLI
and `codex-code-mode-host` together and preserves this model tool mode.

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
