# Transparent launcher evaluation

Each experiment freezes the implementation revision, toolchain, dependency
lock, model settings, task fixture, prompt, environment, seed, and platform.
Shell and transparent MBTX are adjacent AB/BA arms over the same immutable
fixture and separate mutable workspaces.

Eight task classes are collected in four valid pairs each (32 pairs, 64 arms).
Failed attempts are retained. Collection may expand to at most six pairs per
class (48 pairs, 96 arms). A single relay or provider error does not stop
collection. Collection pauses only after five consecutive infrastructure
failures, infrastructure failure in at least half of the latest 16 arms, or
loss of the local runner itself. A partial report is always emitted.

The report has intention-to-treat and complete-pair views. It attributes timing
segments to launcher, child, Codex, relay, or unknown evidence. Since
transparent mode uses Direct execution while Shell may use ZshFork, it contains
both the default Shell comparison and a Direct Shell control.
