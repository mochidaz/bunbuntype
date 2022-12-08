use rand::seq::SliceRandom;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Color::Rgb;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BarChart, Block, Borders, Cell, Clear, Paragraph, Row, Table};
use tui::{Frame, Terminal};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use unicode_width::UnicodeWidthStr;

use crate::app::{App, InputMode, State, TypingTestState};
use crate::error::ErrorKind;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), ErrorKind> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let TypingTestState::Running = app.typing_test_state {
            if app.current_time == 0 {
                app.typing_test_state = TypingTestState::End;
            }
        }

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Up => {
                        app.up();
                    }
                    KeyCode::Down => {
                        app.down();
                    }
                    KeyCode::Enter => {
                        if app.state == State::MainMenu {
                            match app.table_state.selected() {
                                Some(0) => {
                                    app.state = State::TypingTest;
                                }
                                Some(1) => {
                                    app.state = State::Chart;
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Char('b') => {
                        app.state = State::MainMenu;
                    }
                    _ => {}
                },
                InputMode::Typing => match key.code {
                    KeyCode::Char(c) => {
                        if c.is_whitespace() {
                            app.text_input.clear()
                        } else {
                            app.text_input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        let input = app.text_input.drain(..).collect::<String>();

                        let current_word = app.words.pop_front().unwrap();

                        if input == current_word {
                            app.words.push_back(current_word);
                            app.correct_words += 1;
                        } else {
                            app.words.push_back(current_word);
                            app.incorrect_words += 1;
                        }

                        app.text_input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
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

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.state {
        State::TypingTest => {

            match app.typing_test_state {
                TypingTestState::End => {
                    println!("ENDED");
                }
                _ => {}
            }

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Min(2),
                        Constraint::Length(2),
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

            f.render_widget(wrapper, chunks[0]);

            let input = Paragraph::new(app.text_input.as_ref())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default().fg(Color::Red),
                    InputMode::Typing => Style::default().fg(Color::Blue),
                    _ => Style::default().fg(Color::Green),
                })
                .block(Block::default().borders(Borders::ALL).title(Span::styled(
                    "Input",
                    Style::default().add_modifier(Modifier::BOLD),
                )));
            f.render_widget(input, chunks[2]);
            match app.input_mode {
                InputMode::Normal => {}

                InputMode::Typing => f.set_cursor(
                    chunks[2].x + app.text_input.width() as u16 + 1,
                    chunks[2].y + 1,
                ),
                _ => {}
            }
        }

        State::MainMenu => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
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
        State::Chart => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let barchart = BarChart::default()
                .block(Block::default().title("WPM Data").borders(Borders::ALL))
                .data(&app.wpm_results)
                .bar_width(9)
                .bar_style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
            f.render_widget(barchart, chunks[0]);
        }
    }
}
