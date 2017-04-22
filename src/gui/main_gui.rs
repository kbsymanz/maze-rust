extern crate gtk;
extern crate tempdir;


use self::gtk::prelude::*;
use std::collections::HashSet;
use self::tempdir::TempDir;

pub fn main_gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Main window.
    let main = gtk::Window::new(gtk::WindowType::Toplevel);
    main.set_title("Maze Generator");
    main.set_default_size(800, 800);
    main.show_all();

    main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // 1st level Horizontal Box, left are controls, right is maze.
    let outer_hor_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    // 2nd level Frame holding all of the controls.
    let controls_frame = gtk::Frame::new("Maze Generator");

    // 2nd level Image of the maze.
    let maze_image = gtk::Image::new();

    // 3rd level Vertical Box for the controls.
    let controls_ver_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // 4th level Controls: radio buttons within a frame
    let maze_size_frame = gtk::Frame::new("Maze size");
    let radio_ver_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let radio_size_10 = gtk::RadioButton::new_with_label(&[], "10x10");
    let radio_size_20 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "20x20");
    let radio_size_30 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "30x30");
    let radio_size_40 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "40x40");
    let radio_size_60 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "60x60");
    let radio_size_80 = gtk::RadioButton::new_with_label_from_widget(Some(&radio_size_10), "80x80");

    // 4th level Controls: separator and generate button
    let radio_generate_sep = gtk::Separator::new(gtk::Orientation::Horizontal);
    let generate_btn = gtk::Button::new_with_label("Generate Maze");

    // Box everything up.
    radio_ver_box.pack_start(&radio_size_10, true, false, 0);
    radio_ver_box.pack_start(&radio_size_20, false, false, 0);
    radio_ver_box.pack_start(&radio_size_30, false, false, 0);
    radio_ver_box.pack_start(&radio_size_40, false, false, 0);
    radio_ver_box.pack_start(&radio_size_60, false, false, 0);
    radio_ver_box.pack_start(&radio_size_80, false, false, 0);
    maze_size_frame.add(&radio_ver_box);
    controls_ver_box.pack_start(&maze_size_frame, false, false, 10);
    controls_ver_box.pack_start(&radio_generate_sep, false, false, 0);
    controls_ver_box.pack_start(&generate_btn, false, false, 0);
    controls_frame.add(&controls_ver_box);
    outer_hor_box.pack_start(&controls_frame, false, true, 5);
    outer_hor_box.pack_start(&maze_image, true, true, 5);

    main.add(&outer_hor_box);

    // Generate the initial maze.
    run_generate_maze(10usize, &maze_image);

    // Generate the maze as selected in the maze size radio buttons.
    generate_btn.connect_clicked( move | _ | {
        // Find selected maze size and generate it.
        let maze_size: usize;
        if gtk::RadioButton::get_active(&radio_size_10) {
            maze_size = 10usize;
        } else if gtk::RadioButton::get_active(&radio_size_20) {
            maze_size = 20usize;
        } else if gtk::RadioButton::get_active(&radio_size_30) {
            maze_size = 30usize;
        } else if gtk::RadioButton::get_active(&radio_size_40) {
            maze_size = 40usize;
        } else if gtk::RadioButton::get_active(&radio_size_60) {
            maze_size = 60usize;
        } else if gtk::RadioButton::get_active(&radio_size_80) {
            maze_size = 80usize;
        } else {
            maze_size = 10usize;
        }

        run_generate_maze(maze_size, &maze_image);
    });

    main.show_all();

    gtk::main();
}

/// Sets up a temporary directory and file name for the generate_maze()
/// function to use. Cleans everything up afterward.
fn run_generate_maze(maze_size: usize, ref maze_image: &gtk::Image) {
    let tmp_dir = TempDir::new("maze-rust");
    match tmp_dir {
        Ok(tdir) => {
            match tdir.path().join("gui_maze.png").to_str() {
                Some(fp) => {
                    generate_maze(&fp.to_string(), maze_size);
                    gtk::Image::set_from_file(&maze_image, &fp);
                },
                None => println!("Error, unable to generate maze.")
            };
        },
        Err(msg) => println!("{}", msg)
    }
}

fn generate_maze(fullname: &String, size: usize) {
    let mut grid = ::grid::Grid {
        width: size,
        height: size,
        cells: Vec::with_capacity(size * size),
        back_track: Vec::with_capacity(size * size),
        visited: HashSet::with_capacity(size * size),
    };
    grid.init();
    grid.carve(0, 0);
    grid.save_png(&fullname);
}
