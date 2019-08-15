// extern crate libc;

type DWORD = u32;
type PVOID = *mut usize;
#[allow(non_camel_case_types)]
type ULONG_PTR = *mut u32;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: PVOID,
    pub NumberParameters: DWORD,
    pub ExceptionInformation: [ULONG_PTR; 15],
}

// #[cfg(all(target_os = "win32", target_arch = "x86"))]
#[link(name = "kernel32")]
#[allow(non_snake_case)]
extern "stdcall" {
    fn RaiseFailFastException(pExceptionRecord: *const u8, pContextRecord: *const u8, dwFlags: u32) -> u32;
}

// RaiseFailFastException(
//     __in_opt PEXCEPTION_RECORD pExceptionRecord,
//     __in PCONTEXT pContextRecord,
//     __in DWORD dwFlags
//     );

fn main () {
    std::panic::set_hook(Box::new(|_panic_info| {
        unsafe {
            RaiseFailFastException(0 as *const u8, 0 as *const u8, 0);
        }
    }));

    panic!("Fail fast");
}