# Kernel and user program compilation
CC = gcc
LD = ld
CFLAGS += -mcmodel=small -Wall -Wno-builtin-declaration-mismatch -O2 -fno-pie -mno-red-zone -nostdinc -fno-stack-protector -fno-zero-initialized-in-bss -fno-builtin -c
# LDFLAGS = -nostdlib -melf_x86_64 -z max-page-size=0x1000 -n
LDFLAGS = --gc-sections -n

ARCH ?= x86_64
KERNEL := build/$(ARCH)-kernel
RUST_KERNEL = target/$(ARCH)-unknown-mros/debug/libmros.a
ISO := build/$(ARCH)-mros.iso
ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

linker_script := linkers/$(ARCH).lds
grub_cfg := grub.cfg
assembly_source_files := $(wildcard asm/$(ARCH)/*.S)
assembly_object_files := $(patsubst asm/$(ARCH)/%.S, \
    build/%.o, $(assembly_source_files))

.PHONY: all clean run xen qemu

all: $(ISO)

clean:
	cargo clean
	@rm -rf build

qemu: $(ISO)
	@qemu-system-$(ARCH) -m 1024 -drive format=raw,file=$(ISO)

xen: $(BOOT)
	sudo xl create ./kernel.cfg

$(ISO): $(KERNEL) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(KERNEL) build/isofiles/boot/kernel
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles

KERNEL_OBJS = build/kernel_entry.o
KERNEL_OBJS += $(RUST_KERNEL) build/kernel_asm.o

$(KERNEL): $(KERNEL_OBJS)
	@mkdir -p build
	$(LD) $(LDFLAGS) -T $(linker_script) $^ -o $@

# Compile rust kernel
$(RUST_KERNEL):
	@mkdir -p build/$(ARCH)
	RUST_TARGET_PATH="$(ROOT_DIR)" cargo build --target "targets/$(ARCH)-unknown-mros.json"

build/%.o: %.c
	@mkdir -p build
	$(CC) $(CFLAGS) -I ./include -c -o $@ $<

build/%.o: asm/$(ARCH)/%.S
	@mkdir -p build
	$(CC) $(CFLAGS) -I ./include -c -o $@ $<
