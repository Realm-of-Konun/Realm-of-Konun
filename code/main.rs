use std::prelude;

fn main() {
    println!("Generating map");
    generate_map(1000);
    println!("Creating player");
    create_player();
    println!("Rendering map");
    render_map();
}

fn generate_map(size: i128) {
    println!("Generating size {} map.", size);
    if size <= 10 {
        println!("Map is too small.")
    } else if size >= 1000000 {
        println!("Map is too big.");
 }
}

fn create_player() {
    
}

fn render_map() {

}
