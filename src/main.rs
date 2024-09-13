use windows::Win32::Foundation::{HANDLE};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, PROCESS_ALL_ACCESS,
};

unsafe fn open_process() -> Result<HANDLE,windows::core::Error> {
    let process_id = GetCurrentProcessId(); // Get current process ID
    let process_id2:u32 = 7684;
    println!("The Process ID is {}", process_id);
    let handle = OpenProcess(PROCESS_ALL_ACCESS, false, process_id2)?; // Open process with full access
    Ok(handle) //returns handle
}
fn main() {
    unsafe {
        match open_process() {
            Ok(handle) => println!("Successfully opened process with handle: {:?}", handle),
            Err(e) => println!("Failed to open process: {:?}", e),
        }
    }
}