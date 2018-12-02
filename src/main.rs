extern crate mlem;
extern crate mlem_asm;
use mlem_asm::{parse_program, program_to_writer};
use mlem::Machine;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Stdout, Stdin};
use std::fs::File;
use std::borrow::Borrow;

fn usage() {
    println!("Usage: ");
    println!("\tmlem-asm a source [output]\tAssemble the given source file to the given output file, or stdout.");
    println!("\tmlem-asm r source [input]\t\tAssemble the given source file and execute it, hooking up stdin and stdout.");
}

enum Output {
    File(BufWriter<File>),
    StdOut(BufWriter<Stdout>)
}

enum Input {
    File(BufReader<File>),
    StdIn(BufReader<Stdin>)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Assemble,
    Execute
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 4 || args.len() < 3 {
        usage();
        std::process::exit(101);
    }

    let mode = match args[1].borrow() {
        "a" => Mode::Assemble,
        "r" => Mode::Execute,
        other => { panic!("Unknown mode {}; try a for assemble or r for run", other); }
    };

    load_and_process(args, mode);
    
}

fn load_and_process(args: Vec<String>, mode: Mode) {
    let mut f = BufReader::new(File::open(&args[2]).unwrap());
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut out: Option<Output> = None;
    let mut inp: Option<Input> = None;

    match mode {
        Mode::Assemble => {
            out = Some(if args.len() == 4 {
                Output::File(BufWriter::new(File::create(&args[3]).unwrap()))
            } else {
                Output::StdOut(BufWriter::new(std::io::stdout()))
            });
        },
        Mode::Execute => {
            inp = Some(if args.len() == 4 {
                Input::File(BufReader::new(File::create(&args[3]).unwrap()))
            } else {
                Input::StdIn(BufReader::new(std::io::stdin()))
            });
        }
    };

    let result = parse_program(&contents);

    match result {
        Ok(p) => {
            match mode {
                Mode::Assemble => {
                    match out.unwrap() {
                        Output::File(mut o) => { program_to_writer(&p, &mut o).unwrap(); },
                        Output::StdOut(mut o) => { program_to_writer(&p, &mut o).unwrap(); }
                    }
                },
                Mode::Execute => {
                    match inp.unwrap() {
                        Input::File(mut i) => { 
                            let mut o = std::io::stdout();
                            let mut m = Machine::new(65535, &mut i, &mut o);
                            m.load_program(p);
                            println!("\n{:?}", m.run()) 
                        },
                        Input::StdIn(mut i) => { 
                            let mut o = std::io::stdout();
                            let mut m = Machine::new(65535, &mut i, &mut o);
                            m.load_program(p);
                            println!("\n{:?}", m.run()) 
                        }
                    }
                }
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