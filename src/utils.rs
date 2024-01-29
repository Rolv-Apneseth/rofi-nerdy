use std::io;
use std::process::{Child, Stdio};
use std::{env::var, process::Command};

fn is_wayland() -> bool {
    var("WAYLAND_DISPLAY").is_ok()
}

pub fn copy_to_clipboard(content: &str) -> io::Result<Child> {
    if is_wayland() {
        return Command::new("wl-copy").arg(content).spawn();
    }

    let echo_cmd = Command::new("echo")
        .arg(content)
        .stdout(Stdio::piped())
        .spawn()?;

    Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(match echo_cmd.stdout {
            Some(o) => o,
            // No stdout from the above echo command should be impossible
            None => unreachable!(),
        })
        .spawn()
}

pub fn insert(content: &str) -> io::Result<Child> {
    if is_wayland() {
        return Command::new("wtype").arg(content).spawn();
    }

    Command::new("xdotool")
        .args(["sleep", "0.2", "type", content])
        .spawn()
}
