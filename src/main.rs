use std::{thread, time::Duration};

unsafe extern "C" {
    fn _kbhit() -> i32;
    fn _getch() -> i32;
}

fn main() {
    loop {
        unsafe {
            // キーが押されているか
            if _kbhit() != 0 {
                let ch = _getch() as u8 as char;

                println!("pressed: {}", ch);

                if ch == 'q' {
                    break;
                }
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}
