#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Quiet,
    Capture,
    Evasions,
    NonEvasions,
}
