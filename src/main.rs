use std::time::{Duration, Instant};
mod download;
use download::download_file;
use std::thread;

const URL: &str = "https://royalcast.com.br/files";

fn main() {
    let mut handles =  vec![];

    let filename_list = vec![
        "arquivo_1.jpg",
        "arquivo_2.jpg",
        "arquivo_3.jpg",
        "arquivo_4.jpg",
        "arquivo_5.jpg",
        "arquivo_6.jpg",
        "arquivo_7.jpg",
        "arquivo_8.jpg",
        "arquivo_9.jpg",
    ];

    let start = Instant::now();
    for filename in filename_list {
        let h = thread::spawn(||{
            download_file(URL, filename);
        });
        handles.push(h);
    }
    for h in handles{
        h.join().unwrap();
    }
    let duration: Duration = start.elapsed();
    println!(
        "Downloaded files in {:.1} seconds",
        duration.as_millis() as f32 / 1000.
    );
}

