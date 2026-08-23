use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{Action, App, Direction};

/// action maps a terminal key event to an application action.
///
/// @param: key - terminal key event
/// @param: app - current application state
/// @return: mapped action, or None for an unused key
pub(super) fn action(key: KeyEvent, app: &App) -> Option<Action> {
    if key.kind == KeyEventKind::Release {
        return None;
    }
    let navigation = matches!(
        key.code,
        KeyCode::Up
            | KeyCode::Down
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Char('h' | 'j' | 'k' | 'l')
    );
    if key.kind == KeyEventKind::Repeat && !navigation {
        return None;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('?') => Some(Action::ToggleHelp),
        KeyCode::Esc if app.show_help() => Some(Action::ToggleHelp),
        KeyCode::Esc => Some(Action::CancelSelection),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveCursor(Direction::Up)),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::MoveCursor(Direction::Down)),
        KeyCode::Left | KeyCode::Char('h') => Some(Action::MoveCursor(Direction::Left)),
        KeyCode::Right | KeyCode::Char('l') => Some(Action::MoveCursor(Direction::Right)),
        KeyCode::Enter => Some(Action::SelectSquare),
        KeyCode::Char(' ') => Some(Action::ToggleAnalysis),
        KeyCode::Char('f') => Some(Action::FlipBoard),
        KeyCode::Char('n') => Some(Action::NewGame),
        KeyCode::Char('p') => Some(Action::ToggleProtocol),
        _ => None,
    }
}
