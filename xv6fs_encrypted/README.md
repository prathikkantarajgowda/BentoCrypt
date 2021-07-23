This is a Rust file system. The structure is based on the xv6 file system,
but the file system includes a number of optimizations not included in xv6.

The file system is compiled as a Linux kernel module and depends on the
bentofs kernel module.

We use Linux kernel version 4.15 and Rust nightly version 1.43.0.

## Disk setup
**To create the disk image (kernel version):**
In mkfs:
```
make
./mkfs fs.img
```

**To create the disk image (userspace version):**
In mkfs:
```
make userspace
./mkfs fs.img
```

## Kernel version
**To compile bentofs:**
First, compile bentofs in a neighboring directory.
```
make
```

**To clean:**
```
cargo clean
```

**To insert bentofs:**
Next, insert the bentofs kernel module (bentofs directory).
```
sudo insmod bentofs.ko
```

**To compile xv6fs:**
Next, compile xv6fs (project root directory).
```
cd xv6fs/rust
make
```

**To insert xv6fs:**
Next, insert xv6fs kernel module (xv6fs directory).
```
sudo insmod rust/kernel/xv6fs.ko
```

**To mount file system:**
Finally, mount xv6fsll (xv6fs directory).
```
sudo mkdir -p /mnt/xv6fsll
sudo mount -t bentoblk -o loop -o rootmode=40000,user_id=0,group_id=0,blksize=4096,name=xv6fs_ll mkfs/fs.img /mnt/xv6fsll
```

**To unmount file system:**
```
sudo umount /mnt/xv6fsll
```

**To remove module:**
```
sudo rmmod xv6fs
```

## User version
**To compile bentofs:**
First, compile bentofs (bentofs directory).
```
make
```

**To clean:**
```
cargo clean
```

**To insert bentofs:**
Next, insert the bentofs kernel module (bentofs directory).
```
sudo insmod bentofs.ko
```

**To compile xv6fs:**
Next, compile xv6fs (project root directory).
```
cd xv6fs/rust
make userspace
```

**To clean:**
```
make clean
```

**To mount/insert:**
To mount and insert (xv6fs directory).
```
sudo mkdir -p /mnt/xv6fsll
sudo userspace/target/release/user_xv6fs mkfs/fs.img /mnt/xv6fsll blkdev
```

**To unmount file system:**
```
sudo fusermount -u /mnt/xv6fsll
```
