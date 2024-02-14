use std::{
    process::{Command, Stdio},
    str,
};

pub fn get_is_mic_in_use() -> bool {
    let ioreg_child = Command::new("ioreg")
        .arg("-c")
        .arg("AppleUSBAudioEngine")
        .arg("-r")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep_child = Command::new("grep")
        .arg("-o")
        .arg("IOAudioEngineState\" = 1")
        .stdin(ioreg_child.stdout.expect("Failed to open ioreg stdout"))
        .output()
        .unwrap();

    let result = str::from_utf8(&grep_child.stdout).unwrap();

    result.len() > 0
}
