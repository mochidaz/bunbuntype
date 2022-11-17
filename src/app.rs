use crate::loaders::loader::load_wordlist;
use crate::loaders::randomizer::randomizer;
use rand::seq::SliceRandom;
use std::path::Path;
use tui::widgets::TableState;

pub enum InputMode {
    Normal,
    Editing,
    Typing,
}

pub struct App<'a> {
    pub input_mode: InputMode,
    pub table_state: TableState,
    pub text_input: String,
    pub text_input_cursor: usize,
    pub text_input_history: Vec<String>,
    pub text_input_history_index: usize,
    pub items: Vec<Vec<&'a str>>,
    pub words: Vec<String>,
    pub timer: Option<tokio::time::Instant>,
}

impl<'a> App<'a> {
    pub fn new(wordlist: impl AsRef<Path>) -> Self {
        let mut load_words = load_wordlist(wordlist);

        let mut instance = Self {
            input_mode: InputMode::Normal,
            table_state: TableState::default(),
            text_input: String::new(),
            text_input_cursor: 0,
            text_input_history: Vec::new(),
            text_input_history_index: 0,
            items: vec![vec!["Typing Test"], vec!["View Graph"]],
            words: load_words,
            timer: None,
        };

        instance.shuffle_words();

        instance
    }

    pub fn shuffle_words(&mut self) {
        self.words.shuffle(&mut rand::thread_rng());
    }

    pub fn start_timer(&mut self, duration: u64) {
        self.timer = Some(tokio::time::Instant::now());
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;
        });
    }

    pub fn up(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn down(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
