use crate::event::{AppEvent, Event, EventHandler};
use crate::filter::NormalFilter;
use crate::visualizer::Visualizer;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Application.
// #[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Visualizer Widget
    pub visualizer: Visualizer<NormalFilter>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            visualizer: Default::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                // Event::Crossterm(event) => match event {
                //     crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                //     _ => {}
                // },
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?;
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::AddBar => self.visualizer.add_bar(),
                    AppEvent::RemoveBar => self.visualizer.remove_bar(),
                    AppEvent::IncrementScale => self.visualizer.increment_scale(),
                    AppEvent::DecrementScale => self.visualizer.decrement_scale(),
                    AppEvent::ShowStats => {
                        self.visualizer.show_stats = true;
                        self.visualizer.show_keys = false;
                    }
                    AppEvent::ShowKeys => {
                        self.visualizer.show_stats = false;
                        self.visualizer.show_keys = true;
                    }
                    AppEvent::ClosePopup => {
                        self.visualizer.show_stats = false;
                        self.visualizer.show_keys = false;
                    }
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Char('l') => self.events.send(AppEvent::AddBar),
            KeyCode::Char('h') => self.events.send(AppEvent::RemoveBar),
            KeyCode::Char('k') => self.events.send(AppEvent::IncrementScale),
            KeyCode::Char('j') => self.events.send(AppEvent::DecrementScale),
            KeyCode::Char('s') => self.events.send(AppEvent::ShowStats),
            KeyCode::Char('?') => self.events.send(AppEvent::ShowKeys),
            KeyCode::Char('c') => self.events.send(AppEvent::ClosePopup),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        self.visualizer.update();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
