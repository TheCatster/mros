# Kernel and user program compilation
CC = gcc
LD = ld
CFLAGS += -mcmodel=small -Wall -Wno-builtin-declaration-mismatch -O2 -fno-pie -mno-red-zone -nostdinc -fno-stack-protector -fno-zero-initialized-in-bss -fno-builtin -c
# LDFLAGS = -nostdlib -melf_x86_64 -z max-page-size=0x1000 -n
LDFLAGS = --gc-sections -n

ARCH ?= x86_64
KERNEL := build/$(ARCH)-kernel.bin
RUST_KERNEL = build/$(ARCH)/libmros.a
ISO := build/$(ARCH)-mros.iso
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

linker_script := linkers/$(ARCH).lds
grub_cfg := grub.cfg
assembly_source_files := $(wildcard asm/$(ARCH)/*.S)
assembly_object_files := $(patsubst asm/$(ARCH)/%.S, \
    build/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: iso

clean:
	cargo clean
	@rm -rf build

qemu: $(ISO)
	@qemu-system-$(ARCH) -m 1024 -drive format=raw,file=$(ISO)

xen: $(BOOT)
	sudo xl create ./kernel.cfg

iso: $(ISO)

$(ISO): $(KERNEL) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(KERNEL) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles 2> /dev/null

$(KERNEL): $(assembly_object_files) $(linker_script) $(RUST_KERNEL)
	@mkdir -p build
	$(LD) $(LDFLAGS) -T $(linker_script) -o $(KERNEL) $(assembly_object_files)

# Compile rust kernel
$(RUST_KERNEL):
	@mkdir -p build/$(ARCH)
	RUST_TARGET_PATH="$(ROOT_DIR)targets" cargo build --target $(ARCH)-unknown-mros

build/%.o: %.c
	$(CC) $(CFLAGS) -I ./include -c -o $@ $<

build/%.o: asm/$(ARCH)/%.S
	$(CC) $(CFLAGS) -I ./include -c -o $@ $<
