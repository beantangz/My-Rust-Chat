use std::{env, thread, time::Duration};

fn send_char(pid: i32, c: u8) {
    for i in (0..8).rev() {
        let bit = (c >> i) & 1;

        unsafe {
            if bit == 1 {
                libc::kill(pid, libc::SIGUSR2);
            } else {
                libc::kill(pid, libc::SIGUSR1);
            }
        }

        thread::sleep(Duration::from_micros(500));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <PID> <message>", args[0]);
        return;
    }

    let pid: i32 = args[1].parse().expect("Invalid PID");
    let message = &args[2];

    for c in message.bytes() {
        send_char(pid, c);
    }

    send_char(pid, 0);
}