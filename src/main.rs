mod display;
mod display_list;
mod formatted_print;
mod hello_world;

fn main() {
    hello_world::run();
    println!();
    formatted_print::run();
    println!();
    display::run();
    println!();
    display_list::run();
}
