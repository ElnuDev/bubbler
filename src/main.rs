use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

mod config;
use config::*;

fn main() {
    let context_settings = ContextSettings::default();
    let mut window = RenderWindow::new(
        VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 16),
        "bubbler",
        Style::TITLEBAR | Style::CLOSE,
        &context_settings,
    );

    window.display();

    while window.is_open() {
        loop {
            match window.poll_event() {
                Some(event) => match event {
                    Event::Closed | Event::KeyPressed {
                        code: Key::Q,
                        alt: _,
                        ctrl: true,
                        shift: _,
                        system: _,
                    } => window.close(),
                    _ => {}
                },
                None => break
            }
        }
    }
}
