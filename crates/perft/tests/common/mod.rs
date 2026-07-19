use std::time::Instant;

use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::{Accumulator, DefaultAccumulator, EvalState};
use chess_kit_movegen::{DefaultMoveGenerator, MoveGenerator};
use chess_kit_perft::{NodeCount, PerftData, perft};
use chess_kit_position::{DefaultPosition, Fen, Setup};
use chess_kit_primitives::Depth;
use chess_kit_transposition::{DefaultTranspositionTable, TranspositionTable};

const TRANSPOSITION_TABLE_SIZE: usize = 0;

/// `assert_perft` runs one isolated perft case with the selected evaluation state.
///
/// @marker: EvalStateT - evaluation state exercised while moves are made and undone
/// @param: fen - position to test in Forsyth-Edwards Notation
/// @param: depth - depth at which to count leaf nodes
/// @param: expected_nodes - expected leaf-node count
/// @return: void
/// @side-effects: prints timing information and panics if the node count differs
pub(crate) fn assert_perft<EvalStateT>(fen: &str, depth: Depth, expected_nodes: NodeCount)
where
    EvalStateT: EvalState,
{
    let parsed_fen =
        Fen::try_from(fen).unwrap_or_else(|error| panic!("invalid perft FEN '{fen}': {error}"));
    let mut position: DefaultPosition<DefaultAttackTable> = Setup::from(parsed_fen).into();
    let move_generator = DefaultMoveGenerator::<DefaultAttackTable>::new();
    let mut transposition_table =
        DefaultTranspositionTable::<PerftData>::new(TRANSPOSITION_TABLE_SIZE);
    let mut accumulator = DefaultAccumulator::<EvalStateT>::new();
    accumulator.push(EvalStateT::from_position(&position));

    let started_at = Instant::now();
    let actual_nodes = perft(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        depth,
    );
    let elapsed = started_at.elapsed();
    let elapsed_seconds = elapsed.as_secs_f64();
    let nodes_per_second = if elapsed_seconds == 0.0 {
        actual_nodes as f64
    } else {
        actual_nodes as f64 / elapsed_seconds
    };

    println!("depth {depth}: {actual_nodes} nodes in {elapsed:?} ({nodes_per_second:.0} nodes/s)");

    assert_eq!(
        actual_nodes, expected_nodes,
        "perft mismatch at depth {depth} for FEN '{fen}'"
    );
}

// `perft_test!` emits one native test after the outer macros select its profile
// and case.
macro_rules! perft_test {
    (
        [$($profile_attribute:tt)*]
        [$($case_attribute:tt)*]
        [$eval_state:ty]
        $name:ident: $fen:literal, $depth:literal, $expected_nodes:literal
    ) => {
        #[test]
        $($profile_attribute)*
        $($case_attribute)*
        fn $name() {
            crate::common::assert_perft::<$eval_state>($fen, $depth, $expected_nodes);
        }
    };
}

// `perft_test_profile!` gives one evaluation profile its own module and emits
// that profile's copy of every case.
macro_rules! perft_test_profile {
    (
        $profile_attributes:tt
        $profile:ident: $eval_state:ty;
        {
            $(
                $(#[$case_attribute:meta])*
                $name:ident: $fen:literal, $depth:literal, $expected_nodes:literal;
            )+
        }
    ) => {
        mod $profile {
            $(
                crate::common::perft_test! {
                    $profile_attributes
                    [$(#[$case_attribute])*]
                    [$eval_state]
                    $name: $fen, $depth, $expected_nodes
                }
            )+
        }
    };
}

/// `perft_tests!` registers each perft case as an independent test for every
/// evaluation profile.
///
/// The `profiles` block maps module names to evaluation-state types. Each profile
/// expands to a module named after the profile, and profile attributes apply to
/// every test in that module.
///
/// The `cases` block maps test names to FEN, depth, and expected-node tuples. Each
/// case expands to a `#[test]` function in every profile module, and case attributes
/// apply to that case in every profile. Every generated test constructs its own
/// perft dependencies, so cases share no mutable state and may run in parallel.
///
/// Usage:
///
/// ```
/// perft_tests! {
///     profiles {
///         psqt: chess_kit_eval::PSQTEvalState;
///
///         #[ignore = "run explicitly with the no-op evaluation state"]
///         noop: chess_kit_eval::NoOpEvalState;
///     }
///     cases {
///         start_depth_1:
///             "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
///             1,
///             20;
///     }
/// }
/// ```
macro_rules! perft_tests {
    (
        profiles {
            $(
                $(#[$profile_attribute:meta])*
                $profile:ident: $eval_state:ty;
            )+
        }
        cases $cases:tt
    ) => {
        $(
            crate::common::perft_test_profile! {
                [$(#[$profile_attribute])*]
                $profile: $eval_state;
                $cases
            }
        )+
    };
}

pub(crate) use {perft_test, perft_test_profile, perft_tests};
