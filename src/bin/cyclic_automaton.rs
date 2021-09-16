use macroquad::{color::hsl_to_rgb, prelude::*};

const CELL_SIZE: f32 = 8.;
const STATES: usize = 8;

const NB_SIZE: isize = 4;

const THRESHOLD: i32 = 30;

#[macroquad::main("Cyclic Automaton")]
async fn main() {
    let height = (screen_height() / CELL_SIZE) as usize - 2;
    let width = height;

    let mut lattice = vec![0; width * height];

    let mut change_lattice = vec![false; width * height];

    rand::srand(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64() as _,
    );

    let mut reset = true;

    let mut change_view = false;

    let mut time = get_time();

    loop {
        let x_off = screen_width() / 2. - (width) as f32 * CELL_SIZE / 2.;
        let y_off = screen_height() / 2. - (height) as f32 * CELL_SIZE / 2.;

        if reset {
            for i in 0..width {
                for j in 0..height {
                    let state = rand::gen_range(0, STATES);

                    lattice[width * i + j] = state;
                }
            }

            reset = false;
        }

        for i in 0..width {
            for j in 0..height {
                let state = lattice[width * i + j];

                let a = state as f32 / STATES as f32;

                let b = if change_view {
                    if change_lattice[width * i + j] {
                        0.25
                    } else {
                        0.75
                    }
                } else {
                    0.75
                };

                draw_rectangle(
                    x_off + i as f32 * CELL_SIZE,
                    y_off + j as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    hsl_to_rgb(a, 0.8, b),
                );
            }
        }

        if get_time() - time > 1. / 72. {
            time = get_time();

            let mut lattice_clone = lattice.clone();

            for i in 0..width {
                for j in 0..height {
                    let mut count = 0;
                    let state = lattice[width * i + j];

                    for k in -NB_SIZE..=NB_SIZE {
                        for l in -NB_SIZE..=NB_SIZE {
                            let kx = i as isize + k;
                            let ky = j as isize + l;

                            if lattice[width * ((kx + width as isize) % width as isize) as usize
                                + ((ky + height as isize) % height as isize) as usize]
                                == (state + 1) % STATES
                            {
                                count += 1;
                            }
                        }
                    }

                    if count > THRESHOLD {
                        lattice_clone[width * i + j] = (lattice[width * i + j] + 1) % STATES;

                        change_lattice[width * i + j] = true;
                    } else {
                        change_lattice[width * i + j] = false;
                    }
                }
            }

            lattice = lattice_clone;
        }

        if is_key_pressed(KeyCode::R) {
            reset = true;
        }

        if is_key_pressed(KeyCode::C) {
            change_view = !change_view;
        }

        next_frame().await
    }
}
