#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

//
// WinApi bindings
//
#[derive(Copy, Clone)]
pub enum void {}
pub type BYTE = u8;
pub type USHORT = u16;
pub type WCHAR = u16;
pub type BOOL = i32;
pub type UINT = u32;
pub type DWORD = u32;
pub type ULONG = u32;
pub type SIZE_T = u64;
pub type ULONG_PTR = u64;

pub type LPWSTR = *mut u16;
pub type PULONG = *mut u32;
pub type LPDWORD = *mut u32;

pub type HANDLE = *mut void;
pub type HINSTANCE = *mut void;
pub type HMODULE = *mut void;
pub type LPVOID = *mut void;
pub type PVOID = RawMutPtr<void>;

pub type LPCSTR = *const i8;
pub type PWSTR = RawMutPtr<WCHAR>;

pub type KPRIORITY = i32;
pub type NTSTATUS = i32;
pub type KAFFINITY = u64;

pub type LPCVOID = *const void;
pub type FARPROC = *const void;

pub type PROCESSINFOCLASS = i32;
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0;
pub const ProcessDebugPort: PROCESSINFOCLASS = 7;
pub const ProcessWow64Information: PROCESSINFOCLASS = 26;
pub const ProcessImageFileName: PROCESSINFOCLASS = 27;
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29;
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = 75;

#[derive(Copy, Clone)]
pub struct RawMutPtr<T>(pub *mut T);
impl<T> Default for RawMutPtr<T> {
    fn default() -> RawMutPtr<T> {
        return Self(NULL as *mut T)
    }
}

#[repr(C)]
#[derive(Default)]
pub struct UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PWSTR,
}

#[repr(C)]
#[derive(Default)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: ReservedData<BYTE, 16>,
    pub Reserved2: ReservedData<PVOID, 10>,
    pub ImagePathName: UNICODE_STRING,
    pub CommandLine: UNICODE_STRING,
}

pub type PPEB_LDR_DATA = RawMutPtr<void>; // Placeholder pointer binding, won't be used
pub type PRTL_USER_PROCESS_PARAMETERS = RawMutPtr<RTL_USER_PROCESS_PARAMETERS>;
pub type PPS_POST_PROCESS_INIT_ROUTINE = RawMutPtr<void>; // Placeholder pointer binding, won't be used

// Wrapper to implement Default for any array
pub struct ReservedData<T: Default + Copy, const N: usize>(pub [T; N]);
impl<T: Default + Copy, const N: usize> Default for ReservedData<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

#[repr(C)]
#[derive(Default)]
pub struct PEB {
    pub Reserved1: ReservedData<BYTE, 2>,
    pub BeingDebugged: BYTE,
    pub Reserved2: ReservedData<BYTE, 1>,
    pub Reserved3: ReservedData<PVOID, 2>,
    pub Ldr: PPEB_LDR_DATA,
    pub ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: ReservedData<PVOID, 3>,
    pub AtlThunkSListPtr: PVOID,
    pub Reserved5: PVOID,
    pub Reserved6: ULONG,
    pub Reserved7: PVOID,
    pub Reserved8: ULONG,
    pub AtlThunkSListPtr32: ULONG,
    pub Reserved9: ReservedData<PVOID, 45>,
    pub Reserved10: ReservedData<BYTE, 96>,
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: ReservedData<BYTE, 128>,
    pub Reserved12: ReservedData<PVOID, 1>,
    pub SessionId: ULONG
}

pub type PPEB = RawMutPtr<PEB>;

#[repr(C)]
#[derive(Default)]
pub struct PROCESS_BASIC_INFORMATION {
    pub ExitStatus: NTSTATUS,
    pub PebBaseAddress: PPEB,
    pub AffinityMask: KAFFINITY,
    pub BasePriority: KPRIORITY,
    pub UniqueProcessId: ULONG_PTR,
    pub InheritedFromUniqueProcessId: ULONG_PTR,
}
pub type PPBI = *mut PROCESS_BASIC_INFORMATION;

pub const READ_CONTROL: DWORD = 0x00020000;
pub const SYNCHRONIZE: DWORD = 0x00100000;
pub const PROCESS_TERMINATE: DWORD = 0x0001;
pub const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
pub const PROCESS_VM_READ: DWORD = 0x0010;
pub const NULL: *mut void = 0 as *mut void;
pub const ATTACH_PARENT_PROCESS: DWORD = 0xFFFFFFFF;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

#[link(name = "psapi")]
extern "system" {
    pub fn EnumProcesses(lpidProcess: *mut DWORD, cb: DWORD, lpcbNeeded: LPDWORD) -> BOOL;
    pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD) -> HANDLE; 
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
     
    // Requires PROCESS_TERMINATE process access right
    pub fn TerminateProcess(hProcess: HANDLE, uExitCode: UINT) -> BOOL;
     
    pub fn EnumProcessModules(
        hProcess: HANDLE, lphModule: *mut HMODULE, cb: DWORD, lpcbNeeded: LPDWORD
    ) -> BOOL;
     
    // Requires PROCESS_QUERY_INFORMATION and PROCESS_VM_READ process access right
    pub fn GetModuleBaseNameW(
        hProcess: HANDLE, hModule: HMODULE, lpBaseName: LPWSTR, nSize: DWORD
    ) -> DWORD;
     
    // Requires PROCESS_VM_READ process access right
    pub fn ReadProcessMemory(
        hProcess: HANDLE, lpBaseAddress: LPCVOID, lpBuffer: LPVOID, nSize: SIZE_T, 
        lpNumberOfBytesRead: *mut SIZE_T
    ) -> BOOL;
     
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn LoadLibraryA(lpLibFileName: LPCSTR) -> HMODULE;
    pub fn AttachConsole(dwProcessId: DWORD) -> BOOL;
    pub fn FreeConsole() -> BOOL;
}
