// src/app.rs

use ratatui::widgets::ListState;

/// Represents the current UI mode of the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// Main mode: search and select JSON-RPC methods.
    Main,
    /// Parameter input mode: fill in method parameters.
    ParamInput,
    /// History mode: browse and reload previous requests.
    History,
}

/// Application state shared across the TUI.
pub struct App {
    /// Current UI mode.
    pub mode: AppMode,
    /// Flag to indicate when the app should quit.
    pub should_quit: bool,

    /// Current search string for filtering methods.
    pub search_input: String,
    /// Full list of available JSON-RPC methods.
    pub all_methods: Vec<String>,
    /// Filtered list of methods matching `search_input`.
    pub filtered_methods: Vec<String>,
    /// Stateful selection index for the methods list.
    pub methods_state: ListState,

    /// Current parameter inputs for the selected method.
    pub param_inputs: Vec<String>,

    /// History of (request, response) pairs.
    pub history: Vec<(crate::rpc::JsonRpcRequest, crate::rpc::JsonRpcResponse)>,
    /// Stateful selection index for the history list.
    pub history_state: ListState,
}

impl App {
    /// Constructs a new `App` with default values.
    pub fn new() -> Self {
        let all_methods = vec![
            "eth_blockNumber".to_string(),
            "eth_getBalance".to_string(),
            "eth_gasPrice".to_string(),
            "eth_call".to_string(),
            // ... add more methods as needed
        ];

        let mut methods_state = ListState::default();
        methods_state.select(Some(0));

        let mut history_state = ListState::default();
        history_state.select(Some(0));

        let filtered_methods = all_methods.clone();

        App {
            mode: AppMode::Main,
            should_quit: false,
            search_input: String::new(),
            all_methods,
            filtered_methods,
            methods_state,
            param_inputs: Vec::new(),
            history: Vec::new(),
            history_state,
        }
    }

    /// Filters `all_methods` by the current `search_input`, updating `filtered_methods` and resetting selection.
    pub fn filter_methods(&mut self) {
        let query = self.search_input.to_lowercase();
        if query.is_empty() {
            self.filtered_methods = self.all_methods.clone();
        } else {
            self.filtered_methods = self
                .all_methods
                .iter()
                .filter(|m| m.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
        // Reset selection index
        self.methods_state.select(Some(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_new_initializes_correctly() {
        let app = App::new();
        // Default mode is Main
        assert_eq!(app.mode, AppMode::Main);
        // Quit flag should be false
        assert!(!app.should_quit);
        // Search input should be empty
        assert_eq!(app.search_input, "");
        // Filtered methods should match all methods initially
        assert_eq!(app.filtered_methods, app.all_methods);
        // First method selected by default
        assert_eq!(app.methods_state.selected(), Some(0));
        // No parameters by default
        assert!(app.param_inputs.is_empty());
        // History empty by default
        assert!(app.history.is_empty());
        // History selection should be zero
        assert_eq!(app.history_state.selected(), Some(0));
    }

    #[test]
    fn filter_methods_filters_by_query() {
        let mut app = App::new();
        // Setup a known method list
        app.all_methods = vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ];
        // Case-insensitive filter
        app.search_input = "B".to_string();
        app.filter_methods();
        assert_eq!(app.filtered_methods, vec!["bar", "baz"]);
        // Selection resets to 0
        assert_eq!(app.methods_state.selected(), Some(0));

        // Empty query resets
        app.search_input.clear();
        app.filter_methods();
        assert_eq!(app.filtered_methods, app.all_methods);
    }
}
