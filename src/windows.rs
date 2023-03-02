#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::mem;
use std::ffi::CString;

pub type ProcessID = DWORD;

// Move this doghit garbage to separate file
//
// WinApi bindings
//
#[derive(Copy, Clone)]
enum void {}
type BYTE = u8;
type USHORT = u16;
type WCHAR = u16;
type BOOL = i32;
type UINT = u32;
type DWORD = u32;
type ULONG = u32;
type SIZE_T = u64;
type ULONG_PTR = u64;

type LPWSTR = *mut u16;
type PULONG = *mut u32;
type LPDWORD = *mut u32;

type HANDLE = *mut void;
type HINSTANCE = *mut void;
type HMODULE = *mut void;
type LPVOID = *mut void;
type PVOID = RawMutPtr<void>;

type LPCSTR = *const i8;
type PWSTR = RawMutPtr<WCHAR>;

type KPRIORITY = i32;
type NTSTATUS = i32;
type KAFFINITY = u64;

type LPCVOID = *const void;
type FARPROC = *const void;

type PROCESSINFOCLASS = i32;
const ProcessBasicInformation: PROCESSINFOCLASS = 0;
const ProcessDebugPort: PROCESSINFOCLASS = 7;
const ProcessWow64Information: PROCESSINFOCLASS = 26;
const ProcessImageFileName: PROCESSINFOCLASS = 27;
const ProcessBreakOnTermination: PROCESSINFOCLASS = 29;
const ProcessSubsystemInformation: PROCESSINFOCLASS = 75;

#[derive(Copy, Clone)]
struct RawMutPtr<T>(*mut T);
impl<T> Default for RawMutPtr<T> {
    fn default() -> RawMutPtr<T> {
        return Self(NULL as *mut T)
    }
}

#[repr(C)]
#[derive(Default)]
struct UNICODE_STRING {
    Length: USHORT,
    MaximumLength: USHORT,
    Buffer: PWSTR,
}

#[repr(C)]
#[derive(Default)]
struct RTL_USER_PROCESS_PARAMETERS {
    Reserved1: ReservedData<BYTE, 16>,
    Reserved2: ReservedData<PVOID, 10>,
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
}

type PPEB_LDR_DATA = RawMutPtr<void>; // Placeholder pointer binding, won't be used
type PRTL_USER_PROCESS_PARAMETERS = RawMutPtr<RTL_USER_PROCESS_PARAMETERS>;
type PPS_POST_PROCESS_INIT_ROUTINE = RawMutPtr<void>; // Placeholder pointer binding, won't be used

// Wrapper to implement Default for any array
struct ReservedData<T: Default + Copy, const N: usize>([T; N]);
impl<T: Default + Copy, const N: usize> Default for ReservedData<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

#[repr(C)]
#[derive(Default)]
struct PEB {
    Reserved1: ReservedData<BYTE, 2>,
    BeingDebugged: BYTE,
    Reserved2: ReservedData<BYTE, 1>,
    Reserved3: ReservedData<PVOID, 2>,
    Ldr: PPEB_LDR_DATA,
    ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    Reserved4: ReservedData<PVOID, 3>,
    AtlThunkSListPtr: PVOID,
    Reserved5: PVOID,
    Reserved6: ULONG,
    Reserved7: PVOID,
    Reserved8: ULONG,
    AtlThunkSListPtr32: ULONG,
    Reserved9: ReservedData<PVOID, 45>,
    Reserved10: ReservedData<BYTE, 96>,
    PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    Reserved11: ReservedData<BYTE, 128>,
    Reserved12: ReservedData<PVOID, 1>,
    SessionId: ULONG
}

type PPEB = RawMutPtr<PEB>;

#[repr(C)]
#[derive(Default)]
struct PROCESS_BASIC_INFORMATION {
    ExitStatus: NTSTATUS,
    PebBaseAddress: PPEB,
    AffinityMask: KAFFINITY,
    BasePriority: KPRIORITY,
    UniqueProcessId: ULONG_PTR,
    InheritedFromUniqueProcessId: ULONG_PTR,
}
type PPBI = *mut PROCESS_BASIC_INFORMATION;

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
    fn CloseHandle(hObject: HANDLE) -> BOOL;

    // Requires PROCESS_TERMINATE process access right
    fn TerminateProcess(hProcess: HANDLE, uExitCode: UINT) -> BOOL;

    fn EnumProcessModules(
        hProcess: HANDLE, lphModule: *mut HMODULE, cb: DWORD, lpcbNeeded: LPDWORD
    ) -> BOOL;

    // Requires PROCESS_QUERY_INFORMATION and PROCESS_VM_READ process access right
    fn GetModuleBaseNameW(
        hProcess: HANDLE, hModule: HMODULE, lpBaseName: LPWSTR, nSize: DWORD
    ) -> DWORD;

    // Requires PROCESS_VM_READ process access right
    fn ReadProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T, 
        lpNumberOfBytesRead: *mut SIZE_T
    ) -> BOOL;

    fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    fn LoadLibraryA(lpLibFileName: LPCSTR) -> HMODULE;
}

// #[link(name = "ntdll")]
// extern "system" {
//     // Requires PROCESS_QUERY_INFORMATION and PROCESS_VM_READ process access right
//     // fn NtQueryInformationProcess(
//     //     ProcessHandle: HANDLE, 
//     //     ProcessInformationClass: PROCESSINFOCLASS,
//     //     ProcessInformation: PVOID, 
//     //     ProcessInformationLength: ULONG,
//     //     ReturnLength: PULONG
//     // ) -> NTSTATUS;
// }

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
    return all_pids;
}

pub fn get_process_name(process_id: ProcessID) -> Option<String> {
    unsafe {
        let process = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, process_id);
        let mut module_handles: HMODULE = NULL;
        let mut bytes_required: DWORD = 0;

        if process != NULL {
            let result = EnumProcessModules(
                process, &mut module_handles as *mut HMODULE,
                mem::size_of::<HMODULE>() as DWORD,
                &mut bytes_required as LPDWORD
            ); 

            if result != 0 {
                const CAP: usize = 1024;
                let mut string_buffer = [0; CAP];
                let buffer_size = (mem::size_of::<[WCHAR; CAP]>() / mem::size_of::<WCHAR>()) as u32;
                let string_length = GetModuleBaseNameW(
                    process, module_handles, string_buffer.as_mut_ptr(), buffer_size
                ) as usize;

                let process_name = String::from_utf16_lossy(&string_buffer[0..string_length]);
                CloseHandle(process);
                return Some(process_name);
            }
        }
        CloseHandle(process);
    }
    return None;
}

// TODO: ROBUSTNESS: Handle winapi errors
pub fn get_process_cmdline(process_id: ProcessID) -> Option<String> {
    unsafe {
        let ntdll = CString::new("ntdll.dll").unwrap();
        let module_handle = LoadLibraryA(ntdll.as_ptr());
        // let module_handle = GetModuleHandleA(ntdll.as_ptr());
        // let module_handle = GetModuleHandleA("ntdll.dll".as_ptr());
        // println!("ntdll: {module}", module = module_handle as u32);

        let function = CString::new("NtQueryInformationProcess").unwrap();
        let handle_ptr = GetProcAddress(module_handle, function.as_ptr());
        // println!("handle: {handle}", handle = handle_ptr as u32);

        if handle_ptr == NULL {
            return None;
        }

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
        // println!("DEBUG: bytes read: {bytes_read}");
        // println!("DEBUG: {process_info_size} {addr}", addr = process_info.PebBaseAddress.0 as usize);

        let mut peb_data = PEB::default();
        let peb_size = mem::size_of::<PEB>() as u64;
        // println!("peb size: {peb_size}");

        let result = ReadProcessMemory(
            process, process_info.PebBaseAddress.0 as LPCVOID, &mut peb_data as *mut PEB as LPVOID,
            peb_size, &mut bytes_read as *mut u32 as *mut SIZE_T
        );
        // println!("DEBUG: bytes read: {bytes_read}");

        let mut rtl_data = RTL_USER_PROCESS_PARAMETERS::default();
        let rtl_size = mem::size_of::<RTL_USER_PROCESS_PARAMETERS>() as u64;
        let result = ReadProcessMemory(
            process, peb_data.ProcessParameters.0 as LPCVOID, &mut rtl_data as *mut RTL_USER_PROCESS_PARAMETERS as LPVOID,
            rtl_size, &mut bytes_read as *mut u32 as *mut SIZE_T
        );
        // println!("DEBUG: bytes read: {bytes_read}");
        // println!("len: {}", rtl_data.CommandLine.Length);

        let mut buffer = vec![0u16; (rtl_data.CommandLine.Length / 2) as usize];
        let result = ReadProcessMemory(
            process, rtl_data.CommandLine.Buffer.0 as LPCVOID, buffer.as_mut_ptr() as LPVOID,
            rtl_data.CommandLine.Length as u64, &mut bytes_read as *mut u32 as *mut SIZE_T
        );
        // println!("DEBUG: bytes read: {bytes_read}");
        let command_line = String::from_utf16_lossy(&buffer[..]);
        // println!("{command_line}");

        CloseHandle(process);
        return Some(command_line);
    }
}

pub fn program_name_matching(pid: ProcessID, pattern: &String) -> bool {
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
                const CAP: usize = 1024;
                let mut string_buffer = [0; CAP];
                let buffer_size = (mem::size_of::<[WCHAR; CAP]>() / mem::size_of::<WCHAR>()) as u32;
                let string_length = GetModuleBaseNameW(
                    process, module_handles, string_buffer.as_mut_ptr(), buffer_size
                ) as usize;

                let process_name = String::from_utf16_lossy(&string_buffer[0..string_length]).to_lowercase();
                if process_name.contains(pattern.to_lowercase().as_str()) {
                    CloseHandle(process);
                    return true;
                }
            }
        }
        CloseHandle(process);
    }
    return false;
}


pub fn full_name_matching(pid: ProcessID, pattern: &String) -> bool {
    unsafe {
        let ntdll = CString::new("ntdll.dll").unwrap();
        let module_handle = LoadLibraryA(ntdll.as_ptr());
        // let module_handle = GetModuleHandleA(ntdll.as_ptr());
        // let module_handle = GetModuleHandleA("ntdll.dll".as_ptr());
        // println!("ntdll: {module}", module = module_handle as u32);

        let function = CString::new("NtQueryInformationProcess").unwrap();
        let handle_ptr = GetProcAddress(module_handle, function.as_ptr());
        // println!("handle: {handle}", handle = handle_ptr as u32);

        if handle_ptr == NULL {
            return false;
        }

        let NtQueryInformationProcess = mem::transmute::<FARPROC, fn(
            ProcessHandle: HANDLE, 
            ProcessInformationClass: PROCESSINFOCLASS,
            ProcessInformation: PVOID, 
            ProcessInformationLength: ULONG,
            ReturnLength: PULONG
        ) -> NTSTATUS>(handle_ptr);


        let process = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid);

        let mut bytes_read = 0u32;

        let mut process_info = PROCESS_BASIC_INFORMATION::default();
        let process_info_size = mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32;

        let result = NtQueryInformationProcess(
            process, ProcessBasicInformation, RawMutPtr(&mut process_info as PPBI as *mut void),
            process_info_size, &mut bytes_read as PULONG
        );
        // println!("DEBUG: bytes read: {bytes_read}");
        // println!("DEBUG: {process_info_size} {addr}", addr = process_info.PebBaseAddress.0 as usize);

        let mut peb_data = PEB::default();
        let peb_size = mem::size_of::<PEB>() as u64;
        // println!("peb size: {peb_size}");

        let result = ReadProcessMemory(
            process, process_info.PebBaseAddress.0 as LPCVOID, &mut peb_data as *mut PEB as LPVOID,
            peb_size, &mut bytes_read as *mut u32 as *mut SIZE_T
        );
        // println!("DEBUG: bytes read: {bytes_read}");

        let mut rtl_data = RTL_USER_PROCESS_PARAMETERS::default();
        let rtl_size = mem::size_of::<RTL_USER_PROCESS_PARAMETERS>() as u64;
        let result = ReadProcessMemory(
            process, peb_data.ProcessParameters.0 as LPCVOID, &mut rtl_data as *mut RTL_USER_PROCESS_PARAMETERS as LPVOID,
            rtl_size, &mut bytes_read as *mut u32 as *mut SIZE_T
        );
        // println!("DEBUG: bytes read: {bytes_read}");
        // println!("len: {}", rtl_data.CommandLine.Length);

        let mut buffer = vec![0u16; (rtl_data.CommandLine.Length / 2) as usize];
        let result = ReadProcessMemory(
            process, rtl_data.CommandLine.Buffer.0 as LPCVOID, buffer.as_mut_ptr() as LPVOID,
            rtl_data.CommandLine.Length as u64, &mut bytes_read as *mut u32 as *mut SIZE_T
        );
        // println!("DEBUG: bytes read: {bytes_read}");
        let command_line = String::from_utf16_lossy(&buffer[..]).to_lowercase();
        // println!("{command_line}");

        if command_line.contains(pattern.to_lowercase().as_str()) {
            CloseHandle(process);
            return true;
        }

        CloseHandle(process);

    }
    return false;
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
