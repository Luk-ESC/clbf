use std::{
    ffi::{OsStr, OsString},
    path::Path,
};

fn cc_file(name: &str) -> OsString {
    let out = std::process::Command::new("cc")
        .arg(format!("-print-file-name={}", name))
        .output()
        .unwrap();
    OsString::from(String::from_utf8(out.stdout).unwrap().trim())
}

pub fn link_file(object: &Path, out: &Path) {
    let crt1 = cc_file("crt1.o");
    let crti = cc_file("crti.o");
    let crtn = cc_file("crtn.o");

    let mut linker = std::process::Command::new("ld")
        .args([
            OsStr::new("-o"),
            out.as_os_str(),
            object.as_os_str(),
            OsStr::new("-lc"),
            &crti,
            &crt1,
            &crtn,
        ])
        .spawn()
        .unwrap();

    let status = linker.wait().unwrap();

    if !status.success() {
        eprintln!("Linking failed: Exit code {status}");
    }
}
