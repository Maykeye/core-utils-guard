use std::process::{exit, Command};

fn executable_base_name() -> String {
    // We assert argv[0] is meaningful
    let exe = std::env::current_exe().unwrap();
    let exe = exe.to_str();
    let exe = exe.unwrap();
    exe.to_owned()
}

fn main() {
    let full_path = executable_base_name();
    let args: Vec<String> = std::env::args().collect();
    let blacklist = vec![
        "..",
        "/bin",
        "/boot",
        "/dev",
        "/efi",
        "/etc",
        "/home",
        "/lib",
        "/lib64",
        "/mnt",
        "/opt",
        "/proc",
        "/root",
        "/run",
        "/sbin",
        "/srv",
        "/sys",
        "/tmp",
        "/usr",
        "/var",
        "_GUARD_DEBUG_ENTRY_", // For deubgging -- I aint testing by running rm /*
    ];
    let unsafe_path = format!("{full_path}.unsafe");
    let mut cmd = Command::new(unsafe_path);

    for arg in &args[1..] {
        let is_blacklisted = blacklist
            .iter()
            .position(|&forbidden| arg == forbidden || arg == &format!("{forbidden}/"))
            .is_some();
        if is_blacklisted {
            panic!("{full_path}: Argument contain invalid arg {arg}");
        }
        cmd.arg(arg);
    }
    let mut child = cmd.spawn().unwrap();
    let exit_code = child.wait().unwrap();
    let exit_code = exit_code.code().unwrap();
    exit(exit_code)
}
