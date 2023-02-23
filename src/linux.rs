use std::{path::{Path}, fs::{self, read_to_string}};

pub type ProcessID = pid_t;

//
// Libc bindings
//
// Process "kill" signal code
const SIGKILL: i32 = 9;

// Process id
#[allow(non_camel_case_types)]
type pid_t = u32;

extern "C" {
    fn kill(pid: pid_t, sig: i32) -> i32;
}

pub fn get_all_process_ids() -> Vec<ProcessID> {
    let mut all_pids = Vec::<ProcessID>::new();
    let proc = Path::new("/proc/");

    // Assuming that the program has permissions to the proc dir
    for dir in fs::read_dir(proc).unwrap() {
        let dir = dir.unwrap();

        // The directory names shoule be fine here
        let dirname = dir.file_name().into_string().unwrap();

        // Only iterate over the pids directories
        let pid = dirname.parse::<pid_t>();
        if pid.is_err() {
            continue;
        }
        let pid = pid.unwrap();
        all_pids.push(pid);
    }

    return all_pids;
}

pub fn full_name_matching(process_id: ProcessID, pattern: &String) -> bool {
    let cmdline_string = format!("/proc/{process_id}/cmdline");
    let cmdline_path = Path::new(&cmdline_string);
    let cmdline = read_to_string(cmdline_path);

    if cmdline.is_err() {
        return false;
    }
    let full_name = cmdline.unwrap();

    if full_name.contains(pattern.to_lowercase().as_str()) {
        return true;
    }

    return false;
}

pub fn program_name_matching(process_id: ProcessID, pattern: &String) -> bool {
    let status_string = format!("/proc/{process_id}/status");
    let status_path = Path::new(&status_string);
    let status = read_to_string(status_path);

    if status.is_err() {
        return false;
    }
    let status = status.unwrap();

    let mut input = status.lines();
    let name = input.next().unwrap().split("\t").skip(1).next().unwrap();

    if name.contains(pattern.to_lowercase().as_str()) {
        return true;
    }

    return false;
}

pub fn kill_processes(pids: Vec<ProcessID>) {
    for pid in pids {
        println!("Killing process: {pid}");
        unsafe {
            kill(pid, SIGKILL);
        }
    }
}


