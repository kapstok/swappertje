# Swappertje

## Notes

[GTK 4 tutorial](https://gtk-rs.org/gtk4-rs/git/book/hello_world.html)

The swap file implementation in the kernel expects to be able to write to
the file directly, without the assistance of the file system. 
This is a problem on files with holes or on copy-on-write files on file 
systems like Btrfs. Commands like cp(1) or truncate(1) create files with 
holes. These files will be rejected by swapon. 
Preallocated files created by fallocate(1) may be interpreted as files 
with holes too depending of the filesystem. Preallocated swap files are 
supported on XFS since Linux 4.18. 
The most portable solution to create a swap file is to use dd(1) and  /dev/zero.