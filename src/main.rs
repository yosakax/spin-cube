use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 250;
const HEIGHT: usize = 50;
const K1: f64 = 40_f64;
const DISTANCE_FROM_CAM: f64 = 100.0;

fn calculate_x(i: f64, j: f64, k: f64, A: f64, B: f64, C: f64) -> f64 {
    j * A.sin() * B.sin() * C.cos() - k * A.cos() * B.sin() * C.cos()
        + j * A.cos() * C.sin()
        + k * A.sin() * C.sin()
        + i * B.cos() * C.cos()
}
fn calculate_y(i: f64, j: f64, k: f64, A: f64, B: f64, C: f64) -> f64 {
    j * A.cos() * C.cos() + k * A.sin() * C.cos() - j * A.sin() * B.sin() * C.sin()
        + k * A.cos() * B.sin() * C.sin()
        - i * B.cos() * C.sin()
}
fn calculate_z(i: f64, j: f64, k: f64, A: f64, B: f64, C: f64) -> f64 {
    k * A.cos() * B.cos() - j * A.sin() * B.cos() + i * B.sin()
}

fn calculate_for_surface(
    cube_x: f64,
    cube_y: f64,
    cube_z: f64,
    A: f64,
    B: f64,
    C: f64,
    ch: char,
    buffer: &mut Vec<char>,
    z_buffer: &mut Vec<f64>,
    horizontal_offset: f64,
) {
    let x = calculate_x(cube_x, cube_y, cube_z, A, B, C);
    let y = calculate_y(cube_x, cube_y, cube_z, A, B, C);
    let z = calculate_z(cube_x, cube_y, cube_z, A, B, C) + DISTANCE_FROM_CAM;
    let ooz = 1f64 / z;
    let xp = (WIDTH as f64 / 2f64 + horizontal_offset + K1 * ooz * x * 2f64) as isize;
    let yp = (HEIGHT as f64 / 2f64 + K1 * ooz * y) as isize;
    let idx = xp + yp * WIDTH as isize;
    if idx >= 0 && (idx as usize) < WIDTH * HEIGHT && ooz > z_buffer[idx as usize] {
        z_buffer[idx as usize] = ooz;
        buffer[idx as usize] = ch;
    }
}

fn main() {
    print!("\x1b[2J");
    // clear().unwrap();
    let mut buffer = vec![' '; WIDTH * HEIGHT];
    let mut z_buffer = vec![0.0; WIDTH * HEIGHT + 4];
    let background: char = ' ';
    let mut horizontal_offset;
    let increment_speed = 0.2;
    let mut A = 0.0;
    let mut B = 0.0;
    let mut C = 0.0;
    let duration = Duration::from_micros(8000 * 2);
    loop {
        buffer.fill(background);
        z_buffer.fill(0.0);
        let cube_width = 20f64;
        horizontal_offset = -2.0 * cube_width;
        let mut cube_x = -cube_width;
        while cube_x < cube_width {
            let mut cube_y = -cube_width;
            while cube_y < cube_width {
                calculate_for_surface(
                    cube_x,
                    cube_y,
                    -cube_width,
                    A,
                    B,
                    C,
                    '@',
                    &mut buffer,
                    &mut z_buffer,
                    horizontal_offset,
                );
                calculate_for_surface(
                    cube_width,
                    cube_y,
                    cube_x,
                    A,
                    B,
                    C,
                    '$',
                    &mut buffer,
                    &mut z_buffer,
                    horizontal_offset,
                );
                calculate_for_surface(
                    -cube_width,
                    cube_y,
                    cube_x,
                    A,
                    B,
                    C,
                    '~',
                    &mut buffer,
                    &mut z_buffer,
                    horizontal_offset,
                );
                calculate_for_surface(
                    -cube_x,
                    cube_y,
                    cube_width,
                    A,
                    B,
                    C,
                    '#',
                    &mut buffer,
                    &mut z_buffer,
                    horizontal_offset,
                );
                calculate_for_surface(
                    cube_x,
                    -cube_width,
                    -cube_y,
                    A,
                    B,
                    C,
                    ';',
                    &mut buffer,
                    &mut z_buffer,
                    horizontal_offset,
                );
                calculate_for_surface(
                    cube_x,
                    cube_width,
                    cube_y,
                    A,
                    B,
                    C,
                    '+',
                    &mut buffer,
                    &mut z_buffer,
                    horizontal_offset,
                );
                cube_y += increment_speed;
            }
            cube_x += increment_speed;
        }
        print!("\x1b[H");
        for k in (0..WIDTH * HEIGHT).step_by(WIDTH) {
            let mut line = String::new();
            for l in k..k + WIDTH {
                line.push(buffer[l]);
            }
            println!("{}", line);
        }
        A += 0.05;
        B += 0.05;
        C += 0.01;
        sleep(duration);
    }
}
