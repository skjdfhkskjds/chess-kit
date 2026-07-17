# Engine strength-testing roadmap

This document defines how chess-kit should measure whether an incremental
engine change preserves or improves playing strength. It adapts the workflow in
[Proper Chess Engine Testing][guide] to this repository and separates three
questions that require different evidence:

1. **Correctness:** does the implementation obey chess rules and its software
   contracts?
2. **Performance:** does it search the same work faster, or more work in the
   same time?
3. **Strength:** does it score better in paired games against the previous
   version?

A strength result cannot replace a correctness test, and a faster benchmark
does not by itself demonstrate an Elo gain. Every change should pass the
cheapest applicable gates before consuming games in a strength test.

The companion [local SPRT runbook](sprt.md) contains concrete match commands.
This document describes the capabilities that must be built and the policy for
using them.

## Current state

| Capability | Status | Evidence or gap |
| --- | --- | --- |
| Unit and integration tests | Available | `cargo test --workspace --lib --tests` exercises the workspace, including search and engine behavior. |
| Move-generation validation | Available | The `chess-kit-perft` crate has smoke and larger perft fixture suites. |
| UCI position setup | Available | `position startpos`, `position fen`, and move histories are supported. |
| UCI search limits | Parsed, not enforced | Clock, increment, node, and move-time values are parsed, but [`UciAdapter::search`](../crates/comm/src/uci/adapter.rs) forwards only a depth to the engine. |
| Interruptible search | Not available | Search is synchronous; `stop` and `ponderhit` cannot interrupt it. |
| Runtime resource options | Not available | Hash is fixed at 1024 MB in [`src/main.rs`](../src/main.rs); there is no `setoption` support or thread option. |
| Engine benchmark fingerprint | Not available | Collection microbenchmarks and timed perft runs exist, but there is no deterministic engine `bench` command. |
| Opening suite | Not available | No versioned PGN/EPD opening set is stored or fetched by repository tooling. |
| Match automation | Documentation only | The current [SPRT guide](sprt.md) describes manual cutechess-cli use. |
| Continuous integration | Not available | There is no checked-in CI workflow for correctness, benchmark, or match tests. |

The immediate implication is important: chess-kit can be used to test whether
a match runner and the UCI process complete games, but clock-based results are
not representative measurements of strength yet. An engine that always
searches to the fallback depth is not competing under the time control supplied
by the runner.

## Testing policy for incremental changes

Classify a change before testing it. If a change fits more than one row, use the
union of the requirements.

| Change type | Targeted correctness | Workspace tests | Perft | Benchmark fingerprint | Self-play smoke | Statistical match |
| --- | --- | --- | --- | --- | --- | --- |
| Search or evaluation feature/tuning | Required | Required | Smoke | Expected to change; explain why | Required | Gainer SPRT |
| Refactor or other intended non-functional change | Required | Required | Smoke | Must not change; investigate if it does | Required when engine-facing | Non-regression SPRT when strength-sensitive |
| Performance-only optimization | Required | Required | Smoke; full suite if move generation changes | Compare nodes, time, and NPS | Required | Non-regression SPRT at equal clocks; use a gainer test if claiming Elo |
| UCI, time-management, or engine-session change | Required protocol/integration tests | Required | Smoke | Record before and after | Required, including timeout cases | Non-regression SPRT after the playable/UCI gate |
| Correctness fix that can alter searched positions | Add a reproducer | Required | Relevant suite, full for move generation | Change is allowed and must be explained | Required | Gainer or non-regression SPRT according to the claim |
| Move generation, make/unmake, legality, or hashing | Add a reproducer | Required | Full suite | Record and investigate unexpected changes | Required | Non-regression SPRT after all correctness gates pass |
| Documentation/build-only change | As applicable | Required when executable output may change | Not normally needed | Must not change for build-only changes | Not needed | Not needed |

Do not weaken the required test after seeing an inconvenient result. A failed or
inconclusive test is evidence to investigate, not a result to relabel.

For each candidate:

1. Identify exactly one baseline commit and one candidate commit.
2. Record the expected functional, node-count, speed, and Elo effects.
3. Run targeted regressions, workspace tests, and the applicable perft suite.
4. Compare benchmark fingerprints.
5. Run a short baseline-versus-baseline framework sanity check, then a
   baseline-versus-candidate smoke match.
6. Run the preselected SPRT or fixed-game test without changing its bounds.
7. Attach the manifest, summary, and artifact locations to the change review.

## Phase 1: correctness gate

[The guide][strength-testing] explicitly distinguishes implementation
correctness from Elo testing and requires valid move generation before strength
testing. Chess-kit already has a useful base for this phase.

### Work

- Keep focused unit tests close to every new logical function.
- Add integration tests where search, evaluation, transposition, position, and
  protocol components interact.
- Add a minimal reproducing position for every discovered engine bug. Cover at
  least mate-score boundaries, terminal positions, hash collisions or stale
  entries, interrupted iterations, integer boundaries, and malformed UCI
  sequences as those features are introduced.
- Run the normal workspace suite:

  ```sh
  cargo test --workspace --lib --tests
  ```

- Run the release-mode perft smoke suite for every engine change:

  ```sh
  cargo test -p chess-kit-perft --release --test perft_smoke
  ```

- Run the full suite whenever move generation, position mutation, attack
  tables, legality, castling, en passant, promotion, or hashing could change:

  ```sh
  cargo test -p chess-kit-perft --release --test perft_full -- --ignored --no-capture
  ```

- Evaluate mutation testing after fast CI exists. It supplements test-quality
  review; it is not a precondition for the first strength matches.

### Exit criteria

- All normal tests and applicable perft cases pass from a clean checkout.
- A candidate cannot play an illegal move in the covered engine and UCI tests.
- Every fixed bug has a regression test that fails on the faulty revision.
- Failures stop the pipeline before benchmark or match games begin.

## Phase 2: playable and UCI gate

The minimum useful product for strength testing must finish legal games and
obey the match runner's search limits. Parsing a UCI token is not sufficient.

### Work

- Carry clock, increment, moves-to-go, fixed move time, node limit, and depth
  limits through the protocol-neutral engine boundary into search.
- Add a time manager with explicit soft and hard deadlines. Reject or safely
  handle zero/invalid budgets, reserve communication overhead, and always
  return the best move from the last completed iteration.
- Make search cancellable so `stop` is observed promptly. Ensure an interrupted
  iteration cannot overwrite a valid result or transposition entry with
  incomplete data.
- Exercise the guide's required UCI subset end to end: `uci`, `isready`,
  `ucinewgame`, `position startpos`, `position fen`, clock-based `go`, `stop`,
  and `quit`.
- Add runtime `Hash` configuration and keep both competitors equal. Add
  `Threads` only when parallel search exists; initial tests should use one
  thread.
- Add process-level tests for command ordering, EOF, malformed input, rapid
  `stop`, no-legal-move positions, and timeout behavior.
- Run baseline versus itself before testing a candidate. The purpose is to
  expose runner, opening, color, crash, timeout, or adjudication problems, not
  to prove that identical binaries have identical short-match scores.

### Exit criteria

- Baseline-versus-baseline and baseline-versus-candidate smoke matches finish
  without crashes, stalls, time forfeits, or illegal moves.
- Clock, move-time, node, and depth limits have end-to-end tests and measured
  tolerances.
- `stop` interrupts live search and returns a legal move within a documented
  response bound.
- The runner can assign identical hash and thread resources to both engines.
- Only after these criteria pass may a clock-based match be called a strength
  test.

## Phase 3: deterministic benchmark fingerprint

The guide recommends a benchmark over fixed positions to identify whether a
commit changed search behavior. The stable fingerprint is the total node count;
wall-clock time and NPS are machine-dependent performance observations.

### Work

- Add a command-line entry point:

  ```sh
  ./target/release/chess-kit bench
  ```

- Version a compact suite of ordinary middlegames and edge cases. Include
  checkmate, stalemate, one-legal-move, zugzwang, repetition/draw-sensitive,
  promotion, castling, en-passant, tactical, quiet, and transposition-rich
  positions.
- Define fixed depth or node limits, hash size, thread count, position order,
  state-reset rules, and output format. Reset any state whose history could
  make the total depend on a previous run.
- Print per-position diagnostics and a final machine-readable summary containing
  at least total nodes, elapsed time, and NPS. OpenBench expects the human
  output to expose nodes and NPS, for example:

  ```text
  8763483 nodes / 3.426305007s := 2557706 nps
  ```

- Run the benchmark twice in the same build and fail if the node fingerprint
  differs. Track speed separately with multiple samples and controlled CPU
  conditions.
- Add an OpenBench-compatible build target only when distributed testing is
  adopted. For this Rust workspace it should produce the requested executable
  name from a release build, without changing source-controlled files.

### Exit criteria

- Repeated runs of one binary produce the same node fingerprint.
- Every functional search change records and explains its old and new
  fingerprints.
- Intended non-functional changes preserve the fingerprint.
- Benchmark output can be parsed by future local automation and supplies the
  node value required by OpenBench.

## Phase 4: reproducible local match harness

Use an external runner such as cutechess-cli or fastchess rather than building a
tournament manager into the engine. The [local runbook](sprt.md) standardizes
the initial cutechess-cli workflow.

### Work

- Build baseline and candidate from recorded commits with the same Rust
  toolchain, target CPU, profile, features, and linker settings.
- Pin runner version, opening-suite version and checksum, opening selection
  range, random seed, time control, concurrency, hash, threads, and adjudication
  policy in a manifest.
- Use a balanced opening suite. Play each selected opening as a pair with colors
  reversed (`-repeat`) and use pentanomial statistics where the runner supports
  them.
- Start with:
  1. a baseline-versus-baseline sanity match;
  2. a short baseline-versus-candidate smoke match;
  3. the predeclared statistical test.
- Keep concurrency low enough that workers do not contend for CPU or memory.
  Give each process identical resources and account for protocol/OS time margin.
- Write PGN, runner logs, result summaries, and the manifest below a unique
  `results/` directory. That directory is intentionally ignored by Git; publish
  durable artifacts through CI/OpenBench and put the concise result in the
  change review.
- Do not silently enable resign or draw adjudication for a young engine. First
  validate adjudication against complete games; record any later policy change
  because it can alter the result distribution.

### Exit criteria

- A manifest can reproduce the same matchup and opening order.
- Every game has a PGN and abnormal exits are retained in logs.
- Paired colors, fixed resources, and a versioned opening suite are mandatory.
- Smoke failures prevent the statistical run.

## Phase 5: statistical acceptance policy

Use a [Sequential Probability Ratio Test][sprt-guide] to choose between two
predeclared Elo hypotheses. SPRT answers whether the result is statistically
closer to `elo0` or `elo1`; it does not provide an exact Elo measurement.

### Bounds

Use `alpha=0.05` and `beta=0.05` unless a test plan records a different reasoned
choice.

- **Gainer test:** `[0, 10]` for a young or weaker engine, or a feature expected
  to have a large effect. Tighten toward `[0, 5]` as the engine becomes stronger
  or expected gains become smaller.
- **Non-regression test:** `[-10, 0]` initially. Tighten toward `[-5, 0]` when a
  five-Elo loss becomes material.

Select the bounds before starting. Do not restart with easier bounds because a
test trends toward failure.

### Decisions

- Reaching the upper LLR boundary accepts `H1`: a gainer passed, or a
  non-regression candidate stayed sufficiently close to zero.
- Reaching the lower boundary accepts `H0`: reject the claimed gain or treat
  the allowed regression as unresolved according to the declared hypotheses.
- A stopped or exhausted run that reaches neither boundary is inconclusive.
  Continue the same test when valid, or record it as inconclusive; do not call
  the current W/L/D score a pass.
- Examine paired/pentanomial counts, time losses, crashes, and opening balance
  alongside LLR. A statistical pass does not excuse an operational defect.
- Re-run surprising or marginal results with a new recorded seed and otherwise
  unchanged conditions. Never combine incompatible time controls, opening
  suites, builds, or adjudication settings into one result.

Strength-sensitive changes should not be merged without the required recorded
SPRT outcome. An exception must state why the test is inapplicable and what
evidence replaces it.

### Absolute strength

Relative development testing compares candidate to baseline. To estimate an
absolute rating, use a fixed-game match against a pinned release of a reference
engine with a documented rating history, as described in
[Determining Engine Strength][determining-strength]. Use roughly 1,000–5,000
games depending on the desired confidence interval and report the estimate with
its error. Ratings depend on time control, hardware, openings, opponent pool,
and configuration; do not present the result as universally comparable Elo.

## Phase 6: automation and scale

Automate in layers so cheap failures are reported before expensive games.

### Per-change automation

1. Formatting and lint checks.
2. Workspace unit and integration tests.
3. Release perft smoke suite.
4. Applicable full perft suite.
5. Benchmark fingerprint comparison.
6. Manual or dispatched strength test for strength-sensitive changes.

Do not run thousands of games in ordinary pull-request CI. Use a manual,
scheduled, or dedicated worker job and report its artifact URL and conclusion
back to the review.

### OpenBench adoption

[OpenBench][openbench-guide] becomes useful when local capacity or multiple
contributors make distributed testing worthwhile. Adoption requires:

- a real deterministic `bench` command and recorded node fingerprint;
- a build target compatible with OpenBench's requested executable name;
- a pinned Rust toolchain and documented CPU feature policy;
- an engine configuration specifying source, compiler requirements, NPS,
  default bounds, and test resources;
- a versioned opening suite and one or more controlled workers; and
- an operational policy for approving tests and preserving results.

OpenBench-specific build and benchmark integration is not required to run local
cutechess-cli or fastchess tests. It should not block the earlier local phases.

### Exit criteria

- Correctness and fingerprint gates run from a clean checkout without manual
  setup beyond the documented toolchain.
- Strength jobs retain manifests, logs, PGNs, and summaries.
- A failed required gate blocks acceptance; reruns remain linked to the original
  result.
- The project has one discoverable history of accepted, rejected, and
  inconclusive engine experiments.

## Test manifest and result record

Store a machine-readable manifest beside each local result and copy its concise
summary into the pull request or experiment tracker. A future harness may
generate this file; until then, fill it in manually.

```yaml
test_id: 2026-07-17-example-change
purpose: gainer
baseline:
  commit: <full SHA>
  binary_sha256: <SHA-256>
  bench_nodes: <node fingerprint>
candidate:
  commit: <full SHA>
  binary_sha256: <SHA-256>
  bench_nodes: <node fingerprint>
build:
  rustc: <rustc --version --verbose>
  profile: release
  target_cpu: <portable or exact CPU policy>
  flags: <RUSTFLAGS and Cargo features>
host:
  os: <OS and kernel>
  cpu: <CPU model>
runner:
  name: cutechess-cli
  version: <version>
  concurrency: 1
engines:
  threads: 1
  hash_mb: <equal size>
match:
  time_control: <base+increment>
  time_margin_ms: <margin>
  opening_file: <path or URL>
  opening_sha256: <SHA-256>
  opening_format: <pgn or epd>
  opening_range: <range>
  seed: <integer>
  paired: true
  adjudication: none
sprt:
  elo0: 0
  elo1: 10
  alpha: 0.05
  beta: 0.05
result:
  status: <passed, failed, inconclusive, or aborted>
  games: <count>
  wins: <candidate wins>
  losses: <candidate losses>
  draws: <draws>
  pentanomial: [<0>, <0.5>, <1>, <1.5>, <2>]
  llr: <value>
  llr_bounds: [<lower>, <upper>]
artifacts:
  pgn: <path or URL>
  runner_log: <path or URL>
  summary: <path or URL>
notes: <timeouts, crashes, deviations, and interpretation>
```

Record aborted tests too. Omitting failed setup attempts makes the process hard
to audit and encourages accidental cherry-picking.

## References

- [Engine Strength Testing][strength-testing]
- [Sequential Probability Ratio Test][sprt-guide]
- [OpenBench][openbench-guide]
- [Determining Engine Strength][determining-strength]
- [Universal Chess Interface protocol][uci]

[guide]: https://dannyhammer.github.io/engine-testing-guide/
[strength-testing]: https://dannyhammer.github.io/engine-testing-guide/strength-testing.html
[sprt-guide]: https://dannyhammer.github.io/engine-testing-guide/sprt.html
[openbench-guide]: https://dannyhammer.github.io/engine-testing-guide/openbench.html
[determining-strength]: https://dannyhammer.github.io/engine-testing-guide/determining-strength.html
[uci]: https://backscattering.de/chess/uci/
