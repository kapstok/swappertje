# Swappertje

## Notes

[GTK 4 tutorial](https://gtk-rs.org/gtk4-rs/git/book/hello_world.html)

### Add swap

clib::fcntl

```bash
chmod 600 /swapfile
mkswap /swapfile
swapon /swapfile

# check it
swapon --show

# permanent
echo "/swapfile none swap sw 0 0\n" >> /etc/fstab
```