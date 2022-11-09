// use std::prelude;
use std::time;
use std::thread;

struct Createplayer{
    health: i32,
    damage: i32
}


fn main() {

    let _player = Createplayer { health: 10 , damage: 10 };
    println!("Setup running");
    setup();
    println!("Generating map but size check is needed.");
    generate_map(14, 14) ;
    thread::sleep(time::Duration::from_secs(2));
    println!("Creating player.");
    thread::sleep(time::Duration::from_secs(2));
    println!("Read player health check: {}. Read player damage check: {}.", _player.health, _player.damage);
    thread::sleep(time::Duration::from_secs(2));
    println!("Rendering map.");
    thread::sleep(time::Duration::from_secs(2));
    render_map();
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
    //let mut percent = 0;
  
    //while percent!=101 {
        //thread::sleep(time::Duration::from_millis(200));
        //println!("{}% complete rendering.", percent);
        //percent+=1;
    //}

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

fn setup() {
    
} 
