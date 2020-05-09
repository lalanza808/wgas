use std::process::Command;


pub fn wg_cmd(args: &'static [&'static str]) -> String {
    let output = Command::new("./bin/wg_cmd")
        .args(args)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout)
        .unwrap();

    output_str
}

pub fn sh_cmd(cmd: &'static str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout)
        .unwrap();

    output_str
}
