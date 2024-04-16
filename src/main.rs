use std::io::stdout;
use std::time::Instant;
use chrono::Utc;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

enum StopwatchState {
    NotStarted,
    Running,
    Done
}

struct Stopwatch {
    now: Instant,
    state: StopwatchState,
    display: String,
}

impl Stopwatch {
    fn new() -> Self {
        Stopwatch {
            now: Instant::now(),
            state: StopwatchState::NotStarted,
            display: String::from("0:00:00")
        }
    }

    fn get_time(&self) -> String {
        match self.state {
            StopwatchState::NotStarted => String::from("0:00:00"),
            StopwatchState::Running => {
                let mut elapsed = self.now.elapsed().as_millis();
                let minutes = elapsed / 60000;
                elapsed -= minutes * 60000;
                let seconds = elapsed / 1000;
                elapsed -= seconds * 1000;
                let split_seconds = elapsed / 10;
                format!("{minutes}:{seconds}:{split_seconds}")
            }
            StopwatchState::Done => self.display.clone()
        }
    }

    fn next_state(&mut self) {
        match self.state {
            StopwatchState::NotStarted => {
                self.now = Instant::now();
                self.state = StopwatchState::Running;
            }
            StopwatchState::Running => {
                self.display = self.get_time();
                self.state = StopwatchState::Done;
            }
            StopwatchState::Done => {
                self.state = StopwatchState::NotStarted
            }
        }
    }
}

fn block_with(input: &str) -> Block {
    Block::default().title(input).borders(Borders::ALL)
}

fn utc_pretty() -> String {
    Utc::now().format("%Y/%m/%d %H:%M:%S").to_string()
}


fn main() -> Result<(), anyhow::Error>{
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut stopwatch = Stopwatch::new();

    loop {
        if poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key_event) = read()? {
                if let (KeyCode::Enter, KeyEventKind::Press) = (key_event.code, key_event.kind) {
                    stopwatch.next_state()
                }
            }
        }

        terminal
            .draw(|f| {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(f.size());
                let stopwatch_area = layout[0];
                let utc_time_area = layout[1];

                let stopwatch_block = block_with("Stopwatch");
                let utc_time_block = block_with("UTC Time");

                let stopwatch_text = Paragraph::new(stopwatch.get_time()).block(stopwatch_block);
                let utc_time_text = Paragraph::new(utc_pretty()).block(utc_time_block);

                f.render_widget(stopwatch_text,stopwatch_area);
                f.render_widget(utc_time_text, utc_time_area);
            })?;
        std::thread::sleep(std::time::Duration::from_millis(20));
        terminal.clear()?;
    }
}
