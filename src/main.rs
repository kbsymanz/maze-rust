/// Implements the Recursive Backtracker algorithm for
/// maze generation.

// Local
extern crate kbsymanz_maze;

// Local
use kbsymanz_maze::grid;

// Third-party
use std::collections::HashSet;

#[cfg(not(test))]
#[allow(unused_variables)]
fn main() {
    // TODO: 1. Accept command line parameters for batch, width, height, file format, etc.
    let num_mazes = 1;
    println!("Mazes generated:");
    for i in 0..num_mazes {
        let mut grid = grid::Grid {
            width: 40,
            height: 40,
            cells: vec![],
            back_track: vec![],
            visited: HashSet::new(),
        };
        grid.init();
        grid.carve(0, 0);
        grid.sanity();
        grid.save(true, true);
    }
    println!("");
    println!("Created {} mazes.", num_mazes);
}
