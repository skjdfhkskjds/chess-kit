use chess_kit::{
    board::Board, movegen::MoveGenerator, perft::{perft, perft_divide_print}, primitives::Sides
};
mod testcases;
use testcases::TEST_CASES;

use std::time::Instant;

const SEMI_COLON: char = ';';
const SPACE: char = ' ';

struct TestResult;
impl TestResult {
    pub const ERR_NONE: usize = 0;
    pub const ERR_FEN: usize = 1;
    pub const ERR_DEPTH: usize = 2;
    pub const ERR_EXPECT: usize = 3;
    pub const ERR_FAIL: usize = 4;
}

const TEST_RESULTS: [&str; 5] = [
    "No errors. Test completed successfully.",
    "Errors in parsing the FEN-string.",
    "Errors parsing depth from test data.",
    "Errors parsing expected leaf nodes from test data.",
    "Failure: Found leaf nodes not equal to expected value.",
];

// This private function is the one actually running tests.
// This can be the entire suite, or a single test.
#[test]
fn run_tests() {
    // TODO: Write About
    let number_of_tests = TEST_CASES.len();
    let move_generator = MoveGenerator::new();
    let mut board: Board = Board::new();
    let mut result: usize = TestResult::ERR_NONE;

    // Run all the tests.
    let mut test_nr = 0;
    while (test_nr < number_of_tests) && (result == 0) {
        // Split the test's data string into multiple parts.
        let test_data: Vec<&str> = TEST_CASES[test_nr].split(SEMI_COLON).collect();
        let fen = test_data[0].trim();

        // Set up the position according to the provided FEN-string.
        let setup_result = Board::try_from(fen);
        println!("Test {} from {}", test_nr + 1, number_of_tests);
        println!("FEN: {fen}");

        // If setup ok, then print position. Else, print error and continue to the next test.
        match setup_result {
            Ok(b) => {
                println!("board: {}", b);
                for (i, piece) in b.pieces.iter().enumerate() {
                    println!("piece {}: {}", i, piece);
                }
                for (i, bitboard) in b.bitboards[Sides::WHITE].iter().enumerate() {
                    println!("white bitboard {}: {}", i, bitboard);
                }
                for (i, bitboard) in b.bitboards[Sides::BLACK].iter().enumerate() {
                    println!("black bitboard {}: {}", i, bitboard);
                }
                for (i, side) in b.sides.iter().enumerate() {
                    println!("side {}: {}", i, side);
                }

                board = b;
            },
            Err(_) => result = TestResult::ERR_FEN,
        };


        // Run all the parts of a test.
        let mut index: usize = 1;
        while index < test_data.len() && (result == 0) {
            // Data index 0 contains the FEN-string, so skip this and
            // start at index 1 to find the expected leaf nodes per depth.

            // Split "D1 20" into a vector containing "D1" (depth) and "20" (leaf nodes)
            let depth_ln: Vec<&str> = test_data[index].split(SPACE).collect();
            let depth = (depth_ln[0][1..]).parse::<u8>().unwrap_or(0) as u8;
            let expected_ln = depth_ln[1].parse::<u64>().unwrap_or(0);

            // Abort if depth parsing failed
            result = if depth == 0 {
                TestResult::ERR_DEPTH
            } else {
                result
            };

            // Abort if parsing expected number of leaf nodes failed
            result = if expected_ln == 0 {
                TestResult::ERR_EXPECT
            } else {
                result
            };

            if result == 0 {
                print!("Expect for depth {depth}: {expected_ln}");

                // This is the actual perft run for this test and depth.
                let now = Instant::now();
                let found_ln = perft_divide_print(
                    &mut board,
                    &move_generator,
                    depth,
                );
                let elapsed = now.elapsed().as_millis();
                let moves_per_second = ((found_ln * 1000) as f64 / elapsed as f64).floor();
                let is_ok = expected_ln == found_ln;

                // Print the results
                print!(" - Found: {found_ln}");
                print!(" - Result: {}", if is_ok { "OK" } else { "Fail" });
                println!(" ({elapsed} ms, {moves_per_second} leaves/sec)");

                assert_eq!(found_ln, expected_ln);
            }

            index += 1;
        }

        println!("Test {}: {}\n", test_nr + 1, TEST_RESULTS[result]);
        test_nr += 1;
    }
}
