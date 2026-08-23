/// `EngineMessage` preserves a raw engine line and its parsed meaning.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineMessage {
    raw: String,
    kind: EngineMessageKind,
}

impl EngineMessage {
    /// new creates a parsed engine message.
    ///
    /// @param: raw - original line without its trailing newline
    /// @param: kind - parsed message meaning
    /// @return: engine message
    pub(super) fn new(raw: String, kind: EngineMessageKind) -> Self {
        Self { raw, kind }
    }

    /// raw returns the original engine output line.
    ///
    /// @return: raw engine output
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// kind returns the parsed message meaning.
    ///
    /// @return: parsed engine message kind
    pub const fn kind(&self) -> &EngineMessageKind {
        &self.kind
    }
}

/// `EngineMessageKind` describes engine-to-GUI UCI messages.
///
/// Unknown lines are retained because engines commonly emit extensions and
/// diagnostics alongside the standard protocol.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineMessageKind {
    Id {
        field: IdentityField,
        value: String,
    },
    Option(UciOption),
    UciOk,
    ReadyOk,
    Info(SearchInfo),
    BestMove {
        best_move: Option<String>,
        ponder: Option<String>,
    },
    Unknown,
}

/// `IdentityField` identifies a standard UCI engine identity value.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityField {
    Name,
    Author,
}

/// `UciOption` describes one engine configuration option.
///
/// @type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UciOption {
    pub name: String,
    pub kind: String,
    pub default: Option<String>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub variants: Vec<String>,
}

/// `SearchInfo` contains useful fields from a streaming UCI info message.
///
/// @type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchInfo {
    pub depth: Option<u16>,
    pub selective_depth: Option<u16>,
    pub elapsed: Option<DurationMillis>,
    pub nodes: Option<u64>,
    pub nodes_per_second: Option<u64>,
    pub hash_full: Option<u16>,
    pub multi_pv: Option<u16>,
    pub score: Option<Score>,
    pub current_move: Option<String>,
    pub principal_variation: Vec<String>,
    pub text: Option<String>,
}

/// `DurationMillis` is a UCI duration expressed in milliseconds.
///
/// @type
pub type DurationMillis = u64;

/// `Score` describes a centipawn or mate evaluation and its bound.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Score {
    pub value: ScoreValue,
    pub bound: Option<ScoreBound>,
}

/// `ScoreValue` describes the numeric meaning of a UCI score.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScoreValue {
    Centipawns(i32),
    Mate(i32),
}

/// `ScoreBound` describes whether a UCI score is exact or bounded.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScoreBound {
    Lower,
    Upper,
}
