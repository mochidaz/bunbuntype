use rand::seq::SliceRandom;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Color::Rgb;
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table};
use tui::{Frame, Terminal};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::app::{App, InputMode};
use crate::error::ErrorKind;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), ErrorKind> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('t') => {
                        app.input_mode = InputMode::Typing;
                    }
                    KeyCode::Char('r') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Up => {
                        app.up();
                    }
                    KeyCode::Down => {
                        app.down();
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('r') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('s') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('d') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('c') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('m') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('n') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('p') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('u') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('l') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('f') => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn error<B: Backend>(f: &mut Frame<B>, error: &ErrorKind) {
    let block = Block::default()
        .title(format!("An error occured!"))
        .borders(Borders::ALL)
        .style(Style::default().bg(Rgb(0, 0, 0)).fg(Rgb(255, 0, 0)));
    let text = vec![Spans::from(format!("Error: {}", error))];
    let paragraph = Paragraph::new(text)
        .block(block.clone())
        .alignment(Alignment::Center);
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
    f.render_widget(block, area);
}

// write  a function to render the UI to set a timer with 3 choices. 1 minute, 3 minutes, and 5 minutes
// fn set_timer_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
//     let block = Block::default()
//         .title(format!("Set Timer"))
//         .borders(Borders::ALL)
//         .style(Style::default().bg(Rgb(0, 0, 0)).fg(Rgb(255, 0, 0)));
//     let text = vec![Spans::from(format!("Error: {}", error))];
//     let paragraph = Paragraph::new(text)
//         .block(block.clone())
//         .alignment(Alignment::Center);
//     let area = centered_rect(60, 20, f.size());
//     f.render_widget(Clear, area);
//     f.render_widget(paragraph, area);
//     f.render_widget(block, area);
// }

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(4),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let wrapper = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Rgb(255, 255, 255)))
        .title_alignment(Alignment::Center)
        .title(Spans::from(Span::styled(
            "Bunbuntype",
            Style::default().add_modifier(Modifier::BOLD),
        )));

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Rgb(144, 238, 144));
    let header_cells = ["Select Menu"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Rgb(0, 0, 0))));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(wrapper)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, chunks[0], &mut app.table_state);
}
