use std::process::Command;


pub fn wg_cmd(args: Vec<String>) -> (String, i32) {
    let output = Command::new("./bin/wg_cmd")
        .args(args)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout)
        .unwrap();
    let resp_code = output.status.code()
        .unwrap();

    (output_str, resp_code)
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
