OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)
SECTIONS
{
	. = 1M;

	.text : {
		*(.text .gnu.linkonce.t.*)
	}

	.data : {
		*(.data* .gnu.linkonce.d.* .rodata*)
	}

	.bss : {
		*(.bss)
		*(.common)
	}

	/DISCARD/ : {
		*(.eh_frame .eh_frame_hdr .debug* .note* .comment* .gnu.version* .stab .stabstr .ctors .dtors .fini* .init* .line .preinit_array)
	}
}
