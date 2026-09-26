use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

const W: usize = 160;
const H: usize = 120;
const SAND: u32 = 0xE0C080;
const HZ: usize = 144;
const WINDOW_NAME: &str = "Sandbox";

use crate::grid_2d::Grid2D;

pub struct Sandbox {
    window: Window,
    grid: Grid2D<u32>,
}

impl Sandbox {
    pub fn new() -> Self {
        let opts = WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        };
        let mut window = Window::new(WINDOW_NAME, W, H, opts).expect("no window");
        window.set_target_fps(HZ);
        Self {
            window,
            grid: Grid2D::filled(W, H, 0),
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.window.get_mouse_down(MouseButton::Left) {
                if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Discard) {
                    let (x, y) = (x as usize, y as usize);

                    self.grid.update_at(x, y, SAND);
                }
            }
            self.sand_fall_step();

            let buffer = self.grid.get_grid_ref();
            self.window.update_with_buffer(&buffer, W, H).unwrap();
        }
    }

    fn sand_fall_step(&mut self) {
        for i in self.grid.coordinates.clone().iter().rev() {
            let i = *i;
            if self.grid.at(i) != &0 {
                let (x, y) = self.grid.to_xy(i);
                if y == H - 1 {
                    continue;
                }

                let down_y = y + 1;
                if self.grid.at_xy(x, down_y) == &0 {
                    self.grid.update_at(x, y, 0);
                    self.grid.update_at(x, down_y, SAND);
                } else {
                    if rand::random() {
                        self.go_left(x, y, down_y);
                    } else {
                        self.go_right(x, y, down_y);
                    }
                }
            }
        }
    }

    fn go_right(&mut self, x: usize, y: usize, down_y: usize) {
        if x != 0 {
            let right_x = x - 1;
            if self.grid.at_xy(right_x, down_y) == &0 {
                self.grid.update_at(x, y, 0);
                self.grid.update_at(right_x, down_y, SAND);
            }
        }
    }

    fn go_left(&mut self, x: usize, y: usize, down_y: usize) {
        if x != W - 1 {
            let left_x = x + 1;
            if self.grid.at_xy(left_x, down_y) == &0 {
                self.grid.update_at(x, y, 0);
                self.grid.update_at(left_x, down_y, SAND);
            }
        }
    }
}
