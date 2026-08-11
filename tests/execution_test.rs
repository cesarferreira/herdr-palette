#![cfg(unix)]

use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use herdr_palette::{catalog::default_items, command::execute, Invocation};

static ENVIRONMENT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn executes_supported_actions_with_the_catalogued_argv() {
    let _environment = ENVIRONMENT_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let directory = temporary_directory("success");
    let output = directory.join("argv.txt");
    let script = write_script(
        &directory,
        "fake-herdr",
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$HERDR_TEST_ARGV_PATH\"\n",
    );
    env::set_var("HERDR_BIN_PATH", &script);
    env::set_var("HERDR_TEST_ARGV_PATH", &output);

    execute(new_tab(), herdr_bin_path().as_ref()).unwrap();

    assert_eq!(
        fs::read_to_string(&output).unwrap(),
        "tab\ncreate\n--focus\n"
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn reports_exit_status_and_stderr_when_herdr_fails() {
    let _environment = ENVIRONMENT_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let directory = temporary_directory("failure");
    let script = write_script(
        &directory,
        "fake-herdr",
        "#!/bin/sh\nprintf '%s\\n' 'cannot create tab' >&2\nexit 17\n",
    );
    env::set_var("HERDR_BIN_PATH", &script);

    let error = execute(new_tab(), herdr_bin_path().as_ref()).unwrap_err();

    assert!(error.message.contains("17"));
    assert!(error.message.contains("cannot create tab"));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn documentation_only_actions_do_not_spawn_herdr() {
    let _environment = ENVIRONMENT_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let directory = temporary_directory("documentation");
    let output = directory.join("argv.txt");
    let script = write_script(
        &directory,
        "fake-herdr",
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$HERDR_TEST_ARGV_PATH\"\n",
    );
    env::set_var("HERDR_BIN_PATH", &script);
    env::set_var("HERDR_TEST_ARGV_PATH", &output);
    let item = default_items()
        .into_iter()
        .find(|item| item.invocation == Invocation::DocumentationOnly)
        .unwrap();

    assert!(execute(&item, herdr_bin_path().as_ref()).is_err());

    assert!(!output.exists());
    fs::remove_dir_all(directory).unwrap();
}

fn new_tab() -> &'static herdr_palette::PaletteItem {
    static NEW_TAB: OnceLock<herdr_palette::PaletteItem> = OnceLock::new();
    NEW_TAB.get_or_init(|| {
        default_items()
            .into_iter()
            .find(|item| item.id == "new_tab")
            .unwrap()
    })
}

fn herdr_bin_path() -> PathBuf {
    env::var_os("HERDR_BIN_PATH").unwrap().into()
}

fn temporary_directory(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory = env::temp_dir().join(format!(
        "herdr-palette-{name}-{}-{timestamp}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    directory
}

fn write_script(directory: &Path, name: &str, contents: &str) -> PathBuf {
    let path = directory.join(name);
    fs::write(&path, contents).unwrap();
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).unwrap();
    path
}
