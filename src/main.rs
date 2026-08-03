mod arrays_and_slices;
mod display;
mod display_list;
mod formatted_print;
mod formatting;
mod hello_world;
mod if_let;
mod primitives;
mod structs;
mod tuples;

fn main() {
    hello_world::run();
    println!();
    formatted_print::run();
    println!();
    display::run();
    println!();
    display_list::run();
    println!();
    formatting::run();
    println!();
    primitives::run();
    println!();
    tuples::run();
    println!();
    arrays_and_slices::run();
    println!();
    structs::run();
    println!();
    if_let::run();
}
