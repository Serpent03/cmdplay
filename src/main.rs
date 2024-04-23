// TODO web query for the youtube API
// TODO queue based playlist(?)
// TODO enable multi-threading(low priority)

use std::io::{ self, BufRead, BufWriter };

fn main() {
    // get the MVP up and running. should be able to deal with I/O and make web requests.
    
    println!("Reading input five times.");
    let stdin = io::stdin();
    let mut line:String = String::new();

    for _i in 0..5 {
        /* _i is named as such because even though we need it, we don't end up using it. */
        stdin.lock().read_line(&mut line).unwrap();
        println!("The string is: {}", line);
        line.clear();
    }
}
