extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate serde;
extern crate serde_json;
extern crate uuid;

// Local
use super::utils;

// Third-party
//use self::rusttype::Scale;
use self::image::{Rgb, RgbImage};
use self::imageproc::rect::Rect;
use self::imageproc::drawing;
use self::uuid::Uuid;
use std;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub n: bool,
    pub e: bool,
    pub s: bool,
    pub w: bool,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub back_track: Vec<(usize, usize)>,
    pub visited: HashSet<(usize, usize)>,
}

impl Grid {
    /// Initializes the `grid.cells` with Cells according to width and height.
    /// with the first and the last cells having exits to the outside at the
    /// top and the bottom respectively and all of the sides having borders.
    pub fn init(&mut self) {
        for y in 0..self.width {
            for x in 0..self.height {
                self.cells
                    .push(Cell {
                              x: x,
                              y: y,
                              n: (if x == 0 && y == 0 { true } else { false }),
                              e: false,
                              s: (if x == self.width - 1 && y == self.height - 1 {
                                      true
                                  } else {
                                      false
                                  }),
                              w: false,
                          });
            }
        }
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        println!("Maze has {} cells.", self.cells.len());
        for c in &self.cells {
            let is_dead: bool = !c.n && !c.e && !c.s && !c.w;
            if is_dead {
                print!("DEAD: ({},{}) {}{}{}{} ", c.x, c.y, c.n, c.e, c.s, c.w);
            } else {
                print!("({},{}) {}{}{}{} ", c.x, c.y, c.n, c.e, c.s, c.w);
            }
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn sanity(&self) {
        for c in &self.cells {
            if !c.n && !c.e && !c.s && !c.w {
                println!("Found dead cell at: ({}, {}).", c.x, c.y);
            }
            if c.x == 0 && c.y == 0 && c.n != true {
                println!("ERROR, improper entry/exits at ({}, {}).", c.x, c.y);
            }
            if c.x == self.width - 1 && c.y == self.height - 1 && c.s != true {
                println!("ERROR, improper entry/exits at ({}, {}).", c.x, c.y);
            }
        }
    }

    pub fn save(&self, png_output: bool, json_output: bool) {
        let uuid = Uuid::new_v4().hyphenated().to_string();
        if png_output {
            self.save_png(&uuid);
            print!("{}.png ", uuid);
        }
        if json_output {
            self.save_json(&uuid);
            print!("{}.json ", uuid);
        }
        println!("");
    }

    pub fn save_png(&self, uuid: &String) {
        let fname = format!("{}.png", uuid);
        let filename = Path::new(&fname);

        let img_width = 800u32;
        let img_height = 800u32;

        let grid_size = if self.width > std::i32::MAX as usize {
            None
        } else {
            Some(self.width as i32)
        };
        let cell_size = 700i32 / grid_size.unwrap();
        let left_margin = 50i32;
        let top_margin = 50i32;

        let black = Rgb([0u8, 0u8, 0u8]);
        let white = Rgb([255u8, 255u8, 255u8]);

        let mut image = RgbImage::from_pixel(img_width, img_height, white);

        for cell in &self.cells {
            if cell.n == false {
                let x = left_margin + (cell.x as i32 * cell_size);
                let y = top_margin + (cell.y as i32 * cell_size);
                let width = cell_size as u32;
                let height = 2u32;
                drawing::draw_filled_rect_mut(&mut image,
                                              Rect::at(x, y).of_size(width, height),
                                              black);
            }
            if cell.s == false {
                let x = left_margin + (cell.x as i32 * cell_size);
                let y = top_margin + (cell.y as i32 * cell_size) + cell_size;
                let width = cell_size as u32;
                let height = 2u32;
                drawing::draw_filled_rect_mut(&mut image,
                                              Rect::at(x, y).of_size(width, height),
                                              black);
            }
            if cell.e == false {
                let x = left_margin + (cell.x as i32 * cell_size) + cell_size;
                let y = top_margin + (cell.y as i32 * cell_size);
                let width = 2u32;
                let height = cell_size as u32;
                drawing::draw_filled_rect_mut(&mut image,
                                              Rect::at(x, y).of_size(width, height),
                                              black);
            }
            if cell.w == false {
                let x = left_margin + (cell.x as i32 * cell_size);
                let y = top_margin + (cell.y as i32 * cell_size);
                let width = 2u32;
                let height = cell_size as u32;
                drawing::draw_filled_rect_mut(&mut image,
                                              Rect::at(x, y).of_size(width, height),
                                              black);
            }
        }

        /*
        // Write out the filename to the image.
        // NOTE: imageproc does not seem to support this function yet.
        let scale = Scale { x: 10.0, y: 20.0 };
        let font = include_bytes!("Arial Unicode.ttf") as &[u8];
        drawing::draw_text_mut(&mut image, black, left_margin, 705, scale, font, fname);
        */

        image.save(filename).unwrap();
    }


    pub fn save_json(&self, uuid: &String) {
        let filename = format!("{}.json", uuid);
        let file = match File::create(&filename) {
            Err(msg) => panic!("Error creating file {}: {}", filename, msg),
            Ok(file) => file,
        };
        let mut writer = BufWriter::new(file);
        writer
            .write_all(self.to_json_string().as_bytes())
            .unwrap();
    }

    fn to_json_string(&self) -> String {
        let serialized = serde_json::to_string(&self.cells).unwrap();
        serialized
    }


    /// Implements the recursive backtracker algorithm to generate a maze.
    pub fn carve(&mut self, x: usize, y: usize) {
        // Mark this cell as visited. Record this cell in the
        // back track if it is not already there.
        if !self.visited.contains(&(x, y)) {
            self.visited.insert((x, y));
            self.back_track.push((x, y));
        }

        // Get an unvisited neighboring cell, if possible.
        let next_xy =
            utils::get_random_unvisited_neighbor(x, y, self.width, self.height, &self.visited);

        match next_xy {
            Some(next) => {
                // We found an unvisited neighbor so link to it
                // and recursively carve to it.
                utils::link(x, y, next.0, next.1, self.width, &mut self.cells);
                self.carve(next.0, next.1);
            }
            None => {
                // Did not find any more unvisited neighbors, so backtrack
                // one cell and try again.
                let last = self.back_track.pop();
                match last {
                    Some(lst) => {
                        // Check if there are any more unvisited cells here.
                        self.carve(lst.0, lst.1);
                        ()
                    }
                    None => {
                        // back_track vector is empty so we have visited and
                        // linked to all cells now - we are done.
                        ()
                    }
                }
            }
        }
    }
}
