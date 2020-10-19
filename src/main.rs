use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    if args.len() <= 2 {
        println!("please add more args");
        return
    }

    let pattern = &args[1];

    for i in 2..args.len() as usize {
        match args.get(i){
            Some(filename) => {
                let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file");
                println!("With text:\n{}", contents);
            },
            _ => println!("nothing to get")
        }
    }
}
