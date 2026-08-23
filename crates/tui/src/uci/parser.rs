use std::str::FromStr;

use chess_kit_comm::uci::UciMove;

use super::{
    EngineMessage, EngineMessageKind, IdentityField, Score, ScoreBound, ScoreValue, SearchInfo,
    UciOption,
};

impl FromStr for EngineMessage {
    type Err = std::convert::Infallible;

    /// from_str parses a UCI output line without rejecting engine extensions.
    ///
    /// @impl: FromStr::from_str
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let raw = line.trim_end_matches(['\r', '\n']).to_owned();
        let mut tokens = raw.split_whitespace();
        let kind = match tokens.next() {
            Some("id") => parse_id(tokens),
            Some("option") => EngineMessageKind::Option(parse_option(tokens.collect())),
            Some("uciok") => EngineMessageKind::UciOk,
            Some("readyok") => EngineMessageKind::ReadyOk,
            Some("info") => EngineMessageKind::Info(parse_info(tokens.collect())),
            Some("bestmove") => parse_best_move(tokens),
            _ => EngineMessageKind::Unknown,
        };
        Ok(Self::new(raw, kind))
    }
}

/// parse_id parses an engine identity message.
///
/// @param: tokens - identity field and value tokens
/// @return: parsed identity or Unknown
fn parse_id<'a>(mut tokens: impl Iterator<Item = &'a str>) -> EngineMessageKind {
    let field = match tokens.next() {
        Some("name") => IdentityField::Name,
        Some("author") => IdentityField::Author,
        _ => return EngineMessageKind::Unknown,
    };
    EngineMessageKind::Id {
        field,
        value: tokens.collect::<Vec<_>>().join(" "),
    }
}

/// parse_best_move parses a completed search response.
///
/// @param: tokens - best move and optional ponder tokens
/// @return: parsed best-move message
fn parse_best_move<'a>(mut tokens: impl Iterator<Item = &'a str>) -> EngineMessageKind {
    let best_move = tokens.next().and_then(parse_move);
    let ponder = match (tokens.next(), tokens.next()) {
        (Some("ponder"), Some(chess_move)) => parse_move(chess_move),
        _ => None,
    };
    EngineMessageKind::BestMove { best_move, ponder }
}

/// parse_move converts a wire move and treats the null move as no move.
///
/// @param: chess_move - UCI move text
/// @return: owned non-null move
fn parse_move(chess_move: &str) -> Option<String> {
    (chess_move != "0000" && chess_move != "(none)").then(|| chess_move.to_owned())
}

/// parse_option parses the well-known fields of an engine option.
///
/// @param: tokens - option tokens after the option keyword
/// @return: parsed option
fn parse_option(tokens: Vec<&str>) -> UciOption {
    let mut option = UciOption::default();
    let mut index = 0;
    while index < tokens.len() {
        match tokens[index] {
            "name" => {
                let (value, next) = collect_until(&tokens, index + 1, &["type"]);
                option.name = value;
                index = next;
            }
            "type" => {
                option.kind = tokens
                    .get(index + 1)
                    .copied()
                    .unwrap_or_default()
                    .to_owned();
                index += 2;
            }
            "default" => {
                let (value, next) = collect_until(&tokens, index + 1, &["min", "max", "var"]);
                option.default = Some(value);
                index = next;
            }
            "min" => {
                option.min = tokens.get(index + 1).and_then(|value| value.parse().ok());
                index += 2;
            }
            "max" => {
                option.max = tokens.get(index + 1).and_then(|value| value.parse().ok());
                index += 2;
            }
            "var" => {
                let (value, next) = collect_until(&tokens, index + 1, &["var", "min", "max"]);
                option.variants.push(value);
                index = next;
            }
            _ => index += 1,
        }
    }
    option
}

/// parse_info parses independently ordered search-information fields.
///
/// @param: tokens - info tokens after the info keyword
/// @return: parsed search information
fn parse_info(tokens: Vec<&str>) -> SearchInfo {
    let mut info = SearchInfo::default();
    let mut index = 0;
    while index < tokens.len() {
        match tokens[index] {
            "depth" => set_number(&tokens, &mut index, &mut info.depth),
            "seldepth" => set_number(&tokens, &mut index, &mut info.selective_depth),
            "time" => set_number(&tokens, &mut index, &mut info.elapsed),
            "nodes" => set_number(&tokens, &mut index, &mut info.nodes),
            "nps" => set_number(&tokens, &mut index, &mut info.nodes_per_second),
            "hashfull" => set_number(&tokens, &mut index, &mut info.hash_full),
            "multipv" => set_number(&tokens, &mut index, &mut info.multi_pv),
            "currmove" => {
                info.current_move = tokens.get(index + 1).map(ToString::to_string);
                index += 2;
            }
            "score" => {
                let (score, next) = parse_score(&tokens, index + 1);
                info.score = score;
                index = next;
            }
            "pv" => {
                let mut end = index + 1;
                while end < tokens.len() && UciMove::from_str(tokens[end]).is_ok() {
                    end += 1;
                }
                info.principal_variation = tokens[index + 1..end]
                    .iter()
                    .map(ToString::to_string)
                    .collect();
                index = end;
            }
            "string" => {
                info.text = Some(tokens[index + 1..].join(" "));
                break;
            }
            _ => index += 1,
        }
    }
    info
}

/// set_number parses the value following a numeric info key.
///
/// @marker: NumberT - numeric destination type
/// @param: tokens - complete info token list
/// @param: index - current key position
/// @param: destination - field receiving a successfully parsed value
/// @return: void
/// @side-effects: advances index and may update destination
fn set_number<NumberT>(tokens: &[&str], index: &mut usize, destination: &mut Option<NumberT>)
where
    NumberT: FromStr,
{
    *destination = tokens.get(*index + 1).and_then(|value| value.parse().ok());
    *index += 2;
}

/// parse_score parses a UCI score value and optional bound.
///
/// @param: tokens - complete info token list
/// @param: index - score type position
/// @return: parsed score and next unconsumed position
fn parse_score(tokens: &[&str], index: usize) -> (Option<Score>, usize) {
    let value = match (tokens.get(index), tokens.get(index + 1)) {
        (Some(&"cp"), Some(value)) => value.parse().ok().map(ScoreValue::Centipawns),
        (Some(&"mate"), Some(value)) => value.parse().ok().map(ScoreValue::Mate),
        _ => None,
    };
    let mut next = index + 2;
    let bound = match tokens.get(next) {
        Some(&"lowerbound") => {
            next += 1;
            Some(ScoreBound::Lower)
        }
        Some(&"upperbound") => {
            next += 1;
            Some(ScoreBound::Upper)
        }
        _ => None,
    };
    (value.map(|value| Score { value, bound }), next)
}

/// collect_until joins tokens until the next recognized field marker.
///
/// @param: tokens - complete token list
/// @param: start - first value position
/// @param: markers - fields that terminate the value
/// @return: joined value and next marker position
fn collect_until(tokens: &[&str], start: usize, markers: &[&str]) -> (String, usize) {
    let mut end = start;
    while end < tokens.len() && !markers.contains(&tokens[end]) {
        end += 1;
    }
    (tokens[start..end].join(" "), end)
}
