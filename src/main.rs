use macroquad::prelude::*;

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

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        next_frame().await;
    }
}
