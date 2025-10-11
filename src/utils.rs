use std::{
    env::var,
    io::{self, Write},
    process::{Child, Command, Stdio},
};

fn is_wayland() -> bool {
    var("WAYLAND_DISPLAY").is_ok()
}

pub fn copy_to_clipboard(content: &str) -> io::Result<Child> {
    if is_wayland() {
        return Command::new("wl-copy").arg(content).spawn();
    }

    let mut child = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = &mut child.stdin {
        stdin.write_all(content.as_bytes())?;
    }

    Ok(child)
}

pub fn insert(content: &str) -> io::Result<Child> {
    if is_wayland() {
        return Command::new("wtype").arg(content).spawn();
    }

    Command::new("xdotool")
        .args(["sleep", "0.2", "type", content])
        .spawn()
}
