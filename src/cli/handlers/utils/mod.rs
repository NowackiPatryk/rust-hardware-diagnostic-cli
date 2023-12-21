use core::time;

pub fn wait(miliseconds: u64) {
    std::thread::sleep(time::Duration::from_millis(miliseconds))
}

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}
