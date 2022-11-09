// use std::prelude;
use std::time;
use std::thread;

struct MainSetup {

}

struct GamePlay {

}


fn main() {
    let game_over = false;
    let _mainsetup = MainSetup {};
    let _gameplay = GamePlay {};
    println!("Setup running");
    MainSetup::setup();
    println!("Generating map but size check is needed.");
    MainSetup::generate_map(14, 14) ;
    thread::sleep(time::Duration::from_secs(2));
    println!("Creating player.");
    thread::sleep(time::Duration::from_secs(2));
    println!("Rendering map.");
    thread::sleep(time::Duration::from_secs(2));
    MainSetup::render_map();
    while game_over == false {
        GamePlay::do_game_tick();
        GamePlay::do_game_checks();
        GamePlay::create_enemies();
    } 
}

impl MainSetup {
    fn setup() {
         
    }

    fn generate_map(size_local_x: i128, size_local_y: i128) {
        println!("Generating size {} by {} map.", size_local_x, size_local_y);
        if size_local_x & size_local_y <= 10 {
            println!("Map is too small.")
        } else if size_local_x & size_local_y >= 100000 {
            println!("Map is too big.");
        } else {
            println!("Size is acceptable. Generating map.")
        }
    }

    fn render_map() {
        let pause = 500;
        println!("Beginning pre-render.");
        thread::sleep(time::Duration::from_millis(pause));
        println!("Building colour.");
        thread::sleep(time::Duration::from_millis(pause));
        println!("Building shaders.");
        thread::sleep(time::Duration::from_millis(pause));
        println!("Beginning true render.");
        thread::sleep(time::Duration::from_millis(pause));
        println!("Building basic shapes");
        thread::sleep(time::Duration::from_millis(pause));
        println!("Final clean up.");
        thread::sleep(time::Duration::from_millis(pause));
        println!("System check: complete. Rendering: complete.");
    }
}

impl GamePlay {
    fn create_enemies() {

    }

    fn do_game_tick() {

    }

    fn do_game_checks() {

    }
}
