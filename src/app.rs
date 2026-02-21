use crate::event::{AppEvent, Event, EventHandler};
use crate::visualize::visual::{Mode, Visual};
use ratatui::widgets::WidgetRef;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Application.
// #[derive(Debug)]
pub struct App<T: Visual + WidgetRef> {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// Visualizer Widget
    pub visualizer: T,
}

// pub trait Visualizer: Visual + WidgetRef {
//     fn get_mode(&self) -> Mode;
//     fn set_mode(&mut self, mode: Mode);
// }
//
// impl Visualizer for MonoVisualizer {
//     fn get_mode(&self) -> Mode {
//         self.mode
//     }
//
//     fn set_mode(&mut self, mode: Mode) {
//         self.mode = mode;
//     }
// }
//
// impl Visualizer for StereoVisualizer {
//     fn get_mode(&self) -> Mode {
//         self.mode
//     }
//
//     fn set_mode(&mut self, mode: Mode) {
//         self.mode = mode;
//     }
// }
impl<T: Visual + WidgetRef + Default> Default for App<T> {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            visualizer: Default::default(),
        }
    }
}

impl<T: Visual + WidgetRef + Default> App<T> {
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
                        // self.visualizer.mode = Mode::ShowStats;
                        self.visualizer.set_mode(Mode::ShowStats);
                    }
                    AppEvent::ShowKeys => {
                        // self.visualizer.mode = Mode::ShowKeys;
                        self.visualizer.set_mode(Mode::ShowKeys);
                    }
                    AppEvent::ShowColors => {
                        // self.visualizer.mode = Mode::ColorPick;
                        self.visualizer.set_mode(Mode::ColorPick);
                    }
                    AppEvent::ClosePopup => {
                        // self.visualizer.mode = Mode::Default;
                        self.visualizer.set_mode(Mode::Default);
                    }
                    AppEvent::NextColor => {
                        self.visualizer.next_color();
                    }
                    AppEvent::PrevColor => {
                        self.visualizer.prev_color();
                    }
                    AppEvent::ShowInput => {
                        // self.visualizer.mode = Mode::ShowInput;
                        self.visualizer.set_mode(Mode::ShowInput);
                    }
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        let mode = &self.visualizer.get_mode();
        match (mode, key_event.code) {
            (Mode::Default, KeyCode::Char('q')) => self.events.send(AppEvent::Quit),
            (_, KeyCode::Char('c' | 'C')) if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            (Mode::Default, KeyCode::Char('l')) => self.events.send(AppEvent::AddBar),
            (Mode::Default, KeyCode::Char('h')) => self.events.send(AppEvent::RemoveBar),
            (Mode::Default, KeyCode::Char('k')) => self.events.send(AppEvent::IncrementScale),
            (Mode::Default, KeyCode::Char('j')) => self.events.send(AppEvent::DecrementScale),
            (Mode::Default, KeyCode::Char('s')) => self.events.send(AppEvent::ShowStats),
            (Mode::Default, KeyCode::Char('?')) => self.events.send(AppEvent::ShowKeys),
            (Mode::Default, KeyCode::Char('c')) => self.events.send(AppEvent::ShowColors),
            (Mode::Default, KeyCode::Char('i')) => self.events.send(AppEvent::ShowInput),
            (_, KeyCode::Esc) => self.events.send(AppEvent::ClosePopup),
            (Mode::ColorPick, KeyCode::Char('j')) => self.events.send(AppEvent::NextColor),
            (Mode::ColorPick, KeyCode::Char('k')) => self.events.send(AppEvent::PrevColor),
            (Mode::ColorPick, KeyCode::Enter) => self.events.send(AppEvent::ClosePopup),
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

    // pub fn set_vis(&mut self, vis: Box<dyn Visualizer>) {
    //     self.visualizer = vis
    // }

    // pub fn toggle_channels(&mut self) {
    // let chans = self.visualizer.channels();
    // self.visualizer = match chans {
    //     1 => Box::new(StereoVisualizer::default()),
    //     2 => Box::new(MonoVisualizer::default()),
    //     _ => return,
    // }
    // self.visualizer = Box::new(MonoVisualizer::default());
    // }
}
