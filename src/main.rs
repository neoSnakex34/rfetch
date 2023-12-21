mod functions;
use viuer::{print_from_file, Config};
use termion::clear;
use termion::cursor::Goto;
use std::io::{stdout, Write};
use std::thread;
use std::sync::{Arc, Mutex};



fn main() {
    
    let conf = Config{
//        x: 4, 
//        y: 4, 
        //le y si raddoppiano in terminal cells
        //quindi conta raddoppiando la distanza
        width: Some(20),
        height: Some(10),
        //restore_cursor: true,
        ..Default::default()
    };
    
    let stdout = stdout();
    let mutex_stdout = Arc::new(Mutex::new(stdout));
    
    let clone_arc_for_thread = Arc::clone(&mutex_stdout);

    //move vedi docs 
    //decommenta da qua
    let image_printer = thread::spawn(move || {
   
        let mut stdout = clone_arc_for_thread.lock().unwrap();
        write!(stdout, "{}", clear::All).unwrap();    
        print_from_file("ferris.png", &conf).expect("zamn");
        stdout.flush().unwrap();
    });


    image_printer.join().unwrap();
    
    let mut stdout = mutex_stdout.lock().unwrap();
    
    write!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();

    //===================== printing===================
    
    //29 e' width 20 + 8(unsure why) + 1 
    let distro_name = functions::get_linux_distro_name().unwrap(); 
    println!("{:>29}{} ","distro: ", distro_name);
    

    
    write!(stdout, "{}", termion::cursor::Goto(10,10)).unwrap();
    stdout.flush().unwrap(); 
    
}

