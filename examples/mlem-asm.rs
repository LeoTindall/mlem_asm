extern crate mlem_asm;
use mlem_asm::{parse_program, program_to_writer};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Stdout};
use std::fs::File;

fn usage() {
    println!("Usage: ");
    println!("\tmlem-asm a source [output]\t\t Assemble the given source file to the given output file, or stdout.");
    println!("\tmlem-asm r source \t\t Assemble the given source file and execute it, hooking up stdin and stdout.");
}

enum Output {
    File(BufWriter<File>),
    StdOut(BufWriter<Stdout>)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 3 || args.len() < 2 {
        usage();
        std::process::exit(101);
    }

    let mut f = BufReader::new(File::open(&args[1]).unwrap());
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();


    let out: Output =
        if args.len() == 3 {
            Output::File(BufWriter::new(File::create(&args[2]).unwrap()))
        } else {
            Output::StdOut(BufWriter::new(std::io::stdout()))
        };

    let result = parse_program(&contents);

    match result {
        Ok(p) => {
            match out {
                Output::File(mut o) => { program_to_writer(&p, &mut o).unwrap(); },
                Output::StdOut(mut o) => { program_to_writer(&p, &mut o).unwrap(); }
            }
         }
        Err(e) => {
            println!("Could not assemble program.");
            for error in e {
                println!("{:2}: {}", error.0, error.1);
            }
            std::process::exit(1);
        }
    }    
}