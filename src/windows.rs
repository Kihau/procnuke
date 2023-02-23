pub type ProcessID = DWORD;

//
// WinApi bindings
//
type BOOL = i32;
type DWORD = u32;
type LPDWORD = *mut DWORD;

#[allow(non_camel_case_types)]
enum void {}

type HANDLE = *mut void;
type HINSTANCE = HANDLE;
type HMODULE = HINSTANCE;
type WCHAR = u16;
type LPWSTR = *mut WCHAR;

type UINT = u32;

const READ_CONTROL: DWORD = 0x00020000;
const SYNCHRONIZE: DWORD = 0x00100000;
const PROCESS_TERMINATE: DWORD = 0x0001;
const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
const PROCESS_VM_READ: DWORD = 0x0010;
const NULL: *mut void = 0 as *mut void;

#[link(name = "psapi")]
extern "system" {
    fn EnumProcesses(lpidProcess: *mut DWORD, cb: DWORD, lpcbNeeded: LPDWORD) -> BOOL;
    fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE; 
    // Requires PROCESS_TERMINATE process access right
    fn TerminateProcess(hProcess: HANDLE, uExitCode: UINT) -> BOOL;
    fn EnumProcessModules(hProcess: HANDLE, lphModule: *mut HMODULE, cb: DWORD, lpcbNeeded: LPDWORD) -> BOOL;
    // Requires PROCESS_QUERY_INFORMATION and PROCESS_VM_READ process access right
    fn GetModuleBaseNameW(hProcess: HANDLE, hModule: HMODULE, lpBaseName: LPWSTR, nSize: DWORD) -> DWORD;
    fn CloseHandle(hObject: HANDLE) -> BOOL;
}

pub fn get_all_process_ids() -> Vec<ProcessID> {
    use std::mem;
    let mut all_pids = Vec::<ProcessID>::new();

    unsafe {
        let mut buffer = [0; 1024];
        let buffer_size = mem::size_of::<[ProcessID; 1024]>() as u32;
        let mut bytes_read: DWORD = 0;

        if EnumProcesses(buffer.as_mut_ptr(), buffer_size, &mut bytes_read as LPDWORD) == 0 {
            eprintln!("ERROR: Failed to retrive process ids");
            return all_pids;
        }

        let process_count = bytes_read as usize / mem::size_of::<ProcessID>();
        all_pids.extend_from_slice(&buffer[0..process_count]); 
    }
    return all_pids;
}

pub fn program_name_matching(pid: ProcessID, pattern: &String) -> bool {
    use std::mem;

    unsafe {
        let process = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid);
        let mut module_handles: HMODULE = NULL;
        let mut bytes_required: DWORD = 0;

        if process != NULL {
            let result = EnumProcessModules(
                process, &mut module_handles as *mut HMODULE,
                mem::size_of::<HMODULE>() as DWORD,
                &mut bytes_required as LPDWORD
            ); 

            if result != 0 {
                let mut string_buffer = [0; 1024];
                let buffer_size = (mem::size_of::<[WCHAR; 1024]>() / mem::size_of::<WCHAR>()) as u32;
                let string_length = GetModuleBaseNameW(
                    process, module_handles, string_buffer.as_mut_ptr(), buffer_size
                ) as usize;

                let process_name = String::from_utf16_lossy(&string_buffer[0..string_length]).to_lowercase();
                if process_name.contains(pattern.to_lowercase().as_str()) {
                    return true;
                }
            }
        }
        CloseHandle(process);
    }
    return false;
}

// TODO: Mathing by parameters
//       https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntqueryinformationprocess?redirectedfrom=MSDN
pub fn full_name_matching(process_id: ProcessID, pattern: &String) -> bool {
    unimplemented!()
}

pub fn kill_processes(pids: Vec<ProcessID>) {
    for pid in pids {
        println!("Killing process: {pid}");
        unsafe {
            let process = OpenProcess(PROCESS_TERMINATE, 0, pid);
            // Terminate is asynchronous, I might need to call WaitForSingleObject and check if
            // termination completed successfully after the timeout
            TerminateProcess(process, 0);
            CloseHandle(process);
        }
    }
}
