/// Implements the Recursive Backtracker algorithm for
/// maze generation.

extern crate clap;

// Local
extern crate kbsymanz_maze;

// Local
use kbsymanz_maze::grid;

// Third-party
use clap::{App, Arg};
use std::collections::HashSet;

#[cfg(not(test))]
#[allow(unused_variables)]
fn main() {
    let matches = App::new("Maze Generator")
                    .version("0.2.0")
                    .author("Kurt Symanzik <kurt@kbsymanzik.org>")
                    .about("Generates mazes")
                    .arg(Arg::with_name("batch")
                         .short("b")
                         .long("batch")
                         .help("Batch mode generates many mazes at once.")
                         .takes_value(true))
                    .arg(Arg::with_name("json")
                         .short("j")
                         .long("json")
                         .help("Output the maze(s) in JSON format.")
                         .takes_value(false))
                    .arg(Arg::with_name("png")
                         .short("p")
                         .long("png")
                         .help("Output the maze(s) in PNG format.")
                         .takes_value(false))
                    .arg(Arg::with_name("size")
                         .short("s")
                         .long("size")
                         .help("Size of the maze, i.e. number of cells wide and high.")
                         .takes_value(true))
                    .get_matches();

    // Number of mazes to generate, defaults to one.
    let nbr_mazes = matches.value_of("batch").unwrap_or("1");
    let num_mazes = nbr_mazes.parse::<i32>().unwrap_or(1);

    // Output PNG and/or JSON, defaults to PNG.
    let output_json: bool = matches.is_present("json");
    let mut output_png: bool = matches.is_present("png");
    if ! output_json && ! output_png {
        output_png = true;
    }

    // Size of the maze, defaults to 40 x 40.
    let size = matches.value_of("size").unwrap_or("40").parse::<usize>().unwrap_or(40);

    print!("Generating {} maze", num_mazes);
    if num_mazes > 1 {
        print!("s");
    }
    println!(":");

    for i in 0..num_mazes {
        let mut grid = grid::Grid {
            width: size,
            height: size,
            cells: vec![],
            back_track: vec![],
            visited: HashSet::new(),
        };
        grid.init();
        grid.carve(0, 0);
        grid.sanity();
        grid.save(output_png, output_json);
    }
    println!("");
    println!("Created {} mazes.", num_mazes);
}
