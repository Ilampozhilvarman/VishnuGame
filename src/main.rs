use macroquad::prelude::*;

// Optional: Configure your window settings
fn window_conf() -> Conf {
    Conf {
        window_title: "Simple Arcade Game".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    const RADIUS: f32 = 25.0;
    const SPEED: f32 = 350.0;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x += SPEED * dt;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x -= SPEED * dt;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y += SPEED * dt;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y -= SPEED * dt;
        }

        x = x.clamp(RADIUS, screen_width() - RADIUS);
        y = y.clamp(RADIUS, screen_height() - RADIUS);

        draw_circle(x, y, RADIUS, GREEN);

        draw_text("Use WASD or Arrows to Move", 20.0, 30.0, 20.0, WHITE);

        next_frame().await;
    }
}
