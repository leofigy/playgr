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
                let out = match fs::read_to_string(filename){
                    Ok(content) => content,
                    Err(fatal) => fatal.to_string()
                };
                println!("{}",out);
            },
            _ => println!("nothing to get")
        }
    }
}
