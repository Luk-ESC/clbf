use std::{ffi::OsStr, path::Path};

pub fn link_file(object: &Path, out: &Path) {
    let mut linker = std::process::Command::new("ld")
        .args([
            OsStr::new("-o"),
            out.as_os_str(),
            object.as_os_str(),
            OsStr::new("-lc"),
            OsStr::new("/usr/lib/crti.o"),
            OsStr::new("/usr/lib/crt1.o"),
            OsStr::new("/usr/lib/crtn.o"),
        ])
        .spawn()
        .unwrap();

    let status = linker.wait().unwrap();

    if !status.success() {
        eprintln!("Linking failed: Exit code {status}");
    }
}
