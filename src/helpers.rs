use std::process::Command;


pub fn genkey() -> String {
    let output = Command::new("./bin/wg_cmd")
        .arg("genkey")
        .output()
        .expect("failed to execute process");
    let privkey = String::from_utf8(output.stdout)
        .unwrap();

    privkey
}
