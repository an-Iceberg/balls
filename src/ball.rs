use macroquad::{
  prelude::{Color, Vec2},
  time::get_frame_time,
  window::{screen_width, screen_height},
};
use rand::Rng;

pub struct Ball
{
  position: Vec2,
  velocity: Vec2,
  radius: f32,
  color: Color,
}

impl Ball
{
  #[allow(clippy::needless_return)]
  pub fn new() -> Self
  {
    // Put ball in centre of the screen
    let position = Vec2::new(screen_width() / 2., screen_height() / 2.);
    // Everything else is randomly chosen
    let velocity = Vec2::new(
      rand::thread_rng().gen_range(-400_f32..=400_f32),
      rand::thread_rng().gen_range(-400_f32..=400_f32),
    );
    let radius = rand::thread_rng().gen_range(1_f32..=40_f32);
    let color = Color::from_rgba(
      rand::thread_rng().gen_range(0_u8..=255_u8),
      rand::thread_rng().gen_range(0_u8..=255_u8),
      rand::thread_rng().gen_range(0_u8..=255_u8),
      255
    );

    return Ball { position, velocity, radius, color };
  }

  // Collision
  pub fn update(&mut self)
  {
    self.position += self.velocity * get_frame_time();

    // Left wall
    if self.position.x < self.radius
    {
      if self.velocity.x < 0.
      { self.velocity.x *= -1.; }
      self.position.x = self.radius;
    }

    // Right wall
    if self.position.x > (screen_width() - self.radius)
    {
      if self.velocity.x > 0.
      { self.velocity.x *= -1.; }
      self.position.x = screen_width() - self.radius;
    }

    // Top wall
    if self.position.y < self.radius
    {
      if self.velocity.y < 0.
      { self.velocity.y *= -1.; }
      self.position.y = self.radius;
    }

    // Bottom wall
    if self.position.y > (screen_height() - self.radius)
    {
      if self.velocity.y > 0.
      { self.velocity.y *= -1.; }
      self.position.y = screen_height() - self.radius;
    }
  }

  #[allow(clippy::needless_return)]
  pub fn x(&self) -> f32
  { return self.position.x; }

  #[allow(clippy::needless_return)]
  pub fn y(&self) -> f32
  { return self.position.y; }

  #[allow(clippy::needless_return)]
  pub fn radius(&self) -> f32
  { return self.radius; }

  #[allow(clippy::needless_return)]
  pub fn color(&self) -> Color
  { return self.color; }
}

#[cfg(test)]
mod tests {}
