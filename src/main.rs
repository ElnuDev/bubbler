use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

mod structs;
use structs::*;

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
    window.set_framerate_limit(FPS);

    let mut bubbles = Vec::new();
    bubbles.push(Bubble::new(Vector2f::new(64.0, 64.0), false));
    bubbles.push(Bubble::new(Vector2f::new(128.0, 64.0), true));
    bubbles.push(Bubble::new(Vector2f::new(192.0, 64.0), false));

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
                    Event::MouseButtonPressed { button: _, x: _, y: _ } => {
                        for bubble in bubbles.iter_mut() {
                            if bubble.click(&event) {
                                break;
                            }
                        }
                    },
                    _ => {}
                },
                None => break
            }
        }
        window.clear(Color::BLACK);

        for bubble in bubbles.iter() {
            bubble.draw(&mut window);
        }
    
        window.display();
    }
}
