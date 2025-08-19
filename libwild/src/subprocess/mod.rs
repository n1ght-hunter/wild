#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::run_in_subprocess;

#[cfg(target_os = "windows")]
pub use windows::run_in_subprocess;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub use dummy::run_in_subprocess;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
mod dummy {
    /// # Safety
    /// See function of the same name in `subprocess.rs`
    pub unsafe fn run_in_subprocess(args: crate::Args) -> ! {
        let exit_code = match crate::run(args) {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("{}", error.to_string());
                -1
            }
        };
        std::process::exit(exit_code);
    }
}
