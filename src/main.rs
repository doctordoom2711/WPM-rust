use rand::prelude::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    DefaultTerminal,
};
use std::time::{Duration, Instant};

struct App {
    target_text: String,
    input: String,
    start_time: Option<Instant>,
    finish_time: Option<Instant>,
    wpm: f64,
    is_finished: bool,
}

impl App {
    fn new() -> Self {
        let sentences = vec![
            "The quick brown fox jumps over the lazy dog.",
            "Rust is a systems programming language that runs blazingly fast.",
            "To be or not to be, that is the question.",
            "The terminal is a powerful tool for developers.",
            "Ratatui makes building terminal user interfaces easy.",
            "Hello! How are you doing today?",
            "I am fine, thank you for asking.",
            "Can you help me with this project?",
            "What is your favorite programming language?",
            "Where are you from originally?",
            "I am sorry for the delay.",
            "Can I have a glass of water, please?",
            "Do you speak any other languages?",
            "Thank you so much for your help.",
            "Please call me when you get a chance.",
            "How much does this new laptop cost?",
            "I do not understand the current error message.",
            "I do not know how to fix this bug yet.",
            "Can you repeat that instruction, please?",
            "I am looking for a specific library.",
            "What time is the meeting scheduled for?",
            "Can I see the menu before we order?",
            "Where can I buy a comfortable office chair?",
            "Can I pay with a credit card here?",
            "Do you have this shirt in a different size?",
            "Can I have the bill whenever you are ready?",
            "Can you take a picture of our team?",
            "Can I borrow your charger for a moment?",
            "Can you speak a little slower, please?",
            "Can you write that down so I don't forget?",
            "What do you mean by that specific term?",
            "How do you say this word in English?",
            "Is this the correct way to implement this?",
            "Can you translate this paragraph for me?",
            "Do you know what this error code means?",
            "I am still learning how to use Ratatui.",
            "What is the opposite of this function?",
            "Can you give me a synonym for that word?",
            "What is the past tense of the verb to run?",
            "How do you spell your last name correctly?",
            "Can you use this word in a full sentence?",
            "What is the definition of a closure in Rust?",
            "Can you tell me more about async programming?",
            "What do you think about the new update?",
            "What is your opinion on functional programming?",
            "What are your feelings about remote work?",
            "What are you doing later this evening?",
            "Do you have any pets at your home?",
            "Do you have any interesting hobbies?",
            "Do you like drinking coffee or tea?",
            "What is your favorite movie of all time?",
            "Do you have any siblings in your family?",
            "What do you do for a living these days?",
            "What is your current job profession?",
            "What is your favorite color to wear?",
            "What is your favorite season of the year?",
            "What is your favorite holiday to celebrate?",
            "What is your favorite way to relax?",
            "What is your favorite type of music?",
            "The sun rose early over the quiet hills.",
            "The birds sang loudly in the garden.",
            "She studied hard for her final exams.",
            "The dog barked at the passing car.",
            "I like eating fresh apples in autumn.",
            "She went to the market to buy vegetables.",
            "The boy played football with his friends.",
            "He woke up early to start his day.",
            "I am very happy with my progress.",
            "She exercises every morning before work.",
            "His dog barks loudly whenever someone knocks.",
            "My school starts at eight every morning.",
            "We always eat dinner together at night.",
            "They take the bus to work every day.",
            "He does not like eating green vegetables.",
            "I do not want anything to drink right now.",
            "This black dress is not very expensive.",
            "Those kids do not speak English very well.",
            "I went to the store to buy some bread.",
            "She took the test last Friday afternoon.",
            "We talked for hours about our future.",
            "The little girl played at the playground.",
            "He had a great time at the party.",
            "I did not know about the meeting today.",
            "He did not take a shower this morning.",
            "We did not have enough food for everyone.",
            "I will visit my parents next weekend.",
            "She will finish her project by tomorrow.",
            "They will go on vacation next month.",
            "He will start his new job next week.",
            "I am currently working on a new app.",
            "She is dancing gracefully on the stage.",
            "They are enjoying their trip to Hawaii.",
            "We are learning to play the acoustic guitar.",
            "He is studying hard for his upcoming exams.",
            "I am not feeling very well today.",
            "She is not attending the party tonight.",
            "They are not participating in the race.",
            "We are not going out for dinner tonight.",
            "He is not wearing a jacket in the cold.",
            "I was watching a movie late last night.",
        ];

        let mut rng = rand::rng();
        let target_text = sentences.choose(&mut rng).unwrap().to_string();

        Self {
            target_text,
            input: String::new(),
            start_time: None,
            finish_time: None,
            wpm: 0.0,
            is_finished: false,
        }
    }

    fn calculate_wpm(&mut self) {
        let now = self.finish_time.unwrap_or_else(Instant::now);
        if let Some(start) = self.start_time {
            let elapsed = now.duration_since(start).as_secs_f64() / 60.0;
            if elapsed > 0.0 {
                self.wpm = (self.input.len() as f64 / 5.0) / elapsed;
            }
        }
    }

    fn calculate_accuracy(&self) -> u32 {
        if self.input.is_empty() {
            return 100;
        }
        let correct = self
            .input
            .chars()
            .zip(self.target_text.chars())
            .filter(|(i, t)| i == t)
            .count();
        ((correct as f32 / self.input.len() as f32) * 100.0) as u32
    }

    fn check_finished(&mut self) {
        if self.input.trim() == self.target_text.trim() {
            self.is_finished = true;
            self.finish_time = Some(Instant::now());
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();

    loop {
        if !app.is_finished {
            app.calculate_wpm();
        }

        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Enter if app.is_finished => {
                            app = App::new();
                        }
                        KeyCode::Backspace if !app.is_finished => {
                            app.input.pop();
                        }
                        KeyCode::Char(c) if !app.is_finished => {
                            if app.start_time.is_none() {
                                app.start_time = Some(Instant::now());
                            }
                            if app.input.len() < app.target_text.len() {
                                app.input.push(c);
                                app.check_finished();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(area);

    // Header / Stats
    let stats_text = Line::from(vec![
        " WPM: ".yellow(),
        format!("{:.0}", app.wpm).bold().white(),
        " | Accuracy: ".cyan(),
        format!("{}%", app.calculate_accuracy()).bold().white(),
    ]);
    f.render_widget(
        Paragraph::new(stats_text).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Stats "),
        ),
        chunks[0],
    );

    // Main Typing Area
    let mut spans = Vec::new();
    for (i, target_char) in app.target_text.chars().enumerate() {
        if let Some(input_char) = app.input.chars().nth(i) {
            if input_char == target_char {
                spans.push(Span::styled(
                    target_char.to_string(),
                    Style::new().fg(Color::Green),
                ));
            } else {
                spans.push(Span::styled(
                    target_char.to_string(),
                    Style::new().bg(Color::Red).fg(Color::White),
                ));
            }
        } else {
            // Underline the next character to type
            if i == app.input.len() && !app.is_finished {
                spans.push(Span::styled(
                    target_char.to_string(),
                    Style::new()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                ));
            } else {
                spans.push(Span::styled(
                    target_char.to_string(),
                    Style::new().fg(Color::DarkGray),
                ));
            }
        }
    }

    let main_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(if app.is_finished {
            " FINISHED! "
        } else {
            " TYPE "
        })
        .border_style(if app.is_finished {
            Style::new().fg(Color::Green)
        } else {
            Style::new()
        });

    f.render_widget(
        Paragraph::new(Line::from(spans))
            .block(main_block)
            .wrap(Wrap { trim: true }),
        chunks[1],
    );

    // Footer
    let footer_text = if app.is_finished {
        "Press [ENTER] to restart | [ESC] to exit".green()
    } else {
        "Press [ESC] to exit".dim()
    };
    f.render_widget(Paragraph::new(footer_text).centered(), chunks[2]);
}
