
# 📡 Rust Talk

---

## 🧠 Description

**MY Rust Chat** is a client-server communication program written in Rust.  
It demonstrates **inter-process communication (IPC)** using UNIX signals by encoding messages **bit by bit**.

The client sends a string to the server using only:

- `SIGUSR1` → represents bit **0**
- `SIGUSR2` → represents bit **1**

The server reconstructs each character from incoming signals and displays the message in real time.

---

## ⚙️ How It Works

1. The server starts and displays its PID.
2. The client takes:
   - the server PID
   - a string to send
3. Each character is converted into **8 bits**.
4. Each bit is sent using a UNIX signal.
5. The server reconstructs characters from incoming signals and prints them.

---

## 🚀 Instructions

### 🔧 Compilation

Make sure you have Rust installed then build the project:

```bash

cargo build

```

▶️ Run the server

```bash
cargo run --bin server
```

Example output:

PID: 12345

📤 Run the client

```bash

cargo run --bin client <PID from server> "your message"

```

Example:

```bash
cargo run --bin client 12345 "Hello World"
```

🔧 Technical Choices

Rust for memory safety and concurrency guarantees

libc crate to access low-level UNIX functions (kill, signals)

signal-hook crate to safely handle UNIX signals

Atomic types (AtomicU8, AtomicUsize) to share state between signal handling and main logic

## Author

mleineku – 42 School Student

## License

This project is open-source and reusable for personal or educational purposes.**
