extern crate rand;

// Local
use grid;

// Third-party
use std::collections::HashSet;
use self::rand::distributions::{IndependentSample, Range};


pub fn get_random_unvisited_neighbor(cx: usize,
                                     cy: usize,
                                     width: usize,
                                     height: usize,
                                     visited: &HashSet<(usize, usize)>)
                                     -> Option<(usize, usize)> {
    let neighbors = get_unvisited_neighbors(cx, cy, width, height, visited);

    if neighbors.len() == 0 {
        None
    } else {
        let idx = get_random_in_range(0, neighbors.len() as usize);
        Some(neighbors[idx])
    }
}

pub fn link(x1: usize,
            y1: usize,
            x2: usize,
            y2: usize,
            width: usize,
            cells: &mut Vec<grid::Cell>) {
    if x1 == x2 && ((y1 as isize - y2 as isize) as isize).abs() == 1 {
        // North or South
        if y1 < y2 {
            cells[(width * y1) + x1].s = true;
            cells[(width * y2) + x2].n = true;
        } else {
            cells[(width * y1) + x1].n = true;
            cells[(width * y2) + x2].s = true;
        }
    } else if y1 == y2 && ((x1 as isize - x2 as isize) as isize).abs() == 1 {
        // East or West
        if x1 < x2 {
            cells[(width * y1) + x1].e = true;
            cells[(width * y2) + x2].w = true;
        } else {
            cells[(width * y1) + x1].w = true;
            cells[(width * y2) + x2].e = true;
        }
    } else {
        panic!("Trying to link inappropriate cells: ({}, {}) and ({}, {}).",
               x1,
               y1,
               x2,
               y2);
    }
}


/// Return a Vector of tuples representing the x and y coordinates of the
/// immediate neighbors of the specified x and y cell within the context
/// of the grid size per the width and height passed.
///
/// Returns an empty vector if there are no neighbors, i.e. the x and/or
/// y passed is not within the grid.
pub fn get_neighbors(cx: usize, cy: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    if cx > width - 1 || cy > height - 1 {
        let n = neighbors;
        n
    } else {
        if cx > 0 {
            neighbors.push((cx - 1, cy));
        }
        if cx < width - 1 {
            neighbors.push((cx + 1, cy));
        }
        if cy > 0 {
            neighbors.push((cx, cy - 1));
        }
        if cy < height - 1 {
            neighbors.push((cx, cy + 1));
        }

        let n = neighbors;
        n
    }
}

fn get_unvisited_neighbors(cx: usize,
                           cy: usize,
                           width: usize,
                           height: usize,
                           visited: &HashSet<(usize, usize)>)
                           -> Vec<(usize, usize)> {
    let mut unvisited: Vec<(usize, usize)> = vec![];
    let neighbors = get_neighbors(cx, cy, width, height);
    for n in neighbors {
        if !visited.contains(&n) {
            unvisited.push(n);
        }
    }

    unvisited
}

/// Return a random usize between min and max, not including max.
fn get_random_in_range(min: usize, max: usize) -> usize {
    let between = Range::new(min, max);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng) as usize
}
