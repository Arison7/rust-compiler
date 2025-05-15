use std::env;
use std::fs::File;

pub fn get_file() -> (File,String) {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Program requires only one argument {:?} were provided: {:?}",args.len(),args)
    }

    let filename = &args[1];

    (File::open(filename).expect("file not found "),filename[..filename.len()-2].to_string())


}
