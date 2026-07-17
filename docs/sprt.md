# Local SPRT testing

This is the operational runbook for comparing a candidate chess-kit revision
with a baseline using
[`cutechess-cli`](https://github.com/cutechess/cutechess). Read the
[strength-testing roadmap](strength-testing-roadmap.md) for prerequisites,
phase gates, and the policy for choosing a test.

## Current limitation

**Clock-based matches are framework checks, not representative strength tests
yet.**

The UCI parser accepts clock, increment, node, and move-time limits, but
[`UciAdapter::search`](../crates/comm/src/uci/adapter.rs) currently passes only
`depth` to synchronous engine search. `stop` cannot interrupt that search.
Consequently, a runner command with `tc=10+0.1` still makes chess-kit search to
its fallback depth rather than manage the supplied clock.

The current binary can be used for short protocol and legal-game smoke checks.
Do not report Elo or accept an SPRT result until the playable/UCI gate in the
roadmap is complete and end-to-end tests show that search obeys its limits.

## Prerequisites for a representative test

Before starting an SPRT:

- all targeted tests, workspace tests, and applicable perft suites pass;
- clock-based `go`, `movetime`, `nodes`, `depth`, and `stop` work end to end;
- a baseline-versus-baseline match has completed without illegal moves,
  crashes, stalls, or time forfeits;
- the benchmark fingerprint has been recorded for both commits;
- a balanced opening suite, exact opening checksum, and random seed have been
  selected;
- both engines use the same build flags, hash, threads, time control, and
  adjudication policy; and
- the SPRT hypotheses and stopping policy have been selected before games
  begin.

Use one search thread initially. Keep runner concurrency low enough to avoid CPU
oversubscription and memory pressure.

## Prepare the test directory

Create one directory for the test's manifest, logs, PGN, and summary. `results/`
is ignored by Git, so copy durable artifacts to the pull request's CI run or the
project's future experiment store.

```sh
TEST_ID="$(date -u +%Y%m%dT%H%M%SZ)-candidate-name"
RESULT_DIR="results/$TEST_ID"
mkdir -p "$RESULT_DIR"
```

Start a manifest using the template in the
[roadmap](strength-testing-roadmap.md#test-manifest-and-result-record). Record at
least the full commit SHAs, binary checksums, compiler and runner versions, CPU,
build flags, opening checksum, seed, engine resources, and test bounds.

## Build baseline and candidate

Build both revisions from clean worktrees with one toolchain and one set of
flags. The commands below are a pattern; replace the revisions and preserve any
existing worktrees.

```sh
git worktree add /tmp/chess-kit-baseline-worktree <baseline-commit>
git worktree add /tmp/chess-kit-candidate-worktree <candidate-commit>

cargo build --release --manifest-path /tmp/chess-kit-baseline-worktree/Cargo.toml
cargo build --release --manifest-path /tmp/chess-kit-candidate-worktree/Cargo.toml

cp /tmp/chess-kit-baseline-worktree/target/release/chess-kit /tmp/chess-kit-baseline
cp /tmp/chess-kit-candidate-worktree/target/release/chess-kit /tmp/chess-kit-candidate

sha256sum /tmp/chess-kit-baseline /tmp/chess-kit-candidate
rustc --version --verbose
cutechess-cli --version
```

Do not compare a debug binary with a release binary or mix `target-cpu`,
`RUSTFLAGS`, Cargo features, or toolchain versions. Native CPU builds are valid
when both binaries run on the same compatible host and that fact is recorded;
portable builds are easier to reproduce on other workers.

## Run framework sanity checks

First run the baseline against itself. This checks process startup, UCI
communication, game completion, and runner configuration. It does not need to
produce an exactly even score in a small sample.

```sh
set -o pipefail
cutechess-cli \
  -engine name=baseline-a cmd=/tmp/chess-kit-baseline proto=uci \
  -engine name=baseline-b cmd=/tmp/chess-kit-baseline proto=uci \
  -each tc=10+0.1 timemargin=250 \
  -games 2 -rounds 5 -repeat \
  -concurrency 1 \
  -pgnout "$RESULT_DIR/baseline-self.pgn" \
  2>&1 | tee "$RESULT_DIR/baseline-self.log"
```

Then run a short baseline-versus-candidate smoke match:

```sh
set -o pipefail
cutechess-cli \
  -engine name=baseline cmd=/tmp/chess-kit-baseline proto=uci \
  -engine name=candidate cmd=/tmp/chess-kit-candidate proto=uci \
  -each tc=10+0.1 timemargin=250 \
  -games 2 -rounds 5 -repeat \
  -concurrency 1 \
  -pgnout "$RESULT_DIR/candidate-smoke.pgn" \
  2>&1 | tee "$RESULT_DIR/candidate-smoke.log"
```

These exact commands are usable today only as framework smoke checks because of
the current search limitation. Inspect every abnormal termination. Do not
continue after a crash, illegal move, hang, or time loss.

## Select and verify openings

Do not rely on repeated start-position games for a strength conclusion. Select
a reputable balanced PGN or EPD opening suite, pin its version and license, and
record its SHA-256 checksum:

```sh
OPENINGS=/absolute/path/to/openings.epd
sha256sum "$OPENINGS"
```

Use the same opening once with each engine as White. In cutechess-cli,
`-repeat` pairs colors. Fix the random seed so the test can be reproduced.
Different confirmation runs should use a new seed that is recorded before they
start.

## Run the SPRT

The following command is the target workflow after clock-aware, interruptible
search and runtime `Hash`/`Threads` options exist. It is intentionally not a
claim that the current binary supports those capabilities.

```sh
set -o pipefail
cutechess-cli \
  -engine name=baseline cmd=/tmp/chess-kit-baseline proto=uci option.Hash=16 option.Threads=1 \
  -engine name=candidate cmd=/tmp/chess-kit-candidate proto=uci option.Hash=16 option.Threads=1 \
  -each tc=8+0.08 timemargin=250 \
  -openings file="$OPENINGS" format=epd order=random \
  -srand 944 \
  -games 2 -rounds 15000 -repeat \
  -concurrency 1 -recover \
  -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05 \
  -pgnout "$RESULT_DIR/sprt.pgn" \
  2>&1 | tee "$RESULT_DIR/sprt.log"
```

Confirm the option names against the implemented UCI output before using the
target command. If the opening file is PGN, change `format=epd` to `format=pgn`.
Choose `-rounds` high enough that the SPRT, rather than the round cap, normally
stops the test.

`-recover` keeps the tournament runner alive after some engine failures; it
does not make those failures acceptable. Retain and investigate them. Avoid
resign and draw adjudication until it has been validated against complete games
for the engine's current strength.

## Choose bounds before running

With `alpha=0.05` and `beta=0.05`, use:

- `elo0=0 elo1=10` for an initial **gainer** test on a young engine or a change
  expected to gain substantial Elo;
- `elo0=0 elo1=5` for a smaller expected gain or a stronger engine;
- `elo0=-10 elo1=0` for an initial **non-regression** test; or
- `elo0=-5 elo1=0` when a five-Elo regression is already material.

These are starting policies from the
[SPRT chapter](https://dannyhammer.github.io/engine-testing-guide/sprt.html),
not universal constants. Select bounds that match the claim and expected
effect. Do not make them easier after observing results.

Use gainer bounds for search/evaluation features and tuning intended to improve
play. Use non-regression bounds for refactors, protocol work, and optimizations
that are intended not to weaken the engine. A performance optimization claiming
an Elo gain still requires a gainer test at equal clocks.

## Interpret and record the result

Use the runner's reported LLR and boundaries:

- the upper boundary accepts `H1`;
- the lower boundary accepts `H0`; and
- reaching neither boundary is inconclusive, not a pass.

At five-percent false-positive and false-negative thresholds, the basic
unpaired theoretical boundaries are approximately `(-2.94, 2.94)`. Treat the
runner's displayed boundaries as authoritative because its model and paired
statistics may affect the reported values.

Record:

- the SPRT status and final LLR with its bounds;
- game count, candidate wins, losses, and draws;
- pentanomial counts for paired games when available;
- time forfeits, crashes, illegal moves, and recoveries;
- both benchmark node fingerprints;
- links or paths to the manifest, complete log, and PGN; and
- any deviation from the predeclared configuration.

Do not infer an exact Elo gain from an SPRT. The displayed Elo and confidence
interval are estimates. For an absolute strength estimate, run a separate
fixed-game match against a pinned reference-engine release as described in
[Determining Engine Strength](https://dannyhammer.github.io/engine-testing-guide/determining-strength.html).

## Cleanup

After artifacts are safely stored and the worktrees are no longer needed:

```sh
git worktree remove /tmp/chess-kit-baseline-worktree
git worktree remove /tmp/chess-kit-candidate-worktree
```

Do not remove a worktree that contains uncommitted work.
