extern crate ruscii;

use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

#[derive(Copy, Clone)]
struct LogoBox {
    pos: Vec2,
    color: Color,
}

impl LogoBox {
    pub fn new(pos: Vec2, color: Color) -> Self {
        return LogoBox {
            pos: pos,
            color: color,
        };
    }
}

enum ScreenState {
    LOADING,
    ERROR,
    RESTART,
    START,
}

fn main() {
    let mut app = App::new();
    let mut key_events = Vec::new();
    let size = app.window().size();
    let box_size = Vec2::xy(4, 2);
    let mut frames_passed = 0;
    let mut seconds_passed = 0;
    let mut text_index = 0;

    let mut text_prev_frame = 0;
    let mut loading_prev_frame = 0;
    let mut times_enter_pressed = 0;

    let loading_line_width = 5;
    let mut x_pos = loading_line_width * 2 + 2;
    let mut state = ScreenState::START;

    let logo_boxes = [
        LogoBox::new(Vec2::xy(-2, 0), Color::Red),
        LogoBox::new(Vec2::xy(2, 0), Color::Green),
        LogoBox::new(Vec2::xy(-2, 2), Color::Blue),
        LogoBox::new(Vec2::xy(2, 2), Color::Yellow),
    ];

    println!("{:?}", size);

    app.run(|app_state: &mut State, window: &mut Window| {
        // Handle keyboard keys
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(key) => match key {
                    Key::Esc => app_state.stop(),
                    Key::Q => app_state.stop(),
                    Key::Enter => times_enter_pressed += 1,
                    Key::R => match state {
                        ScreenState::ERROR => {
                            seconds_passed = 0;
                            times_enter_pressed = 0;
                            state = ScreenState::RESTART;
                        }
                        _ => (),
                    },
                    _ => {
                        key_events.push(*key);
                    }
                },
                _ => (),
            }
        }

        match state {
            ScreenState::LOADING => {
                // Create windows logo
                let logo_offset = Vec2::xy(2, 4);
                let origin = size / 2;
                for logo_box in logo_boxes {
                    Pencil::new(window.canvas_mut())
                        .set_origin(origin - logo_offset)
                        .set_foreground(logo_box.color)
                        .set_background(logo_box.color)
                        .draw_rect(&RectCharset::simple_lines(), logo_box.pos, box_size);
                }

                // Loading bar
                let bar_width = 30;
                Pencil::new(window.canvas_mut()).draw_rect(
                    &RectCharset::double_lines(),
                    origin + Vec2::xy(-bar_width / 2, 5),
                    Vec2::xy(bar_width, 3),
                );

                // Loading entities
                if frames_passed - loading_prev_frame > 1 {
                    x_pos -= 1;
                    loading_prev_frame = frames_passed;
                    if x_pos < -loading_line_width * 2 {
                        x_pos = loading_line_width * 2 + 3;
                    }
                }

                Pencil::new(window.canvas_mut())
                    .set_foreground(Color::Cyan)
                    .draw_hline('-', origin + Vec2::xy(-x_pos, 6), 3);

                // Loading text
                let texts = [
                    "Starting Windows 1.43 ...",
                    "Please wait till we load your desktop",
                    "In the meantime, you can tell how painful your day so far",
                    "Oh! Oh! Oh! I got it, try smashing Enter 50 times.",
                ];

                if frames_passed - text_prev_frame > 120 {
                    text_prev_frame = frames_passed;
                    text_index += 1;
                    text_index = text_index % texts.len();
                }

                let text = texts[text_index];
                Pencil::new(window.canvas_mut())
                    .set_origin(origin - Vec2::xy(text.len() / 2, 9))
                    .draw_text(text, Vec2::y(12));

                if times_enter_pressed > 50 {
                    state = ScreenState::ERROR;
                }
            }
            ScreenState::ERROR => {
                let mut pencil = Pencil::new(window.canvas_mut());

                pencil
                    .set_background(Color::Blue)
                    .set_foreground(Color::Blue)
                    .draw_rect(&RectCharset::double_lines(), Vec2::xy(0, 0), size);

                show_error_messages(pencil.new_one());
            }
            ScreenState::RESTART => {
                let restarting_text = "Restarting...";
                Pencil::new(window.canvas_mut())
                    .set_origin(size / 2)
                    .draw_text(
                        restarting_text,
                        Vec2::zero() - Vec2::x(restarting_text.len() / 2),
                    );

                if seconds_passed > 200 {
                    state = ScreenState::LOADING;
                }
            }
            ScreenState::START => {
                let restarting_text = "Starting...";
                Pencil::new(window.canvas_mut())
                    .set_origin(size / 2)
                    .draw_text(
                        restarting_text,
                        Vec2::zero() - Vec2::x(restarting_text.len() / 2),
                    );

                if seconds_passed > 200 {
                    state = ScreenState::LOADING;
                }
            }
        }
        if frames_passed > 30 {
            seconds_passed += 1;
        }
        frames_passed += 1;
    });
}

fn show_error_messages(mut pencil: Pencil) {

    pencil.set_foreground(Color::White);
    pencil.set_background(Color::Black);
    pencil.draw_text(
        "This whole was supposed to be blue but couldn't do it",
        Vec2::xy(2, 1),
    );
    pencil.draw_text(":(", Vec2::xy(2, 3));
    pencil.draw_text(
        "Your PC ran into a problem and needs to restart.",
        Vec2::xy(2, 6),
    );
    pencil.draw_text(
        "We're just collecting some error info, and then we'll restart for you.",
        Vec2::xy(2, 7),
    );
    pencil.draw_text(
        "I guess, smashing Enter 50 times wasn't a good idea :p",
        Vec2::xy(2, 10),
    );
    pencil.draw_text("You can press 'R' to restart.", Vec2::xy(2, 12));
}

