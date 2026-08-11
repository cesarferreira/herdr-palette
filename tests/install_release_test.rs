use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn keeps_an_existing_development_binary() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the Unix epoch")
        .as_nanos();
    let temporary_directory = std::env::temp_dir().join(format!("herdr-palette-{unique}"));
    let binary_directory = temporary_directory.join("bin");
    let binary = binary_directory.join("herdr-palette");

    fs::create_dir_all(&binary_directory).expect("temporary bin directory should be created");
    fs::write(&binary, "development binary").expect("development binary should be created");

    let output = Command::new("sh")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/scripts/install-release.sh"
        ))
        .current_dir(&temporary_directory)
        .output()
        .expect("install script should run");

    fs::remove_dir_all(&temporary_directory).expect("temporary directory should be removed");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("script output should be UTF-8"),
        "bin/herdr-palette already exists; using development binary.\n"
    );
}
