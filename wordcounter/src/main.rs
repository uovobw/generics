use std::fs::File;
use std::io::Read;
use std::env;
use std::process;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: wordcounter <file> [print_tree]");
        process::exit(1);
    }

    let filename = &args[1];

    let mut f = match File::open(filename) {
        Ok(fd) => fd,
        Err(e) => {
            println!("Error opening file {}: {}", filename, e);
            process::exit(1);
        }
    };

    let mut map: HashMap<String, u32> = HashMap::new();

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(size) => println!("loaded correctly {}", size),
        Err(e) => println!("error loading: {}", e),
    }
    for line in contents.lines() {
        for word in line.split_whitespace() {
            let counter = map.entry(String::from(word)).or_insert(0);
            *counter += 1;
        }
    }

    println!("Len: {}", map.len());
//    if args.len() == 3 {
//        println!("Dict: {:#?}", map);
//    }
    let mut reverse_map: HashMap<u32, Vec<String>> = HashMap::new();
    for (k, v) in map.iter() {
        if reverse_map.contains_key(v) {
            let this = reverse_map.get_mut(v).expect("BUG!");
            (*this).push((*k).clone());
        } else {
            reverse_map.insert(*v, vec![(*k).clone()]);
        }
    }
    let maximum = reverse_map.keys().max().expect("tremendo!");
    println!("most common word: {:#?} ({} times)", reverse_map.get(maximum).expect("fatal!")[0], maximum);}
