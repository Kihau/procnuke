#[cfg(target_os = "linux")]
use crate::linux::*;
#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Config {
    pub case_sensitive: bool,
    pub match_cmdline: bool,
    pub match_exact: bool,
    pub match_pid: bool,
    pub listing: bool,

}

impl Default for Config {
    fn default() -> Self {
        Self {
            match_cmdline: false,
            case_sensitive: false,
            match_exact: false,
            match_pid: false,
            listing: false,
        }
    }
}

pub fn print_help(program_name: &String) {
    println!("ProcNuke v{VERSION} (also known as fuckoff): Simple process killer.");
    println!();
    println!("Usage:");
    println!("    {program_name} -a [string to match against]");
    println!("    {program_name} [program name]");
    println!("    TODO: Add some more");
    println!();
    println!("Options:");
    println!("    -c, --cmdline           Matches processes by their name AND their execution arguments");
    println!("    -s, --case-sensitive    Don't ignore case when matching strings");
    println!("    -e, --exact             Match exact string");
    println!("    -l, --list              List processes");
    println!("    -p, --pid               Find/Kill process by pid");
    println!("    -h, --help              Display this prompt");
    std::process::exit(0);
}

pub fn print_version() {
    println!("Version: ProcNuke-v{VERSION}");
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

    return matched_pids;
}

pub fn get_matching_by_string(config: &Config, kill_args: &Vec<String>) -> Vec<ProcessID> {
    let mut matched_pids = Vec::<ProcessID>::new();

    let self_pid = std::process::id();
    println!("Self pid is {self_pid}");

    let all_pids = get_all_process_ids();
    let mut kill_string = kill_args.join(" ");
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

    return matched_pids;
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


/// Retrieves process ID's by matching the process program name with provided pattern
pub fn get_matching_pids_name(pattern: &String) -> Vec<ProcessID> {
    let all_pids = get_all_process_ids();

    let self_pid = std::process::id();
    println!("Self pid is {self_pid}");

    let mut matched_pids = Vec::new();
    for pid in all_pids {
        // Return process name instead of bool, allows for more extensibility, if I want to expand
        // on this program
        //                                vvvvvvvvvvvvvvvvvvvvv
        if pid != 0 && pid != self_pid && program_name_matching(pid, &pattern) {
            matched_pids.push(pid);
        }
    }
    return matched_pids;
}

/// Retrieves process ID's by matching process name and execution arguments with provided pattern
pub fn get_matching_pids_full(pattern: &String) -> Vec<ProcessID> {
    let all_pids = get_all_process_ids();

    let self_pid = std::process::id();
    println!("Self pid is {self_pid}");

    let mut matched_pids = Vec::new();
    for pid in all_pids {
        // Return full commandline name instead of bool, allows for more extensibility, if I want to expand
        // on this program
        //                                vvvvvvvvvvvvvvvvvv
        if pid != 0 && pid != self_pid && full_name_matching(pid, &pattern) {
            matched_pids.push(pid);
        }
    }
    return matched_pids;
}
