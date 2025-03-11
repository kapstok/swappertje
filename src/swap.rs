use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::os::unix::io::AsRawFd;
use std::process::Command;
use std::fs::remove_file;

#[derive(Clone)]
pub struct Swapfile<'a> {
    filename: &'a str,
    filesize: &'a i64
}

impl Swapfile<'_> {
    pub fn create<'a>(filename: &'a str, filesize: &'a i64) -> io::Result<Swapfile<'a>> {
        allocate(filename, filesize)
    }

    pub fn get_filename(&self) -> String {
        String::from(self.filename)
    }

    pub fn get_filesize(&self) -> i64 {
        *self.filesize
    }
}

pub fn destroy(swapfile: &str) -> io::Result<()> {
    let status = Command::new("swapoff")
        .arg(swapfile)
        .status()
        .expect("Failed to dismount swapfile.");

    if !status.success() {
        return Err(std::io::Error::new(ErrorKind::Other, "Failed to dismount swapfile."));
    }

    deallocate(swapfile);
    Ok(())
}

// E.g. for a size of 150 GB: 150 * 1024 * 1024 * 1024;
fn allocate<'a>(filename: &'a str, filesize: &'a i64) -> io::Result<Swapfile<'a>> {
    if *filesize <= 0 {
        return Err(std::io::Error::new(ErrorKind::InvalidInput, "filesize should be higher than 0."));
    }

    let fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;

    let raw_fd = fd.as_raw_fd();
    let size: libc::off_t = filesize.clone();
    
    let ret = unsafe {
        libc::fallocate(raw_fd, 0, 0, size)
    };

    if ret == -1 {
        return Err(io::Error::last_os_error());
    }

    prepare(Swapfile {
        filename,
        filesize
    })
}

fn deallocate(swapfile: &str) {
    let _result = remove_file(swapfile);
}

fn prepare(swapfile: Swapfile) -> io::Result<Swapfile> {
    let status = Command::new("chmod")
        .args(["600", &swapfile.filename])
        .status()
        .expect("Failed to change rights.");

    if !status.success() {
        deallocate(&swapfile.filename);
        return Err(std::io::Error::new(ErrorKind::Other, "Failed to change rights."));
    }

    let status = Command::new("mkswap")
        .arg(&swapfile.filename)
        .status()
        .expect("Failed to make swap partition type.");

    if !status.success() {
        deallocate(&swapfile.filename);
        return Err(std::io::Error::new(ErrorKind::Other, "Failed to make swap partition type."));
    }

    let status = Command::new("swapon")
        .arg(&swapfile.filename)
        .status()
        .expect("Failed to use swapfile.");

    if !status.success() {
        deallocate(&swapfile.filename);
        return Err(std::io::Error::new(ErrorKind::Other, "Failed to use swapfile."));
    }

    Ok(swapfile)
}