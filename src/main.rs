mod ball;

use ball::Ball;
use egui_macroquad::{ui, egui::{Window, Align2, Slider}, draw};
use macroquad::prelude::*;

pub fn window_config() -> Conf {
  return Conf {
    window_title: "Balls".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Conf::default()
  };
}

#[macroquad::main(window_config)]
async fn main()
{ let mut balls: Vec<Ball> = vec![];
  let mut balls_amount = 1_u32;

  loop
  { clear_background(BLACK);

    // Draws balls
    balls.iter_mut()
      .for_each(|ball|
      { ball.update();
        draw_circle(ball.x(), ball.y(), ball.radius(), ball.color());
      });

    ui(|egui_context|
    { Window::new("")
        .title_bar(false)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, egui_macroquad::egui::Vec2::new(-10., 10.))
        .show(egui_context, |ui|
        { ui.horizontal(|ui|
          { if ui.button(if balls_amount == 1 { "Add ball" } else { "Add balls" }).clicked()
            { for _ in 0..balls_amount
              { balls.push(Ball::new());
              }
            }

            if ui.button(if balls_amount == 1 { "Remove ball" } else { "Remove balls" }).clicked()
            { for _ in 0..balls_amount
              { if balls.len() <= 0 { break; }
                balls.remove(0);
              }
            }

            if ui.button("Clear all balls").clicked(){ balls.clear(); }
          });

          ui.horizontal(|ui|
          { ui.label("Amount of balls:");
            ui.add(Slider::new(&mut balls_amount, 1..=30));
          });

          ui.horizontal(|ui| {
            ui.label("v1.0.0");
            ui.separator();
            ui.label(format!("FPS:{}", get_fps()));
          });
        });
    });

    draw();

    next_frame().await;
  }
}
