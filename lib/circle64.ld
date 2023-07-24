/*
 * circle64.ld
 */

ENTRY(_start)

SECTIONS
{
	. = 0x80000;

	.init : {
		*(.init)
	}

	.text : {
		*(.text*)

		_etext = .;
	}

	.rodata : {
		*(.rodata*)
	}

	.init_array : {
		__init_start = .;

		KEEP(*(.init_array*))

		__init_end = .;
	}

	.ARM.exidx : {
		__exidx_start = .;

		*(.ARM.exidx*)

		__exidx_end = .;
	}

	.eh_frame : {
		*(.eh_frame*)
	}

	.data : {
		*(.data*)
	}

	.bss : {
		__bss_start = .;

		*(.bss*)
		*(COMMON)

		_end = .;
	}
}
