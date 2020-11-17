use std::collections::HashMap;
use winapi::um::*;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::raw::c_char;

unsafe fn unsafe_main() {
    let mut map: HashMap<String, Duration> = HashMap::new();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();

    while running.load(Ordering::SeqCst) {
        {
            let window = winuser::GetForegroundWindow();
            let mut pid: u32 = 0;
            winuser::GetWindowThreadProcessId(window, &mut pid as *mut u32);
            let h = processthreadsapi::OpenProcess(
                winnt::PROCESS_QUERY_LIMITED_INFORMATION,
                false as i32,
                pid,
            );

            let mut buf: Vec<c_char> = vec![0; 255];
            let mut len: u32 = 255;
            winbase::QueryFullProcessImageNameA(h, 0, buf.as_mut_ptr(), &mut len as *mut u32);

            let full_path = String::from_utf8(buf.into_iter().map(|c| c as u8).collect()).unwrap().trim_end_matches('\u{0}').to_string();
            let exe_name = full_path.split('/').last().unwrap().to_string();

            if map.contains_key(&exe_name) {
                *map.get_mut(&exe_name).unwrap() += Duration::from_secs(1);
            } else {
                map.insert(exe_name, Duration::from_secs(1));
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    }

    dbg!(map);

    let mut _s = String::new();
    let _ = std::io::stdin().read_line(&mut _s);
}

fn main() {
    unsafe { unsafe_main() }
}
