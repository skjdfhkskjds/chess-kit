use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::{App, Score, ScoreBound, ScoreValue};

/// render draws the latest engine evaluation and principal variation.
///
/// @param: frame - destination terminal frame
/// @param: area - analysis widget area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes the analysis widget into the frame buffer
pub(super) fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let analysis = app.analysis();
    let info = &analysis.info;
    let evaluation = info.score.map_or_else(|| "—".to_owned(), format_score);
    let depth = info
        .depth
        .map_or_else(|| "—".to_owned(), |value| value.to_string());
    let nodes = info.nodes.map_or_else(|| "—".to_owned(), format_count);
    let speed = info.nodes_per_second.map_or_else(
        || "—".to_owned(),
        |value| format!("{}/s", format_count(value)),
    );
    let elapsed = info
        .elapsed
        .map_or_else(|| "—".to_owned(), |value| format!("{value} ms"));
    let best_move = analysis.best_move.as_deref().unwrap_or("—");
    let principal_variation = if info.principal_variation.is_empty() {
        "—".to_owned()
    } else {
        info.principal_variation.join(" ")
    };
    let lines = vec![
        Line::from(format!("Eval {evaluation}   Depth {depth}")),
        Line::from(format!("Nodes {nodes}   NPS {speed}")),
        Line::from(format!("Time {elapsed}   Best {best_move}")),
        Line::from(""),
        Line::styled("Principal variation", Style::default().fg(Color::Cyan)),
        Line::from(principal_variation),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" Analysis ").borders(Borders::ALL)),
        area,
    );
}

/// format_score formats a UCI score for compact display.
///
/// @param: score - UCI score
/// @return: formatted evaluation
fn format_score(score: Score) -> String {
    let prefix = match score.bound {
        Some(ScoreBound::Lower) => "≥",
        Some(ScoreBound::Upper) => "≤",
        None => "",
    };
    match score.value {
        ScoreValue::Centipawns(value) => {
            format!("{prefix}{:+.2}", f64::from(value) / 100.0)
        }
        ScoreValue::Mate(value) => format!("{prefix}M{value:+}"),
    }
}

/// format_count formats a large integer using compact suffixes.
///
/// @param: value - count to format
/// @return: compact count
fn format_count(value: u64) -> String {
    match value {
        1_000_000.. => format!("{:.1}m", value as f64 / 1_000_000.0),
        1_000.. => format!("{:.1}k", value as f64 / 1_000.0),
        _ => value.to_string(),
    }
}
