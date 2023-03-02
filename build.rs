use std::process::Command;

fn main() {
    Command::new("sh")
        .args(&["-c", "cd resources && glib-compile-resources resources.xml"])
        .output()
        .unwrap();

    let output = Command::new("git")
        .arg("describe")
        .arg("--always")
        .arg("--tags")
        .arg("--dirty")
        .output()
        .unwrap();
    let version = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=VERSION={}", version);

    println!("cargo:rerun-if-changed=resources");
}
