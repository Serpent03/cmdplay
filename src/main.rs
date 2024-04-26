// TODO web query for the youtube API
// TODO queue based playlist(?)
// TODO enable multi-threading(low priority)

use std::io::{ self, BufRead, BufWriter };
use rustube::Video;

// FIX returning promise/future for the video download
fn get_audio(url: &str) -> _ {
    return Video::from_id(url); 
}

fn main() {
    println!("Starting download!");
    let stdin = io::stdin();
    let mut line:String = String::new();

    // one thread handles music and another thread handles play/pause
    // for _i in 0..5 {
    //     stdin.lock().read_line(&mut line).unwrap();
    //     println!("The string is: {}", line);
    //     line.clear();
    // }
    
    let url: &str = "https://youtu.be/nv2wQvn6Wxc";
    let path_to_video = rustube::blocking::download_best_quality(url);

    println!("Video Downloaded!");
}
