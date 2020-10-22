use std::env;
use std::fs;

#[derive(Debug)]
struct Config {
    filename : String,
    content : String,
}

#[derive(Debug)]
struct Search {
    query: String,
    data : Vec<Config>,
}


fn main() {
    let args : Vec<String> = env::args().collect();
    
    if args.len() <= 2 {
        println!("please add more args");
        return
    }

    let mut search_data = Search{
        query: args[1].clone(),
        data: Vec::new(),
    };

    for i in 2..args.len() as usize {
        match args.get(i){
            Some(filename) => {
                match fs::read_to_string(filename){
                    Ok(content) => {
                        search_data.data.push(Config{
                            filename: filename.to_string(),
                            content: content,
                        })
                    },
                    Err(fatal) => println!("{}", fatal)
                };
            },
            _ => println!("nothing to get")
        }
    }

    for data in search_data.data{
        println!("{:?}", data);
        let result = search(&search_data.query, &data.content);
        println!("matches found {:?}", result);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines(){
      if line.contains(query){
        results.push(line);
      }
  }
  results
}