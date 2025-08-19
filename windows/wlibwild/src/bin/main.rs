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

    let (pid, tid) = unsafe {
        (
            (*NtCurrentTeb()).ClientId.UniqueProcess as ULONG_PTR,
            (*NtCurrentTeb()).ClientId.UniqueThread as ULONG_PTR,
        )
    };

    println!("Demo for Process Cloning ");
    println!(
        "Hello from the parent! My PID is {}, TID is {}\r\n",
        pid, tid
    );

    match unsafe { fork(&mut hprocess, &mut hthread) } {
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
            std::thread::sleep(std::time::Duration::from_millis(5000));


            println!(
                "Hello from the clone! My PID is {}, TID is {}\r\n",
                pid, tid
            );
            unsafe { NtTerminateProcess(NtCurrentProcess, STATUS_PROCESS_CLONED) };
        }
        status if status == 0 => {
      
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

unsafe fn fork(hprocess: &mut HANDLE, hthread: &mut HANDLE) -> i32 {
    let mut create_info: PS_CREATE_INFO = unsafe { std::mem::zeroed() };
    create_info.Size = std::mem::size_of::<PS_CREATE_INFO>() as _;

    unsafe {
        NtCreateUserProcess(
            hprocess,
            hthread,
            PROCESS_ALL_ACCESS,
            THREAD_ALL_ACCESS,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            PROCESS_CREATE_FLAGS_INHERIT_HANDLES,
            0,
            std::ptr::null_mut(),
            &mut create_info,
            std::ptr::null_mut(),
        )
    }
}

// fn subprocess_result(args: Args) -> Result<i32> {
//     let mut fds: [c_int; 2] = [0; 2];
//     // create the pipe used to communicate between the parent and child processes - exit on failure
//     make_pipe(&mut fds).context("make_pipe")?;

//     // Safety: The function we're in is private to this module and is only called from
//     // run_in_subprocess, which imposed the requirement that threads have not yet been started on
//     // its caller.
//     match unsafe { fork() } {
//         0 => {
//             // Fork success in child - Run linker in this process.

//             crate::setup_tracing(&args)?;
//             let args = args.activate_thread_pool()?;
//             let linker = crate::Linker::new();
//             let _outputs = linker.run(&args)?;
//             inform_parent_done(&fds);
//             Ok(0)
//         }
//         -1 => {
//             // Fork failure in the parent - Fallback to running linker in this process

//             crate::run(args)?;
//             Ok(0)
//         }
//         pid => {
//             // Fork success in the parent - wait for the child to "signal" us it's done
//             let exit_status = wait_for_child_done(&fds, pid);
//             Ok(exit_status)
//         }
//     }
// }

// Inform the parent process that work of linker is done and that it succeeded.
// fn inform_parent_done(fds: &[c_int]) {
    // unsafe {
    //     libc::close(fds[0]);
    //     let stream = libc::fdopen(fds[1], "w".as_ptr() as *const c_char);
    //     let bytes: [u8; 1] = [b'X'];
    //     libc::fwrite(bytes.as_ptr() as *const c_void, 1, 1, stream);
    //     libc::fclose(stream);
    //     libc::close(libc::STDOUT_FILENO);
    //     libc::close(libc::STDERR_FILENO);
    // }
// }

// Wait for the child process to signal it is done, by sending a byte on the pipe. In the case the
// child crashes, or exits via some path that doesn't send a byte, then the pipe will be closed and
//  we'll then wait for the subprocess to exit, returning its exit code.
// fn wait_for_child_done(fds: &[c_int], child_pid: pid_t) -> i32 {
    //     unsafe {
    //         // close our sending end of the pipe
    //         libc::close(fds[1]);
    //         // open the other end of the pipe for reading
    //         let stream = libc::fdopen(fds[0], "r".as_ptr() as *const c_char);

    //         // Wait for child to send a byte via the pipe or for the pipe to be closed.
    //         let mut response: [u8; 1] = [0u8; 1];
    //         match libc::fread(response.as_mut_ptr() as *mut c_void, 1, 1, stream) {
    //             1 => {
    //                 // Child sent a byte, which indicates that it succeeded and is now shutting down in
    //                 // the background.
    //                 0
    //             }
    //             _ => {
    //                 // Child closed pipe without sending a byte - get the process exit_status
    //                 let mut status: libc::c_int = -1i32;
    //                 libc::waitpid(child_pid, &mut status, 0);
    //                 libc::WEXITSTATUS(status)
    //             }
    //         }
    //     }
// }

// Create a pipe for communication between parent and child processes.
// If successful it will return Ok and `fds` will have file descriptors for reading and writing
// If errors it will return an error message with the errno set, if it can be read or -1 if not
// fn make_pipe(fds: &mut [c_int; 2]) -> Result {
    // match unsafe { libc::pipe(fds.as_mut_ptr()) } {
    //     0 => Ok(()),
    //     _ => bail!(
    //         "Error creating pipe. Errno = {:?}",
    //         std::io::Error::last_os_error().raw_os_error().unwrap_or(-1)
    //     ),
    // }
// }
