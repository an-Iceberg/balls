mod ball;

use ball::Ball;
use egui_macroquad::{draw, egui::{Align2, ComboBox, Vec2, Window}, ui};
use macroquad::prelude::*;

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

#[allow(clippy::needless_return)]
pub fn window_config() -> Conf
{
  return Conf
  {
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
{
  let mut balls: Vec<Ball> = vec![];
  let mut balls_amount = 1_u32;

  loop
  {
    clear_background(BLACK);

    balls.iter_mut()
      .for_each(|ball|
      {
        ball.update();
        draw_circle(ball.x(), ball.y(), ball.radius(), ball.color());
      });

    ui(|egui_context|
    {
      Window::new("")
        .movable(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, egui_macroquad::egui::Vec2::new(-10., 10.))
        .fixed_size(Vec2::new(0., 0.))
        .show(egui_context, |ui|
        {
          ui.horizontal(|ui|
          {
            if ui.button("Add").clicked()
            {
              for _ in 0..balls_amount
              { balls.push(Ball::new()); }
            }

            if ui.button("Remove").clicked()
            {
              // Removing «more» balls than there are present
              if balls_amount > balls.len() as u32 { balls.clear(); }
              else
              {
                (0..balls_amount).for_each(|_| { balls.swap_remove(0); });
              }
            }

            ui.separator();

            if ui.button("Clear all").clicked(){ balls.clear(); }
          });

          ui.horizontal(|ui|
          {
            ui.label("Amount:");
            ComboBox::from_label("")
              .selected_text(format!("{:?}", balls_amount))
              .width(45.)
              .show_ui(ui, |ui|
              {
                ui.selectable_value(&mut balls_amount, 1, "1");
                ui.selectable_value(&mut balls_amount, 2, "2");
                ui.selectable_value(&mut balls_amount, 5, "5");
                ui.selectable_value(&mut balls_amount, 10, "10");
                ui.selectable_value(&mut balls_amount, 20, "20");
                ui.selectable_value(&mut balls_amount, 50, "50");
              });
          });

          ui.separator();
          ui.horizontal(|ui|
          {
            ui.label(format!("# of balls: {}", balls.len()));
            ui.separator();
            ui.label(format!("FPS: {}", get_fps()));
          });

          ui.separator();

          // --- CREDITS (!important) ---
          ui.horizontal(|ui|
          {
            ui.label(format!("v{}", VERSION.unwrap_or("unknown")));
            ui.separator();
            ui.label("Made by");
            ui.hyperlink_to(AUTHORS.unwrap_or("Sandra").to_string(), "https://github.com/an-Iceberg");
          });
        });
    });

    draw();

    next_frame().await;
  }
}
