# Local SPRT testing

The engine now implements the minimum UCI command set needed to start running
automated matches. The current search is synchronous and fixed-depth, so clock,
node, and stop-aware search remain follow-up work before results should be
treated as representative strength measurements.

## Prepare two engine versions

Build a release binary for the baseline revision and the candidate revision,
then keep them at distinct paths. For example:

```sh
cargo build --release
cp target/release/chess-kit /tmp/chess-kit-candidate
```

Repeat from the baseline revision and save it as `/tmp/chess-kit-baseline`.

## Run a smoke match

With [`cutechess-cli`](https://github.com/cutechess/cutechess) installed, first
verify that both binaries can finish games:

```sh
cutechess-cli \
  -engine name=baseline cmd=/tmp/chess-kit-baseline proto=uci \
  -engine name=candidate cmd=/tmp/chess-kit-candidate proto=uci \
  -each tc=10+0.1 \
  -games 2 -rounds 5 -repeat \
  -pgnout smoke.pgn
```

## Start an SPRT

Once smoke games are stable, add an SPRT termination rule. The `[0, 10]` Elo
bounds below are a reasonable initial gainer test for a young engine, rather
than a universal default:

```sh
cutechess-cli \
  -engine name=baseline cmd=/tmp/chess-kit-baseline proto=uci \
  -engine name=candidate cmd=/tmp/chess-kit-candidate proto=uci \
  -each tc=10+0.1 \
  -games 2 -rounds 10000 -repeat \
  -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05 \
  -pgnout sprt.pgn
```

Use a balanced opening suite before relying on the result. Candidate and
baseline should receive both colors from every selected opening (`-repeat`),
and all other build and runtime settings should remain identical.
