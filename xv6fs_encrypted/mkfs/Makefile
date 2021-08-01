CFLAGS += -O2 -g -fPIC

all:	mkfs

mkfs:	mkfs.o
	gcc -o mkfs $^

userspace: mkfs_user.o
	gcc -o mkfs $^

clean:
	rm -rf *.o mkfs fs.img
