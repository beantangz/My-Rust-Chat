use signal_hook::iterator::Signals;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

static CURRENT_CHAR: AtomicU8 = AtomicU8::new(0);
static BIT_COUNT: AtomicUsize = AtomicUsize::new(0);

fn handle_signal(sig: i32) {
    let mut c = CURRENT_CHAR.load(Ordering::Relaxed);

    c <<= 1;

    if sig == libc::SIGUSR2 {
        c |= 1;
    }

    CURRENT_CHAR.store(c, Ordering::Relaxed);

    let bits = BIT_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    if bits == 8 {
        BIT_COUNT.store(0, Ordering::Relaxed);
    }
}

fn main() {
    println!("PID: {}", std::process::id());

    let mut signals = Signals::new(&[libc::SIGUSR1, libc::SIGUSR2]).unwrap();

    for sig in signals.forever() {
        handle_signal(sig);

        if BIT_COUNT.load(Ordering::Relaxed) == 0 {
            let c = CURRENT_CHAR.load(Ordering::Relaxed);

            if c == 0 {
                println!();
            } else {
                print!("{}", c as char);
            }
        }
    }
}