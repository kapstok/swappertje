use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::os::unix::io::AsRawFd;

// E.g. for a size of 150 GB: 150 * 1024 * 1024 * 1024;
fn allocate(filename: &str, filesize: &i64) -> io::Result<()> {
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

    Ok(())
}