mod display;
mod display_list;
mod formatted_print;
mod formatting;
mod hello_world;
mod primitives;

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
}
