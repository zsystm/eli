// src/ui.rs

use ratatui::{
  Frame,
  layout::{Constraint, Direction, Layout},
  style::{Color, Style},
  widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app::{App, AppMode};

/// Top-level dispatch: draw according to current AppMode
pub fn draw_ui(f: &mut Frame, app: &mut App) {
  match app.mode {
      AppMode::Main       => draw_main_mode(f, app),
      AppMode::ParamInput => draw_param_input_mode(f, app),
      AppMode::History    => draw_history_mode(f, app),
  }
}

fn draw_main_mode(f: &mut Frame, app: &mut App) {
  let area = f.area();
  let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
      .split(area);

  // 1) Search box (string slice to avoid type ambiguity)
  let search = Paragraph::new(app.search_input.as_str())
      .block(Block::default().title("Search").borders(Borders::ALL));
  f.render_widget(search, chunks[0]);

  // 2) Methods list
  let items: Vec<ListItem> = app
      .filtered_methods
      .iter()
      .map(|m| ListItem::new(m.clone()))
      .collect();

  let list = List::new(items)
      .block(Block::default().title("Methods").borders(Borders::ALL))
      .highlight_style(Style::default().fg(Color::Yellow));

  f.render_stateful_widget(list, chunks[1], &mut app.methods_state);
}

fn draw_param_input_mode(f: &mut Frame, app: &mut App) {
  let area = f.area();
  let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
          Constraint::Length(3),
          Constraint::Length(3),
          Constraint::Min(0),
      ].as_ref())
      .split(area);

  // Param 1
  let p1 = app.param_inputs.get(0).map(|s| s.as_str()).unwrap_or("");
  let input1 = Paragraph::new(p1)
      .block(Block::default().title("Param 1").borders(Borders::ALL));
  f.render_widget(input1, chunks[0]);

  // Param 2
  let p2 = app.param_inputs.get(1).map(|s| s.as_str()).unwrap_or("");
  let input2 = Paragraph::new(p2)
      .block(Block::default().title("Param 2").borders(Borders::ALL));
  f.render_widget(input2, chunks[1]);

  // Instructions
  let help = Paragraph::new("Enter=Send • Esc=Back")
      .block(Block::default().title("Help").borders(Borders::ALL));
  f.render_widget(help, chunks[2]);
}

fn draw_history_mode(f: &mut Frame, app: &mut App) {
  let area = f.area();
  let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
      .split(area);

  // History list items
  let items: Vec<ListItem> = app
      .history
      .iter()
      .enumerate()
      .map(|(i, (req, res))| {
          let line = format!("{}: {} → {:?}", i, req.method, res.result);
          ListItem::new(line)
      })
      .collect();

  let list = List::new(items)
      .block(Block::default().title("History").borders(Borders::ALL))
      .highlight_style(Style::default().fg(Color::Yellow));

  f.render_stateful_widget(list, chunks[0], &mut app.history_state);

  // Instructions
  let help = Paragraph::new("↑/↓=Navigate • Enter=Load • Esc=Back")
      .block(Block::default().title("Help").borders(Borders::ALL));
  f.render_widget(help, chunks[1]);
}
