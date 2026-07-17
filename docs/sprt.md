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
- both engines use the same build flags, hash, time control, and adjudication
  policy, plus equal threads if threads are configurable; and
- the SPRT hypotheses and stopping policy have been selected before games
  begin.

Chess-kit is currently inherently single-threaded, so a `Threads` UCI option is
not a prerequisite. Keep runner concurrency low enough to avoid CPU
oversubscription and memory pressure.

## Prepare the test directory

Create one directory for the test's binaries, manifest, logs, PGN, and summary.
`results/` is ignored by Git, so upload durable artifacts to a declared artifact
store, OpenBench, or a future CI workflow.

```sh
set -euo pipefail
TEST_ID="$(date -u +%Y%m%dT%H%M%SZ)-candidate-name"
RESULT_DIR="$(pwd)/results/$TEST_ID"
BIN_DIR="$RESULT_DIR/bin"
mkdir -p "$BIN_DIR"
```

Start a manifest using the template in the
[roadmap](strength-testing-roadmap.md#test-manifest-and-result-record). Record at
least the full commit SHAs, binary checksums, compiler and runner versions, CPU,
build flags, opening checksum, seed, engine resources, and test bounds.

## Build baseline and candidate

Rust edition 2024 requires Rust 1.85 or newer. Until the project pins a
toolchain, build both revisions with the same newer toolchain and record it.
Build from clean, detached worktrees with one set of flags. The commands below
are a pattern; replace the commit values.

```sh
set -euo pipefail
BASELINE_COMMIT="replace-with-full-baseline-sha"
CANDIDATE_COMMIT="replace-with-full-candidate-sha"
BASELINE_WORKTREE="/tmp/chess-kit-$TEST_ID-baseline"
CANDIDATE_WORKTREE="/tmp/chess-kit-$TEST_ID-candidate"
BASELINE_BINARY="$BIN_DIR/chess-kit-baseline"
CANDIDATE_BINARY="$BIN_DIR/chess-kit-candidate"

rm -f "$BASELINE_BINARY" "$CANDIDATE_BINARY"
git worktree add --detach "$BASELINE_WORKTREE" "$BASELINE_COMMIT"
git worktree add --detach "$CANDIDATE_WORKTREE" "$CANDIDATE_COMMIT"

test "$(git -C "$BASELINE_WORKTREE" rev-parse HEAD)" = "$BASELINE_COMMIT"
test "$(git -C "$CANDIDATE_WORKTREE" rev-parse HEAD)" = "$CANDIDATE_COMMIT"

cargo build --locked --release --manifest-path "$BASELINE_WORKTREE/Cargo.toml"
cargo build --locked --release --manifest-path "$CANDIDATE_WORKTREE/Cargo.toml"

cp "$BASELINE_WORKTREE/target/release/chess-kit" "$BASELINE_BINARY"
cp "$CANDIDATE_WORKTREE/target/release/chess-kit" "$CANDIDATE_BINARY"
test -x "$BASELINE_BINARY"
test -x "$CANDIDATE_BINARY"

sha256sum "$BASELINE_BINARY" "$CANDIDATE_BINARY"
rustc --version --verbose
cargo --version
cutechess-cli --version
sha256sum "$(command -v cutechess-cli)"
```

Do not compare a debug binary with a release binary or mix `target-cpu`,
`RUSTFLAGS`, Cargo features, or toolchain versions. Native CPU builds are valid
when both binaries run on the same compatible host and that fact is recorded;
portable builds are easier to reproduce on other workers.

## Optional current framework smoke checks

Before the playable/UCI gate is complete, the following checks only process
startup, basic UCI communication, and game completion. They do not validate a
future strength-test configuration and do not need to produce an exactly even
score in a small sample.

The current binary eagerly allocates a 1024 MB transposition table per process.
Two-engine checks therefore need more than 2 GiB of available memory plus
runner and operating-system overhead.

```sh
set -euo pipefail
cutechess-cli \
  -engine name=baseline-a cmd="$BASELINE_BINARY" proto=uci \
  -engine name=baseline-b cmd="$BASELINE_BINARY" proto=uci \
  -each tc=10+0.1 timemargin=250 \
  -games 2 -rounds 5 -repeat \
  -concurrency 1 \
  -pgnout "$RESULT_DIR/baseline-self.pgn" \
  2>&1 | tee "$RESULT_DIR/baseline-self.log"
```

Then run a short baseline-versus-candidate smoke match:

```sh
set -euo pipefail
cutechess-cli \
  -engine name=candidate cmd="$CANDIDATE_BINARY" proto=uci \
  -engine name=baseline cmd="$BASELINE_BINARY" proto=uci \
  -each tc=10+0.1 timemargin=250 \
  -games 2 -rounds 5 -repeat \
  -concurrency 1 \
  -pgnout "$RESULT_DIR/candidate-smoke.pgn" \
  2>&1 | tee "$RESULT_DIR/candidate-smoke.log"
```

Each command caps the check at 10 games. Inspect every abnormal termination. Do
not continue after a crash, illegal move, hang, or time loss.

## Select and verify openings

Do not rely on repeated start-position games for a strength conclusion. Select
a reputable balanced PGN or EPD opening suite, pin its version and license, and
record its SHA-256 checksum:

```sh
OPENINGS=/absolute/path/to/openings.epd
OPENING_FORMAT=epd
OPENING_PLIES=16
OPENING_POLICY=default
SEED=944
sha256sum "$OPENINGS"
```

Use the same opening once with each engine as White. In cutechess-cli,
`-repeat` pairs colors. Fix the random seed so the test can be reproduced.
Different confirmation runs should use a new seed that is recorded before they
start. Declare `plies` explicitly for PGN suites, because cutechess-cli
otherwise uses the complete PGN line; alternatively use a documented,
pre-truncated suite. Record the opening order, depth, start index, and policy.

## Run representative sanity checks

After the playable/UCI gate is complete, run preflights with the same openings,
seed, time control, hash, concurrency, and adjudication policy intended for the
SPRT. Disable recovery so an engine failure stops the preflight. Change only
the engine pairing and the short round cap.

```sh
set -euo pipefail
cutechess-cli \
  -engine name=baseline-a cmd="$BASELINE_BINARY" proto=uci option.Hash=16 \
  -engine name=baseline-b cmd="$BASELINE_BINARY" proto=uci option.Hash=16 \
  -each tc=8+0.08 timemargin=250 \
  -openings file="$OPENINGS" format="$OPENING_FORMAT" order=random \
    plies="$OPENING_PLIES" policy="$OPENING_POLICY" \
  -srand "$SEED" \
  -games 2 -rounds 5 -repeat \
  -concurrency 1 \
  -pgnout "$RESULT_DIR/representative-baseline-self.pgn" \
  2>&1 | tee "$RESULT_DIR/representative-baseline-self.log"

cutechess-cli \
  -engine name=candidate cmd="$CANDIDATE_BINARY" proto=uci option.Hash=16 \
  -engine name=baseline cmd="$BASELINE_BINARY" proto=uci option.Hash=16 \
  -each tc=8+0.08 timemargin=250 \
  -openings file="$OPENINGS" format="$OPENING_FORMAT" order=random \
    plies="$OPENING_PLIES" policy="$OPENING_POLICY" \
  -srand "$SEED" \
  -games 2 -rounds 5 -repeat \
  -concurrency 1 \
  -pgnout "$RESULT_DIR/representative-candidate-smoke.pgn" \
  2>&1 | tee "$RESULT_DIR/representative-candidate-smoke.log"
```

These commands assume the planned UCI `Hash` option exists. Confirm its exact
name in the engine's `uci` output.

## Run the SPRT

The following command is the target workflow after clock-aware, interruptible
search and a runtime `Hash` option exist. It is intentionally not a claim that
the current binary supports those capabilities.

```sh
set -euo pipefail
cutechess-cli \
  -engine name=candidate cmd="$CANDIDATE_BINARY" proto=uci option.Hash=16 \
  -engine name=baseline cmd="$BASELINE_BINARY" proto=uci option.Hash=16 \
  -each tc=8+0.08 timemargin=250 \
  -openings file="$OPENINGS" format="$OPENING_FORMAT" order=random \
    plies="$OPENING_PLIES" policy="$OPENING_POLICY" \
  -srand "$SEED" \
  -games 2 -rounds 15000 -repeat \
  -concurrency 1 -recover \
  -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05 \
  -pgnout "$RESULT_DIR/sprt.pgn" \
  2>&1 | tee "$RESULT_DIR/sprt.log"
```

Candidate must be engine A (the first `-engine`) so positive bounds test
candidate-minus-baseline Elo. Record that point of view in the manifest. The
command has a 30,000-game cap; choose the cap high enough that the SPRT, rather
than the cap, normally stops the test.

`-recover` keeps the tournament runner alive after some engine failures; it
does not make those failures acceptable. Retain and investigate them. Avoid
resign and draw adjudication until it has been validated against complete games
for the engine's current strength.

Cutechess-cli's SPRT uses trinomial W/L/D statistics and can stop between the
two games in a color-reversed pair. `-repeat` still controls color balance, but
it does not turn this into a pentanomial test. Prefer fastchess or OpenBench when
pair-boundary stopping and pentanomial statistics are required.

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
boundaries are approximately `(-2.94, 2.94)`. Record the runner's displayed
values as authoritative, especially when alpha and beta differ.

Record:

- the SPRT status and final LLR with its bounds;
- game count, candidate wins, losses, and draws;
- the statistical model and pentanomial counts only when the controller
  supports them;
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
git worktree remove "$BASELINE_WORKTREE"
git worktree remove "$CANDIDATE_WORKTREE"
rm -f "$BASELINE_BINARY" "$CANDIDATE_BINARY"
```

Do not remove a worktree that contains uncommitted work.
