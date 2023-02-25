#[cfg(target_os = "linux")]
use crate::linux::*;
#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

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
