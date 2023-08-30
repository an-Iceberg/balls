use macroquad::{
  prelude::{clamp, Color, Vec2},
  time::get_frame_time,
  window::{screen_width, screen_height},
};
use rand::Rng;

pub struct Ball
{ position: Vec2,
  velocity: Vec2,
  radius: f32,
  color: Color,
}

impl Ball
{ pub fn new() -> Self
  { let position = Vec2::new(screen_width() / 2., screen_height() / 2.);
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

  pub fn update(&mut self)
  { self.position += self.velocity * get_frame_time();

    // Reverse x direction if the border is hit
    if self.position.x < self.radius || self.position.x > (screen_width() - self.radius)
    { clamp( self .position .x, self.radius, screen_width() - self.radius,);
      self.velocity.x *= -1.;
    }

    // Same for y direction
    if self.position.y < self.radius || self.position.y > (screen_height() - self.radius)
    { clamp( self .position .y, self.radius, screen_height() - self.radius,);
      self.velocity.y *= -1.;
    }
  }

  pub fn x(&self) -> f32
  { return self.position.x;
  }

  pub fn y(&self) -> f32
  { return self.position.y;
  }

  pub fn radius(&self) -> f32
  { return self.radius;
  }

  pub fn color(&self) -> Color
  { return self.color;
  }
}

#[cfg(test)]
mod tests {}
