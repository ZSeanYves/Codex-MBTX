# Online collection authentication audit

The Linux online artifacts were uploaded before the authentication and Code Mode
checks were corrected. They are retained unchanged under:

- `evidence/linux/codex-relay/20260913T054257468939522/` (smoke)
- `evidence/linux/codex-relay/20260913T055216479877978/` (formal attempt)

The Codex event stream proves that the collector did call the configured
Responses endpoint. Each arm reached `https://tokenadvent.com/v1/responses` and
received HTTP 401 with `API_KEY_REQUIRED`; no command execution event was
observed. The old report classifier included the relay hostname in its
transport rule, so those immutable reports label the attempts as relay errors.
That label is not used for the corrected interpretation: the observed failure
is provider authentication, and the run contains no backend comparison.

The corrected isolated-home configuration explicitly sets
`env_key = "OPENAI_API_KEY"` and `requires_openai_auth = false`. A local mock
Responses test verifies that the Bearer header is attached, the key is absent
from the evidence directory, HTTP 401 is classified as provider authentication,
and partial reports retain all attempts. A second local mock test runs all eight
task categories through both the default Shell and Transparent MBTX arms. It
produces 16 successful arms and 8 complete pairs without a real provider.

This audit does not turn the Linux online attempt into a valid performance or
behavior conclusion. A new Linux run with a valid relay credential is required;
the old smoke and formal artifacts remain the record of what was observed at
that time.
