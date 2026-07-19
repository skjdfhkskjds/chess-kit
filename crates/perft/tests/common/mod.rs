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

/// `perft_tests!` registers each perft case as an independent test for every
/// evaluation profile.
///
/// Each profile expands to a module named after the profile, and each case expands
/// to a `#[test]` function within that module. Every generated test constructs its
/// own perft dependencies, so cases share no mutable state and may run in parallel.
/// Profile attributes apply to every case in that profile, while case attributes
/// apply to the corresponding case in every profile.
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
            mod $profile {
                crate::common::perft_tests! {
                    @cases
                    [$(#[$profile_attribute])*]
                    [$eval_state]
                    $cases
                }
            }
        )+
    };
    (
        @cases
        $profile_attributes:tt
        [$eval_state:ty]
        {
            $(
                $(#[$case_attribute:meta])*
                $name:ident: $fen:literal, $depth:literal, $expected_nodes:literal;
            )+
        }
    ) => {
        $(
            crate::common::perft_tests! {
                @case
                $profile_attributes
                [$(#[$case_attribute])*]
                [$eval_state]
                $name: $fen, $depth, $expected_nodes
            }
        )+
    };
    (
        @case
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

pub(crate) use perft_tests;
