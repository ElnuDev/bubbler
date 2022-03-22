use sfml::system::*;
use sfml::graphics::*;
use sfml::window::*;

use crate::config::*;

pub struct Bubble<'a> {
	position: Vector2f,
	outline_circle: CircleShape<'a>,
	fill_circle: CircleShape<'a>,
	pub filled: bool,
}

impl<'a> Bubble<'a> {
	pub fn new(position: Vector2f, filled: bool) -> Self {
		Self {
			position,
			outline_circle: {
				let mut circle = CircleShape::new(BUBBLE_RADIUS, BUBBLE_POINT_COUNT);
				circle.set_origin(Vector2f::new(BUBBLE_RADIUS, BUBBLE_RADIUS));
				circle.set_position(position);
				circle.set_fill_color(Color::TRANSPARENT);
				circle.set_outline_thickness(2.0);
				circle.set_outline_color(BUBBLE_COLOR);
				circle
			},
			fill_circle: {
				const RADIUS: f32 = BUBBLE_RADIUS - BUBBLE_FILL_OFFSET;
				let mut circle = CircleShape::new(RADIUS, BUBBLE_POINT_COUNT);
				circle.set_origin(Vector2f::new(RADIUS, RADIUS));
				circle.set_position(position);
				circle.set_fill_color(BUBBLE_COLOR);
				circle
			},
			filled
		}
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		if self.filled {
			window.draw(&self.fill_circle);
		}
		window.draw(&self.outline_circle);
	}

	pub fn contains(&self, position: Vector2f) -> bool {
		let distance_squared = (self.position.x - position.x as f32).powi(2) + (self.position.y - position.y as f32).powi(2);
		distance_squared < BUBBLE_CLICK_RADIUS.powi(2)
	}

	pub fn click(&mut self, event: &Event) -> bool {
		match event {
			Event::MouseButtonPressed {
				button: mouse::Button::LEFT,
				x,
				y
			} => {
				if self.contains(Vector2f::new(*x as f32, *y as f32)) {
					self.filled = !self.filled;
					true
				} else {
					false
				}
			},
			_ => false
		}
	}
}