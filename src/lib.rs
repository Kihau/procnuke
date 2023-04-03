#[cfg(target_os = "linux")]
use crate::linux::*;
#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub mod winapi_bindings;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// Shouldnt be part of the lib?
#[derive(Default)]
pub struct Config {
    pub case_sensitive: bool,
    pub match_cmdline: bool,
    pub match_exact: bool,
    pub match_pid: bool,
    pub listing: bool,
    pub ignore_unrecognised: bool,
}

pub fn print_help(program_name: &String) {
    println!("ProcNuke (also known as fuckoff): Simple process killer.");
    println!();
    println!("Example usage:");
    println!("    {program_name} [program name]");
    println!("    {program_name} -c [string to match against]");
    println!("    {program_name} -p [process id]");
    println!("    {program_name} -c [string to match] --list");
    println!("    {program_name} [program name] --exact -s");
    println!();
    println!("Options:");
    println!("    -c, --cmdline                Matches processes by their name AND their execution arguments");
    println!("    -s, --case-sensitive         Don't ignore case when matching strings");
    println!("    -e, --exact                  Match exact string");
    println!("    -p, --pid                    Match process by their process IDs");
    println!("    -l, --list                   List processes (can be used with other matching options)");
    println!("    -i, --ignore-unrecognised    Ignore unrecognised command line options and treat them as regular input arguments");
    println!("    -v, --version                Display version number and other information");
    println!("    -h, --help                   Display this prompt");
    std::process::exit(0);
}

pub fn print_version() {
    println!("Version: ProcNuke-v{VERSION}");
    println!("Author:  {AUTHORS}");
    println!("Source:  {REPOSITORY}");
    std::process::exit(0);
}

pub fn list_processes(pids_to_list: Vec<ProcessID>) {
    for pid in pids_to_list {
        if let Some(process_name) = get_process_name(pid) {
            println!("{pid} - {process_name}");
        } else {
            println!("{pid} - <Unavailable>");
        }
    }

    std::process::exit(0);
}

pub fn get_matching_by_pid(kill_args: &Vec<String>) -> Vec<ProcessID> {
    let mut matched_pids = Vec::<ProcessID>::new();

    let self_pid = std::process::id();
    println!("Self pid is {self_pid}");

    let all_pids = get_all_process_ids();

    for arg in kill_args {
        let Ok(pid) = arg.parse::<ProcessID>() else {
            eprintln!("ERROR: Failed to parse: {arg}");
            continue;
        };
        
        if self_pid != pid && all_pids.contains(&pid) {
            matched_pids.push(pid);
        }
    }

    matched_pids
}

pub fn get_matching_by_string(config: &Config, kill_args: &Vec<String>) -> Vec<ProcessID> {
    let mut matched_pids = Vec::<ProcessID>::new();

    let self_pid = std::process::id();
    println!("Self pid is {self_pid}");

    let all_pids = get_all_process_ids();
    for pid in all_pids {
        if pid == self_pid {
            continue;
        }

        let process_string = if config.match_cmdline {
            get_process_cmdline(pid)
        } else { 
            get_process_name(pid)
        };

        let Some(mut process_string) = process_string else {
            continue;
        };

        for arg in kill_args {
            let mut kill_string = String::from(arg);
            if !config.case_sensitive {
                process_string = process_string.to_lowercase();
                kill_string = kill_string.to_lowercase();
            }

            let string_matching = if config.match_exact {
                kill_string == process_string
            } else {
                process_string.contains(&kill_string)
            };

            if string_matching {
                matched_pids.push(pid);
            }
        }
    }

    matched_pids
}

pub fn kill_processes(pids: Vec<ProcessID>) {
    for pid in pids {
        if let Some(process_name) = get_process_name(pid) {
            println!("Killing process: {pid} - {process_name}");
        } else {
            println!("Killing process: {pid} - <Unavailable>");
        }
        kill_process(pid);
    }
}
