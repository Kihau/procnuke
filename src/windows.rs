use std::mem;
use std::ffi::CString;

use crate::winapi_bindings::*;

pub type ProcessID = DWORD;

pub fn get_all_process_ids() -> Vec<ProcessID> {
    let mut all_pids = Vec::<ProcessID>::new();

    unsafe {
        const CAP: usize = 1024;
        let mut buffer = [0; CAP];
        let buffer_size = mem::size_of::<[ProcessID; CAP]>() as u32;
        let mut bytes_read: DWORD = 0;

        if EnumProcesses(buffer.as_mut_ptr(), buffer_size, &mut bytes_read as LPDWORD) == 0 {
            eprintln!("ERROR: Failed to retrive process ids");
            return all_pids;
        }

        let process_count = bytes_read as usize / mem::size_of::<ProcessID>();
        all_pids.extend_from_slice(&buffer[0..process_count]); 
    }
    all_pids
}

pub fn get_process_name(process_id: ProcessID) -> Option<String> {
    unsafe {
        let process = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, process_id);
        let mut module_handles: HMODULE = NULL;
        let mut bytes_required: DWORD = 0;

        if process == NULL {
            CloseHandle(process);
            return None;
        }

        let result = EnumProcessModules(
            process, &mut module_handles as *mut HMODULE,
            mem::size_of::<HMODULE>() as DWORD,
            &mut bytes_required as LPDWORD
        ); 

        if result == 0 {
            CloseHandle(process);
            return None;
        }

        const CAP: usize = 1024;
        let mut string_buffer = [0; CAP];
        let buffer_size = (mem::size_of::<[WCHAR; CAP]>() / mem::size_of::<WCHAR>()) as u32;
        let string_length = GetModuleBaseNameW(
            process, module_handles, string_buffer.as_mut_ptr(), buffer_size
        ) as usize;
        CloseHandle(process);

        let process_name = String::from_utf16_lossy(&string_buffer[0..string_length]);
        Some(process_name)
    }
}

pub fn get_process_cmdline(process_id: ProcessID) -> Option<String> {
    unsafe {
        let ntdll = CString::new("ntdll.dll").unwrap();
        let module_handle = LoadLibraryA(ntdll.as_ptr());

        let function = CString::new("NtQueryInformationProcess").unwrap();
        let handle_ptr = GetProcAddress(module_handle, function.as_ptr());

        if handle_ptr == NULL {
            return None;
        }

        #[allow(non_snake_case)]
        let NtQueryInformationProcess = mem::transmute::<FARPROC, fn(
            ProcessHandle: HANDLE, 
            ProcessInformationClass: PROCESSINFOCLASS,
            ProcessInformation: PVOID, 
            ProcessInformationLength: ULONG,
            ReturnLength: PULONG
        ) -> NTSTATUS>(handle_ptr);


        let process = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, process_id);

        let mut bytes_read = 0u32;

        let mut process_info = PROCESS_BASIC_INFORMATION::default();
        let process_info_size = mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32;
        let result = NtQueryInformationProcess(
            process, ProcessBasicInformation, RawMutPtr(&mut process_info as PPBI as *mut void),
            process_info_size, &mut bytes_read as PULONG
        );

        if result == 0 {
            CloseHandle(process);
            return None;
        }

        let mut peb_data = PEB::default();
        let peb_size = mem::size_of::<PEB>() as u64;
        let result = ReadProcessMemory(
            process, process_info.PebBaseAddress.0 as LPCVOID, &mut peb_data as *mut PEB as LPVOID,
            peb_size, &mut bytes_read as *mut u32 as *mut SIZE_T
        );

        if result == 0 {
            CloseHandle(process);
            return None;
        }

        let mut rtl_data = RTL_USER_PROCESS_PARAMETERS::default();
        let rtl_size = mem::size_of::<RTL_USER_PROCESS_PARAMETERS>() as u64;
        let result = ReadProcessMemory(
            process, peb_data.ProcessParameters.0 as LPCVOID, &mut rtl_data as *mut RTL_USER_PROCESS_PARAMETERS as LPVOID,
            rtl_size, &mut bytes_read as *mut u32 as *mut SIZE_T
        );

        if result == 0 {
            CloseHandle(process);
            return None;
        }

        let mut buffer = vec![0u16; (rtl_data.CommandLine.Length / 2) as usize];
        let result = ReadProcessMemory(
            process, rtl_data.CommandLine.Buffer.0 as LPCVOID, buffer.as_mut_ptr() as LPVOID,
            rtl_data.CommandLine.Length as u64, &mut bytes_read as *mut u32 as *mut SIZE_T
        );

        if result == 0 {
            CloseHandle(process);
            return None;
        }

        let command_line = String::from_utf16_lossy(&buffer[..]);

        CloseHandle(process);
        Some(command_line)
    }
}

pub fn kill_process(process_id: ProcessID) {
    unsafe {
        let process = OpenProcess(PROCESS_TERMINATE, 0, process_id);
        // Terminate is asynchronous, I might need to call WaitForSingleObject and check if
        // termination completed successfully after the timeout
        TerminateProcess(process, 0);
        CloseHandle(process);
    }
}

// pub fn kill_processes(pids: Vec<ProcessID>) {
//     for pid in pids {
//         println!("Killing process: {pid}");
//         unsafe {
//             let process = OpenProcess(PROCESS_TERMINATE, 0, pid);
//             // Terminate is asynchronous, I might need to call WaitForSingleObject and check if
//             // termination completed successfully after the timeout
//             TerminateProcess(process, 0);
//             CloseHandle(process);
//         }
//     }
// }
