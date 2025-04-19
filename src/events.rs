// src/events.rs

use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle key events in Main mode:
/// - Ctrl+C: quit
/// - Character keys: append to search_input and filter methods
/// - Backspace: remove last char and filter methods
/// - Arrow keys: navigate filtered_methods list
/// - Enter: switch to ParamInput mode and initialize param_inputs
/// - 'h': switch to History mode
pub async fn handle_main_mode(app: &mut App, key: KeyEvent) {
    match key {
        // Ctrl+C to quit
        KeyEvent { code: KeyCode::Char('c'), modifiers, .. } if modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        // Printable characters add to search input
        KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. } if !c.is_control() => {
            app.search_input.push(c);
            app.filter_methods();
        }
        // Backspace deletes last character
        KeyEvent { code: KeyCode::Backspace, .. } => {
            app.search_input.pop();
            app.filter_methods();
        }
        // Navigate up in the filtered methods list
        KeyEvent { code: KeyCode::Up, .. } => {
            let i = app.methods_state.selected().unwrap_or(0);
            if i > 0 {
                app.methods_state.select(Some(i - 1));
            }
        }
        // Navigate down in the filtered methods list
        KeyEvent { code: KeyCode::Down, .. } => {
            let i = app.methods_state.selected().unwrap_or(0);
            if i + 1 < app.filtered_methods.len() {
                app.methods_state.select(Some(i + 1));
            }
        }
        // Enter to go to ParamInput mode
        KeyEvent { code: KeyCode::Enter, .. } => {
            app.param_inputs = vec!["".to_string(), "".to_string()];
            app.mode = AppMode::ParamInput;
        }
        // 'h' goes to History mode
        KeyEvent { code: KeyCode::Char('h'), modifiers: KeyModifiers::NONE, .. } => {
            app.mode = AppMode::History;
        }
        _ => {}
    }
}

/// Handle key events in ParamInput mode:
/// - Ctrl+C: quit
/// - Esc: return to Main mode
/// - Enter: send request & return to Main mode
/// - Character keys: append to first parameter
/// - Backspace: remove last char from first parameter
pub async fn handle_param_input_mode(app: &mut App, key: KeyEvent) {
    match key {
        // Ctrl+C to quit
        KeyEvent { code: KeyCode::Char('c'), modifiers, .. } if modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        // Esc to return to Main mode
        KeyEvent { code: KeyCode::Esc, .. } => {
            app.mode = AppMode::Main;
        }
        // Enter also returns to Main mode
        KeyEvent { code: KeyCode::Enter, .. } => {
            app.mode = AppMode::Main;
        }
        // Printable characters: append to first parameter
        KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. } if !c.is_control() => {
            if let Some(field) = app.param_inputs.get_mut(0) {
                field.push(c);
            }
        }
        // Backspace: remove last char from first parameter
        KeyEvent { code: KeyCode::Backspace, .. } => {
            if let Some(field) = app.param_inputs.get_mut(0) {
                field.pop();
            }
        }
        _ => {}
    }
}

/// Handle key events in History mode:
/// - Ctrl+C: quit
/// - Esc: return to Main mode
/// - Arrow keys: navigate history list
/// - Enter: reload selected request into ParamInput mode
pub async fn handle_history_mode(app: &mut App, key: KeyEvent) {
    match key {
        // Ctrl+C to quit
        KeyEvent { code: KeyCode::Char('c'), modifiers, .. } if modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        // Esc to return to Main mode
        KeyEvent { code: KeyCode::Esc, .. } => {
            app.mode = AppMode::Main;
        }
        // Navigate up in history list
        KeyEvent { code: KeyCode::Up, .. } => {
            let i = app.history_state.selected().unwrap_or(0);
            if i > 0 {
                app.history_state.select(Some(i - 1));
            }
        }
        // Navigate down in history list
        KeyEvent { code: KeyCode::Down, .. } => {
            let i = app.history_state.selected().unwrap_or(0);
            if i + 1 < app.history.len() {
                app.history_state.select(Some(i + 1));
            }
        }
        // Reload selected history entry
        KeyEvent { code: KeyCode::Enter, .. } => {
            if let Some((req, _)) = app.history.get(app.history_state.selected().unwrap_or(0)) {
                // Reset filtered_methods and selection
                app.filtered_methods = app.all_methods.clone();
                if let Some(idx) = app.all_methods.iter().position(|m| m == &req.method) {
                    app.methods_state.select(Some(idx));
                }
                // Load parameters
                app.param_inputs = req.params.as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.to_string())
                    .collect();
                app.mode = AppMode::ParamInput;
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, AppMode};
    use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

    #[tokio::test]
    async fn typing_char_in_main_mode_filters_methods() {
        let mut app = App::new();
        app.all_methods = vec!["foo".to_string(), "bar".to_string()];
        app.filtered_methods = app.all_methods.clone();

        // Press 'b'
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE)).await;
        assert_eq!(app.search_input, "b");
        assert_eq!(app.filtered_methods, vec!["bar"]);
    }

    #[tokio::test]
    async fn backspace_in_main_mode_removes_char() {
        let mut app = App::new();
        app.search_input = "ab".to_string();
        app.filter_methods();
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)).await;
        assert_eq!(app.search_input, "a");
    }

    #[tokio::test]
    async fn arrows_navigate_selection() {
        let mut app = App::new();
        app.filtered_methods = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        app.methods_state.select(Some(1));

        // Up arrow
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)).await;
        assert_eq!(app.methods_state.selected(), Some(0));

        // Down arrow twice
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)).await;
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)).await;
        assert_eq!(app.methods_state.selected(), Some(2));
    }

    #[tokio::test]
    async fn enter_switches_to_param_input_mode() {
        let mut app = App::new();
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)).await;
        assert_eq!(app.mode, AppMode::ParamInput);
        assert_eq!(app.param_inputs.len(), 2);
    }

    #[tokio::test]
    async fn h_switches_to_history_mode() {
        let mut app = App::new();
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE)).await;
        assert_eq!(app.mode, AppMode::History);
    }

    #[tokio::test]
    async fn ctrl_c_sets_should_quit() {
        let mut app = App::new();
        handle_main_mode(&mut app, KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)).await;
        assert!(app.should_quit);
    }
}