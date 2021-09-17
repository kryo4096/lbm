use macroquad::prelude::*;


type Node = [bool; 4];

const CELL_SIZE: f32 = 8.;
const PERIODIC: bool = false;



fn draw_node(node: &Node, x: f32, y: f32) {
    if node[0] {
        draw_circle(x + CELL_SIZE / 4., y, CELL_SIZE / 8., GREEN);
    }
    if node[1] {
        draw_circle(x, y- CELL_SIZE / 4., CELL_SIZE / 8. , GREEN);
    }
    if node[2] {
        draw_circle(x - CELL_SIZE / 4., y, CELL_SIZE / 8., GREEN);
    }
    if node[3] {
        draw_circle(x, y + CELL_SIZE / 4., CELL_SIZE / 8., GREEN);
    }
}

fn collide(node: &Node) -> Node {
    match node {
        [true, false, true, false] => [false, true, false, true],
        [false, true, false, true] => [true, false, true, false],
        _ => *node,
    }
}

fn update_lattice(lattice: &mut Vec<Vec<Node>>, height: usize, width: usize) {
    let mut new_lattice : Vec<Vec<Node>> = vec![vec![[true;4];height];width];

    for i in 0..width {
        for j in 0 .. height {
            new_lattice[i][j] = collide(&lattice[i][j]);
        }
    }

    *lattice = new_lattice.clone();

    for i in 0..width {
        for j in 0 .. height {
            if i == 0 {
                new_lattice[i][j][0] = lattice[i][j][2];
            } else {
                new_lattice[i][j][0] = lattice[i-1][j][0];
            }

            if j == 0 {
                new_lattice[i][j][3] = lattice[i][j][1];

            } else {
                new_lattice[i][j][3] = lattice[i][j-1][3];
            }

            if i == width - 1 {
                new_lattice[i][j][2] = lattice[i][j][0];

            } else {
                new_lattice[i][j][2] = lattice[i+1][j][2];
            }

            if j == height - 1 {
                new_lattice[i][j][1] = lattice[i][j][3];
            } else {
                new_lattice[i][j][1] = lattice[i][j+1][1];
            }
        }
    }

    *lattice = new_lattice;
} 

#[macroquad::main("2D HPP Lattice-Gas Automaton")]
async fn main() {
    let height = (screen_height() / CELL_SIZE) as usize - 2;
    let width = height;

    let mut lattice : Vec<Vec<Node>> = vec![vec![[false; 4];height];width];

    for i in width/2 - height / 3 .. width / 2 + width / 3 {
        for j in height/2 - height / 3 .. height / 2 + height / 3 {
            for d in 0..4 {
                if rand::gen_range(0.0, 1.0) < 0.5 {
                    lattice[i][j][d] = true;
                }
            }
        }
    }

    let mut time = get_time();

    loop {
        let x_off = screen_width() / 2. - (width - 1) as f32 * CELL_SIZE / 2.;
        let y_off = screen_height() / 2. - (height - 1) as f32 * CELL_SIZE / 2.;

        for i in 0..width {
            for j in 0 .. height {
                let node = lattice[i][j];

               draw_node(&node, x_off + i as f32 * CELL_SIZE , y_off + j as f32 * CELL_SIZE);
            }
        }

        if get_time() - time > 0.05 {
            time = get_time();

            update_lattice(&mut lattice, height, width);
        }

        next_frame().await
    }
    

}