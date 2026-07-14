use chess_kit_primitives::Pieces;

/// CaptureInfo contains information about the capture used to enter the current position
///
/// CaptureInfo is stored separately from [`super::position_metadata::PositionMetadata`]
/// because it describes only the move that entered a history ply and must not be copied
/// forward as persistent position metadata
///
/// @type
#[derive(Clone, Copy)]
pub(crate) struct CaptureInfo {
    pub(crate) captured_piece: Pieces,
}

impl Default for CaptureInfo {
    fn default() -> Self {
        Self {
            captured_piece: Pieces::None,
        }
    }
}
