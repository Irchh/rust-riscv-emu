use std::{fs, env};
use crate::Flags::{Interactive, File};
use std::io::{Write, Read};
use crate::Cmd::*;

mod cpu;

#[derive(Debug)]
#[derive(PartialOrd, PartialEq)]
enum Flags {
    Interactive,
    File{path: String},
}

fn parse_arg(args: &Vec<String>, index: usize) -> Vec<Flags> {
    let mut i = 0;
    let mut opts: Vec<Flags> = vec!();
    for ch in args[index].chars() {
        if i == 0 && ch != '-' {
            opts.push(File{path: args[index].clone()});
            return opts;
        } else if i == 0 && ch == '-' {
            i += 1;
            continue;
        }

        match ch {
            'i' => /* Interactive */ {
                opts.push(Interactive);
            }
            _ => {
                println!("Unknown arg: {}", ch);
            }
        }
        i += 1;
    }
    opts
}

fn get_input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s = String::new();

    print!("> ");
    stdout().flush();
    stdin().read_line(&mut s);
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

#[derive(Debug)]
#[derive(PartialOrd, PartialEq)]
enum Cmd {
    Step,
    PrintAll,
    PrintRegs,
    PrintMemRegion{ addr: usize, len: usize},
    Nothing,
}

fn parse_cmd(s: String) -> Cmd {
    let c = s.split_whitespace().collect::<Vec<&str>>();
    for mut i in 0..c.len() {
        match c[i] {
            "p" | "print" => {
                if i >= c.len()-1 {
                    return PrintAll;
                }
                i += 1;
                match c[i] {
                    "r" | "reg" | "register" => {
                        return PrintRegs;
                    }
                    "m" | "mem" | "memory" => {
                        if i >= c.len()-1 {
                            return PrintMemRegion { addr: 0, len: 16 };
                        }
                        let mut _f = c[i+1].parse::<u64>();
                        if _f.is_err() {
                            _f = u64::from_str_radix(c[i+1].trim_start_matches("0x"), 16);
                            if _f.is_err() {
                                println!("Not valid address: {:?}", c[i+1]);
                                return Nothing;
                            }
                        }
                        let f = _f.unwrap() as usize;
                        if i >= c.len()-2 {
                            return PrintMemRegion { addr: f, len: 16 };
                        }
                        let mut _l = c[i+2].parse::<u64>();
                        if _l.is_err() {
                            _l = u64::from_str_radix(c[i+2].trim_start_matches("0x"), 16);
                            if _l.is_err() {
                                println!("Not valid address: {:?}", c[i+2]);
                                return Nothing;
                            }
                        }
                        let l = _l.unwrap() as usize;
                        return PrintMemRegion { addr: f, len: l };
                    }
                    s => {
                        println!("Unknown print cmd: {}, printing all", s);
                        return PrintAll;
                    }
                }
            }
            "s" | "step" => {
                return Step;
            }
            st => {
                println!("CMD: {}", st);
                return Nothing;
            }
        }
    }
    Nothing
}

fn main() {
    let buffer: Vec<u8>;
    let args: Vec<String> = env::args().collect();

    // parse all args
    let mut pargs: Vec<Flags> = vec!();
    for i in 1..args.len() {
        let mut a = parse_arg(&args, i);
        pargs.append(&mut a);
    }
    println!("pargs: {:?}", pargs);

    // Check if we passed in a file
    let file = pargs.iter().find(|s| match s {
        File{..} => true,
        _ => false
    });
    if file.is_some() {
        match file.unwrap() {
            Flags::File { path } => {
                buffer = fs::read(path).expect("Error opening file!")
            }
            _ => {
                // This code will never be reached, but rust complains if it this codepath doesn't exist
                buffer = fs::read("tests/firmware.bin").expect("Could not find firmware!")
            }
        }
    } else {
        buffer = fs::read("tests/firmware.bin").expect("Could not find firmware!");
    }

    let mut rvcpu = cpu::CPU::new(buffer);
    let mut i = 0;
    while rvcpu.is_running() {
        // Enables stepping and printing of regs if we passed in the -i flag
        // TODO: Breakpoints and continuous operation.
        if pargs.contains(&Interactive) {
            let input = get_input();
            let cmd = parse_cmd(input);
            println!("Command: {:?}", cmd);
            match cmd {
                Step => {
                    i+=1;
                    rvcpu.step();
                }
                PrintAll => {
                    rvcpu.print_all();
                }
                PrintRegs => {
                    rvcpu.print_regs();
                }
                Cmd::PrintMemRegion { addr, len } => {
                    rvcpu.print_mem_reg(addr, len);
                }
                Nothing => {}
            }
        }
        else {
            i+=1;
            rvcpu.step();
        }
        if i > 500 {
            break;
        }
    }
    println!("CPU STATE: {}", rvcpu);
}
