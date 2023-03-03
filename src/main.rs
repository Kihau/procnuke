use std::{path::Path};

use procnuke::*;

#[cfg(target_os = "windows")]
use procnuke::windows::*;

#[cfg(target_os = "linux")]
use procnuke::linux::*;

fn get_program_name(program_path: String) -> Option<String> {
    let path = Path::new(&program_path);
    let os_name = path.file_name()?.to_str()?;
    return Some(String::from(os_name));
}

fn main() {
    let mut args = std::env::args();
    let exec_path = args.next().unwrap();
    let exec_name = get_program_name(exec_path).unwrap();

    let cmdline_args: Vec<String> = args.collect();
    let mut config = Config::default();

    let mut kill_args = Vec::new();

    if cmdline_args.is_empty() {
        print_help(&exec_name);
    }

    for arg in cmdline_args {
        match arg.as_str() {
            "-c" | "--cmdline"             => config.match_cmdline = true,
            "-s" | "--case-sensitive"      => config.case_sensitive = true,
            "-e" | "--exact"               => config.match_exact = true,
            "-l" | "--list"                => config.listing = true,
            "-p" | "--pid"                 => config.match_pid = true,
            "-i" | "--ignore-unrecognised" => config.ignore_unrecognised = true,
            "-v" | "--version"             => print_version(),
            "-h" | "--help"                => print_help(&exec_name),
            // TODO: -j, --join - join the string together and dont treat is as separate arguments
            _                              => kill_args.push(arg),
        }
    }

    if !config.ignore_unrecognised {
        for arg in &kill_args {
            if arg.starts_with('-') {
                eprintln!("ERROR: Unrecognised option '{arg}'. Use {exec_name} --help to list available options.");
                return;
            }
        }
    }

    // Handle some error cases
    let mut error_occurred = true;
    match config {
        Config { match_pid: true, match_cmdline: true, .. } => 
            eprintln!("ERROR: Option for matching by PID cannot be used with option for command line matching"),
        Config { match_pid: true, match_exact: true, .. } =>
            eprintln!("ERROR: Option for matching by PID cannot be used with option for exact string matching"),
        Config { match_pid: true, .. } if kill_args.is_empty() && !config.listing =>
            eprintln!("ERROR: You must provide one or more process ids to kill. Use {exec_name} --help for more info."),
        _ if kill_args.is_empty() && !config.listing  => /* print_help(&exec_name), */
            eprintln!("ERROR: You must provide a process name to kill. Use {exec_name} --help for more info."),
        _ => error_occurred = false,
    }

    if error_occurred {
        return;
    }

    let mut pids = if !config.match_pid {
        get_matching_by_string(&config, &kill_args)
    } else {
        get_matching_by_pid(&kill_args)
    };

    if !config.listing {
        if pids.len() == 0 {
            println!("No matching processes found.");
            return;
        }

        kill_processes(pids);
    } else {
        if pids.len() == 0 && kill_args.is_empty(){
            pids = get_all_process_ids();
        }
        list_processes(pids);
    }
}
