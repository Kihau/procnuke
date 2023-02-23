use procnuke::*;

#[cfg(target_os = "windows")]
use procnuke::windows::*;

#[cfg(target_os = "linux")]
use procnuke::linux::*;

// TODO: Change some logic
//       Add "-a" flag to enable agressive matching (matching by process name and arguments)
//       By default only match by processe name
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!("You must provide a process name to kill");
        return;
    }

    let mut kill_string = String::new();
    // Skip the first argument (the program name)
    for i in 1..args.len() {
        kill_string.push_str(&args[i]);
    }

    let pids = get_matching_pids_name(&kill_string);
    // let pids = get_matching_pids_full(&kill_string);
    kill_processes(pids);
}
