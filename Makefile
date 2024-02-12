BOOT = boot.iso
KERNEL = kernel
RUST_KERNEL = ./target/x86_64-mros/debug/libmros.a
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

all: $(BOOT)

qemu: $(BOOT)
	qemu-system-x86_64 -m 1024 -drive format=raw,file=$(BOOT)

xen: $(BOOT)
	sudo xl create ./kernel.cfg

$(BOOT): $(KERNEL)
	@mkdir -p ./iso_image/boot/grub
	@cp ./grub.cfg ./iso_image/boot/grub/grub.cfg
	@cp ./kernel ./iso_image/boot/kernel
	@grub-mkrescue -o ./boot.iso iso_image
	@rm -r ./iso_image

# Compile rust kernel
$(RUST_KERNEL):
	RUST_TARGET_PATH="$(ROOT_DIR)" xargo build --target x86_64-mros

# Kernel and user program compilation
CC = gcc
LD = ld
CFLAGS += -mcmodel=small -Wall -Wno-builtin-declaration-mismatch -O2 -fno-pie -mno-red-zone -nostdinc -fno-stack-protector -fno-zero-initialized-in-bss -fno-builtin -c
# LDFLAGS = -nostdlib -melf_x86_64 -z max-page-size=0x1000 -n
LDFLAGS = --gc-sections -n

KERNEL_OBJS = kernel_entry.o
KERNEL_OBJS += $(RUST_KERNEL) kernel_asm.o

$(KERNEL): $(KERNEL_OBJS)
	$(LD) $(LDFLAGS) -T ./kernel.lds $^ -o $@

%.o: %.c
	$(CC) $(CFLAGS) -I ./include -c -o $@ $<

%.o: %.S
	$(CC) $(CFLAGS) -I ./include -c -o $@ $<

clean:
	cargo clean
	@rm -rf $(KERNEL) $(KERNEL_OBJS) $(BOOT) iso_image
