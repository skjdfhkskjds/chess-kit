mod common;

use common::perft_tests;

perft_tests! {
    profiles {
        psqt: chess_kit_eval::PSQTEvalState;

        #[ignore = "run explicitly as the no-op evaluation performance suite"]
        noop: chess_kit_eval::NoOpEvalState;
    }
    cases {
        case_001: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 1, 20;
        case_002: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 2, 400;
        case_003: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 3, 8902;
        case_004: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, 197281;
        case_005: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 5, 4865609;
        case_006: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 6, 119060324;
        case_007: "8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1", 6, 824064;
        case_008: "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1", 6, 1440467;
        case_009: "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1", 6, 1440467;
        case_010: "5k2/8/8/8/8/8/8/4K2R w K - 0 1", 6, 661072;
        case_011: "4k2r/8/8/8/8/8/8/5K2 b k - 0 1", 6, 661072;
        case_012: "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1", 6, 803711;
        case_013: "r3k3/8/8/8/8/8/8/3K4 b q - 0 1", 6, 803711;
        case_014: "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1", 4, 1274206;
        case_015: "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1", 4, 1274206;
        case_016: "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1", 4, 1720476;
        case_017: "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1", 4, 1720476;
        case_018: "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1", 6, 3821001;
        case_019: "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1", 6, 3821001;
        case_020: "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1", 5, 1004658;
        case_021: "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1", 5, 1004658;
        case_022: "4k3/1P6/8/8/8/8/K7/8 w - - 0 1", 6, 217342;
        case_023: "8/k7/8/8/8/8/1p6/4K3 b - - 0 1", 6, 217342;
        case_024: "8/P1k5/K7/8/8/8/8/8 w - - 0 1", 6, 92683;
        case_025: "8/8/8/8/8/k7/p1K5/8 b - - 0 1", 6, 92683;
        case_026: "K1k5/8/P7/8/8/8/8/8 w - - 0 1", 6, 2217;
        case_027: "8/8/8/8/8/p7/8/k1K5 b - - 0 1", 6, 2217;
        case_028: "8/k1P5/8/1K6/8/8/8/8 w - - 0 1", 7, 567584;
        case_029: "8/8/8/8/1k6/8/K1p5/8 b - - 0 1", 7, 567584;
        case_030: "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1", 4, 23527;
        case_031: "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1", 4, 23527;
        case_032: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 5, 193690690;
        case_033: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 6, 11030083;
        case_034: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 5, 15833292;
        case_035: "rnbqkb1r/pp1p1ppp/2p5/4P3/2B5/8/PPP1NnPP/RNBQK2R w KQkq - 0 1", 3, 53392;
        case_036: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 1", 5, 164075551;
        case_037: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 7, 178633661;
        case_038: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 6, 706045033;
        case_039: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 5, 89941194;
        case_040: "1k6/1b6/8/8/7R/8/8/4K2R b K - 0 1", 5, 1063513;
        case_041: "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1", 6, 1134888;
        case_042: "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1", 6, 1015133;
    }
}
