use std::process::Command;

fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.xml",
        "resources.gresource",
    );

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
