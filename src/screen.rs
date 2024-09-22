use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    prelude::{Color, Span},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum InputEvent {
    Toggle,
    Copy,
    Quit,
    Tick,
}

struct App {
    last: usize,
    text: Vec<liz::write::Text>,
    notifications: String,
    running: bool,
    loader: usize,
    hearer: liz::hear::Hearer,
    writer: liz::write::Writer,
}

impl App {
    fn new(model: &str) -> Result<Self> {
        Ok(App {
            last: 0,
            text: Vec::new(),
            notifications: String::new(),
            running: false,
            loader: 0,
            hearer: liz::hear::Hearer::new()?,
            writer: liz::write::Writer::new(model)?,
        })
    }

    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn add_text(&mut self, new_text: &[liz::write::Text]) {
        self.last = self.text.last().map_or(0, |x| x.stop as usize);
        self.text = new_text.to_vec();
    }

    fn add_notification(&mut self, notification: &str) {
        self.notifications.push_str(notification);
        self.notifications.push('\n');
    }

    fn copy_to_clipboard(&mut self) -> Result<()> {
        let text = self
            .text
            .iter()
            .map(|x| x.text.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let mut clipboard = arboard::Clipboard::new()?;

        clipboard.set_text(text)?;

        Ok(())
    }

    fn update_loader(&mut self) {
        if self.running {
            self.loader = (self.loader + 1) % 4;
        }
    }
    fn reset_loader(&mut self) {
        self.loader = 0;
    }

    fn get_loader_text(&self) -> &str {
        match self.loader {
            0 => "",
            1 => ".",
            2 => "..",
            3 => "...",
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = std::time::Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char(' '), _) => tx.send(InputEvent::Toggle).unwrap(),
                        (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            tx.send(InputEvent::Quit).unwrap()
                        }
                        (KeyCode::Char('c'), _) => tx.send(InputEvent::Copy).unwrap(),

                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                tx.send(InputEvent::Tick).unwrap();
                last_tick = std::time::Instant::now();
            }
        }
    });

    let model_loc = std::env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("No model provided"))?;

    let mut app = App::new(&model_loc)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(25),
                        Constraint::Percentage(5),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let mut text = vec![];

            text.extend(app.text.iter().map(|x| {
                if (x.stop as usize) < app.last
                    || (((x.stop as usize) > app.last) && ((x.start as usize) < app.last))
                {
                    Line::from(x.to_string())
                } else {
                    Line::from(vec![Span::styled(
                        x.to_string(),
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Blue),
                    )])
                }
            }));
            text.push(Line::from(app.get_loader_text()));

            let text =
                Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Text"));
            f.render_widget(text, chunks[0]);

            let notifications = Paragraph::new(app.notifications.as_str()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Notifications"),
            );
            f.render_widget(notifications, chunks[1]);

            let controls = Line::from(vec![
                ratatui::text::Span::styled(
                    if app.running {
                        "Stop <space>"
                    } else {
                        "Start <space>"
                    },
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled(
                    "Copy <c>",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled(
                    "Quit <q>",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]);
            let controls = Paragraph::new(controls).alignment(ratatui::layout::Alignment::Center);
            f.render_widget(controls, chunks[2]);
        })?;

        match rx.recv()? {
            InputEvent::Toggle => {
                if app.running {
                    app.stop();
                    app.add_notification("Stopped text generation");
                    app.reset_loader();
                    let output = app.hearer.stop(|x, _| app.writer.generate_text(x))?;
                    app.add_text(&output);
                } else {
                    app.start();
                    app.hearer.start()?;
                    app.add_notification("Started text generation");
                }
            }
            InputEvent::Copy => {
                if let Err(err) = app.copy_to_clipboard() {
                    app.add_notification(&format!("Error copying to clipboard: {}", err));
                } else {
                    app.add_notification("Text copied to clipboard!");
                }
            }
            InputEvent::Quit => {
                break;
            }
            InputEvent::Tick => {
                app.update_loader();
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
