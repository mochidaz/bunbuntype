use std::borrow::Borrow;
use std::collections::VecDeque;
use crate::loaders::loader::load_wordlist;
use crate::loaders::randomizer::randomizer;
use rand::seq::SliceRandom;
use std::path::Path;
use tui::widgets::TableState;
use crate::serializers::wpm_results::WpmResult;

pub enum InputMode {
    Normal,
    Editing,
    Typing,
}

#[derive(PartialEq)]
pub enum State {
    TypingTest,
    MainMenu,
    Chart,
}

pub enum TypingTestState {
    NotStarted,
    Running,
    End,
}

pub struct App<'a> {
    pub input_mode: InputMode,
    pub table_state: TableState,
    pub text_input: String,
    pub text_input_cursor: usize,
    pub text_input_history: Vec<String>,
    pub text_input_history_index: usize,
    pub items: Vec<Vec<&'a str>>,
    pub words: VecDeque<String>,
    pub timer: Option<tokio::time::Instant>,
    pub current_time: u64,
    pub state: State,
    pub correct_words: usize,
    pub incorrect_words: usize,
    pub typing_test_state: TypingTestState,
    pub wpm_results: Vec<(&'a str, u64)>,
}

impl<'a> App<'a> {
    pub fn new<T>(wordlist: T, result_file: T) -> Self
    where T: AsRef<Path> {
        let mut load_words = VecDeque::from(load_wordlist(wordlist));


        // The data for tui-rs bar chart is required to be a tuple of an &str a u64 which is why leaking is required
        let res = match WpmResult::from_file(&result_file) {
            Ok(f) => {
                f.iter()
                    .map(|wr| (Box::leak(wr.date_time
                        .to_string()
                        .into_boxed_str()) as &str, wr.awpm as u64))
                    .collect::<Vec<(&str, u64)>>()
            },
            Err(_) => {
                vec![("", 0)]
            }
        };

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
            current_time: 0,
            state: State::MainMenu,
            correct_words: 0,
            incorrect_words: 0,
            typing_test_state: TypingTestState::NotStarted,
            wpm_results: res,
        };

        instance.shuffle_words();

        instance
    }

    pub fn shuffle_words(&mut self) {
        let mut another_vec = Vec::from(self.words.clone());
        another_vec.shuffle(&mut rand::thread_rng());
        self.words = VecDeque::from(another_vec);
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
