use std::path::Path;

use procnuke::*;

#[cfg(target_os = "windows")]
use procnuke::windows::*;

#[cfg(target_os = "linux")]
use procnuke::linux::*;

fn print_help(program_name: &String) {
    println!("ProcNuke (also known as fuckoff): Simple process killer.");
    println!();
    println!("Usage:");
    println!("    {program_name} -a [string to match against]");
    println!("    {program_name} [program name]");
    println!();
    println!("Options:");
    println!("    -a, --aggressive    Aggressive mode. Matches processes by their name and execution arguments");
    println!("    -h, --help          Display this prompt");
}

fn get_program_name(program_path: String) -> Option<String> {
    let path = Path::new(&program_path);
    let os_name = path.file_name()?.to_str()?;
    return Some(String::from(os_name));
}

// TODO: Add more options:
//           -s, --case-sensitice     case sensitive
//           -l, --list               list processes with pids, don't kill anything
//           -p, --pid                kill by pid
//           -os, --operating-system  shutdown operating system (requested by frisk)

fn main() {
    let mut args = std::env::args();
    let program_path = args.next().unwrap();
    let program_name = get_program_name(program_path).unwrap();

    let cmdline_args: Vec<String> = args.collect();
    let mut aggressive = false;

    let mut kill_string = String::new();

    for arg in cmdline_args {
        match arg.as_str() {
            "-a" | "--aggressive" => aggressive = true,
            "-h" | "--help" => {
                print_help(&program_name);
                return;
            }
            _ => kill_string.push_str(&arg),
        }
    }

    if kill_string.is_empty() {
        eprintln!("You must provide a process name to kill. Use {program_name} --help for more info.");
        return;
    }

    let pids = if aggressive {
        get_matching_pids_full(&kill_string)
    } else {
        get_matching_pids_name(&kill_string)
    };

    kill_processes(pids);
}
