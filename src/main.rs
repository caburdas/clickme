use macroquad::audio::{load_sound, play_sound_once};
use macroquad::{prelude::*, time};

/*
instead ofdoing #[macroquad::main("Clickme game")] //window tittle
you can do:

fn conf() -> Conf {
 Conf {
   window_title: "Clicker Game", //this field is not optional!
   fullscreen:false,
   //you can add other options too, or just use the default ones:
   ..Default::default()
 }
}
//then pass the function to the attribute
#[macroquad::main(conf)]
async fn main() {
...
...
}
*/
const COLORS: [Color; 9] = [
    RED, GREEN, BLUE, PURPLE, SKYBLUE, BLACK, WHITE, YELLOW, PINK,
];

#[macroquad::main("Click me! game")] //window tittle
async fn main() {
    let mut x = screen_width() / 2.;
    let mut y = screen_height() / 2.;
    let mut r = 70.;
    let mut circle = Circle::new(x, y, r);
    let mut score = 0;
    let beep_sound = load_sound("res/beep.wav").await.unwrap();
    let mut color = COLORS[0]; //RED
    let mut total_circles = 1;
    let mut circle_last_time = time::get_time();

    loop {
        clear_background(GRAY);

        let time_now = time::get_time();
        if time_now - circle_last_time > 1. {
            // after 1 sec draw another circle
            x = rand::gen_range(10., screen_width() - 10.);
            y = rand::gen_range(10., screen_height() - 10.);
            r = rand::gen_range(10., screen_width() / 10.);
            circle.x = x;
            circle.y = y;
            circle.r = r;
            total_circles += 1;
            circle_last_time = time_now;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_circ = Circle::new(mouse_x, mouse_y, 1.0);

            if mouse_circ.overlaps(&circle) {
                score += 1;
                play_sound_once(beep_sound);
                color = COLORS[rand::gen_range(0, 9)];
            }
        }

        draw_text(
            "Click me! game",
            (screen_width() / 2.) - 100.,
            100.,
            50.,
            WHITE,
        );
        draw_text(
            format!("Clicks: {} / {}", score, total_circles).as_str(),
            (screen_height() / 2.) - 100.,
            500.,
            50.,
            WHITE,
        );
        draw_circle(x, y, r, color);
        next_frame().await;
    }
}
