use chess_kit_primitives::Move;

/// format_uci_move formats an engine move using lowercase UCI notation
///
/// @param: mv - move to format
/// @return: UCI move string
pub fn format_uci_move(mv: Move) -> String {
    // Piece display uses uppercase letters, while UCI promotions are lowercase.
    mv.to_string().to_ascii_lowercase()
}
