use std::env;
use std::io;
use std::process;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let line_to_look_for = &args[1..].join(" ");
    let line_to_look_for = line_to_look_for.trim();

    if line_to_look_for.is_empty() {
        println!("Cannot look for an empty string! Please provide command line arguments");
        process::exit(1);
    }

    println!("Will look for: {:?}", line_to_look_for);
    let program_start = Instant::now();

    let mut buffer = String::new();
    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                if buffer.contains(line_to_look_for) {
                    println!("Match! {}", buffer.trim_end());
                    println!("Took {} ms", program_start.elapsed().as_millis());
                }

                buffer.clear();
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
