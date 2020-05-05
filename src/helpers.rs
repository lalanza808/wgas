use std::process::Command;


pub fn wg_cmd(arg: String) -> String {
    let output = Command::new("./bin/wg_cmd")
        .arg(arg)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout)
        .unwrap();

    output_str
}

pub fn sh_cmd(cmd: String) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout)
        .unwrap();

    output_str
}
