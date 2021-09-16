use macroquad::{prelude::*, rand::gen_range};

const CELL_SIZE: f32 = 64.;
const CELL_SIZE_Y: f32 = CELL_SIZE * 1.7321 * 0.5;

const OFFSETS: [[[isize; 2]; 6]; 2] = [
    [[1, 0], [0, -1], [-1, -1], [-1, 0], [-1, 1], [0, 1]],
    [[0, 0], [1, -1], [0, -1], [-1, 0], [0, 1], [1, 1]],
];

type CollisionRule = [f32; 64];

#[inline]
fn get_cell(i: usize, node: u8) -> bool {
    ((node >> i) & 0b1) != 0b0
}

#[inline]
fn set_cell(i: usize, node: &mut u8, val: bool) {
    *node = (!(0b1 << i) & *node) | (val as u8) << i;
}

fn generate_vectors<const N: usize>() -> [[f32; 2]; N] {
    let mut a = [[0.; 2]; N];

    a.iter_mut().enumerate().for_each(|(i, v)| {
        let alpha = i as f32 / N as f32 * 2. * std::f32::consts::PI;
        *v = [alpha.cos(), alpha.sin()];
    });

    a
}

fn draw_node(node: &u8, x: f32, y: f32) {
    let vecs = generate_vectors::<6>();

    for i in 0..6 {
        draw_circle(
            0.2 * vecs[i][0] * CELL_SIZE + x,
            0.2 * vecs[i][1] * CELL_SIZE + y,
            0.1 * CELL_SIZE,
            if get_cell(i, *node) { WHITE } else { DARKGRAY },
        );
    }
}

fn init_grid(w: usize, h: usize) -> Vec<Vec<u8>> {
    let mut v = vec![];

    for i in 0..h {
        v.push(vec![0; w])
    }

    v
}

#[macroquad::main("2D Lattice-Gas Automaton")]
async fn main() {
    let height = (screen_height() / CELL_SIZE_Y) as usize - 2;
    let width = height;

    let mut lattice: Vec<Vec<u8>> = init_grid(width, height);

    for j in height / 4..height * 3 / 4 {
        for i in width / 4..width * 3 / 4 {
            lattice[j][i] = rand::gen_range(0, 63);
        }
    }

    let mut time = get_time();

    loop {
        let x_off = screen_width() / 2. - (width - 1) as f32 * CELL_SIZE / 2.;
        let y_off = screen_height() / 2. - (height - 1) as f32 * CELL_SIZE_Y / 2.;

        for j in 0..height {
            for i in 0..lattice[j].len() {
                draw_node(
                    &lattice[j][i],
                    x_off + i as f32 * CELL_SIZE + if j % 2 == 0 { 0. } else { 0.5 * CELL_SIZE },
                    y_off + j as f32 * CELL_SIZE_Y,
                );
            }
        }

        if get_time() - time >= 1. {
            time = get_time();

            let mut new_lattice = lattice.clone();

            for j in 0..height {
                for i in 0..lattice[j].len() {
                    for d in 0..6 {
                        let [i_off, j_off] = OFFSETS[j % 2][d];

                        let nj =
                            ((j as isize - j_off + height as isize) % height as isize) as usize;

                        let w = lattice[nj].len();

                        let ni = ((i as isize + i_off + w as isize) % w as isize) as usize;

                        set_cell(d, &mut new_lattice[nj][ni], get_cell(d, lattice[j][i]));
                    }
                }
            }

            lattice = new_lattice;
        }

        next_frame().await
    }
}
