use std::ptr;

use phnt::{
    ext::NtCurrentTeb,
    ffi::{
        HANDLE, NtClose, NtCreateUserProcess, NtTerminateProcess, NtWaitForSingleObject,
        PROCESS_CREATE_FLAGS_INHERIT_HANDLES, PS_CREATE_INFO, ULONG_PTR,
    },
};
use tracing_subscriber::{
    Layer as _, fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use windows_sys::Win32::{
    Foundation::{FALSE, STATUS_PROCESS_CLONED},
    System::{
        Console::{ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole},
        Threading::{PROCESS_ALL_ACCESS, THREAD_ALL_ACCESS},
    },
};
use wlibwild::run;

fn my_background_function() {
    for i in 1..=5 {
        println!("Background thread says: {}", i);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    println!("Background thread finished.");
}

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_file(true)
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or("wlibwild=info,warn".into()),
                ),
        )
        .init();
    // run();

    unsafe { run_in_subprocess() }
}

pub unsafe fn run_in_subprocess() -> ! {
    let exit_code = match subprocess_result() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{}", error.to_string());
            -1
        }
    };
    std::process::exit(exit_code);
}

#[allow(non_upper_case_globals)]
pub const NtCurrentProcess: HANDLE = -1isize as *mut std::ffi::c_void;

fn subprocess_result() -> Result<i32, Box<dyn std::error::Error>> {
    let mut hprocess: HANDLE = std::ptr::null_mut();

    let mut hthread: HANDLE = std::ptr::null_mut();

    let mut createInfo: PS_CREATE_INFO = unsafe { std::mem::zeroed() };
    createInfo.Size = std::mem::size_of::<PS_CREATE_INFO>() as _;

    match unsafe {
        NtCreateUserProcess(
            &mut hprocess,
            &mut hthread,
            PROCESS_ALL_ACCESS,
            THREAD_ALL_ACCESS,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            PROCESS_CREATE_FLAGS_INHERIT_HANDLES,
            0,
            std::ptr::null_mut(),
            &mut createInfo,
            std::ptr::null_mut(),
        )
    } {
        STATUS_PROCESS_CLONED => {
            // executing inside the clone

            // re attach to the parent's console to be able to write to it
            unsafe {
                FreeConsole();
                AttachConsole(ATTACH_PARENT_PROCESS);
            };

            let (pid, tid) = unsafe {
                (
                    (*NtCurrentTeb()).ClientId.UniqueProcess as ULONG_PTR,
                    (*NtCurrentTeb()).ClientId.UniqueThread as ULONG_PTR,
                )
            };

            println!("Demo for Process Cloning via NtCreateUserProcess by Hunt & Hackett.");
            println!(
                "Hello from the parent! My PID is {}, TID is {}\r\n",
                pid, tid
            );
            unsafe { NtTerminateProcess(NtCurrentProcess, STATUS_PROCESS_CLONED) };
        }
        status if status == 0 => {
            let status = unsafe { NtWaitForSingleObject(hprocess, FALSE as _, ptr::null_mut()) };

            unsafe {
                NtClose(hprocess);
                NtClose(hthread);
            };

            if status != 0 {
                // If the wait failed, we print the error and return
                eprintln!("Failed to wait for the clone: 0x{:x}", status);
                return Ok(status);
            }

            println!("Process cloned successfully!");

            return Ok(0);
        }
        code => {
            eprintln!("NtCreateUserProcess failed with error code: {}", code);
            return Err(Box::new(std::io::Error::from_raw_os_error(code)));
        }
    }
    Ok(0)
}
