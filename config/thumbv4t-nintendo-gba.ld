/* THIS LINKER SCRIPT FILE IS RELEASED TO THE PUBLIC DOMAIN (SPDX: CC0-1.0) */

ENTRY(_start)

MEMORY {
  ewram (rwx) : ORIGIN = 0x2000000, LENGTH = 256K
  iwram (rwx) : ORIGIN = 0x3000000, LENGTH = 32K
  rom (rx)    : ORIGIN = 0x8000000, LENGTH = 32M
}

SECTIONS {
  .text : {
    /* be sure that the ROM header is the very first */
    *(.text.boot);
    *(.text .text.*);
    . = ALIGN(4);
  } >rom = 0x00

  .rodata : {
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } >rom = 0x00

  . = ALIGN(4);
  .data : {
    __iwram_start = ABSOLUTE(.);
    
    *(.data .data.*);
    *(.iwram .iwram.*);
    . = ALIGN(4);
    
    __iwram_end = ABSOLUTE(.);
  } >iwram AT>rom = 0x00

  . = ALIGN(4);
  .ewram : {
    __ewram_start = ABSOLUTE(.);
    
    *(.ewram .ewram.*);
    . = ALIGN(4);
    
    __ewram_end = ABSOLUTE(.);
  } >ewram AT>rom = 0x00

  . = ALIGN(4);
  .bss : {
    __bss_start = ABSOLUTE(.);

    *(.bss .bss.*);
    . = ALIGN(4);

    __bss_end = ABSOLUTE(.);
  } >iwram

  /* rust-lld demands we keep the `section header string table` */
  .shstrtab        0 : { *(.shstrtab) }

  /* debugging sections */
  /* Stabs */
  .stab            0 : { *(.stab) }
  .stabstr         0 : { *(.stabstr) }
  .stab.excl       0 : { *(.stab.excl) }
  .stab.exclstr    0 : { *(.stab.exclstr) }
  .stab.index      0 : { *(.stab.index) }
  .stab.indexstr   0 : { *(.stab.indexstr) }
  .comment         0 : { *(.comment) }
  /* DWARF 1 */
  .debug           0 : { *(.debug) }
  .line            0 : { *(.line) }
  /* GNU DWARF 1 extensions */
  .debug_srcinfo   0 : { *(.debug_srcinfo) }
  .debug_sfnames   0 : { *(.debug_sfnames) }
  /* DWARF 1.1 and DWARF 2 */
  .debug_aranges   0 : { *(.debug_aranges) }
  .debug_pubnames  0 : { *(.debug_pubnames) }
  /* DWARF 2 */
  .debug_info      0 : { *(.debug_info) }
  .debug_abbrev    0 : { *(.debug_abbrev) }
  .debug_line      0 : { *(.debug_line) }
  .debug_frame     0 : { *(.debug_frame) }
  .debug_str       0 : { *(.debug_str) }
  .debug_loc       0 : { *(.debug_loc) }
  .debug_macinfo   0 : { *(.debug_macinfo) }
  /* SGI/MIPS DWARF 2 extensions */
  .debug_weaknames 0 : { *(.debug_weaknames) }
  .debug_funcnames 0 : { *(.debug_funcnames) }
  .debug_typenames 0 : { *(.debug_typenames) }
  .debug_varnames  0 : { *(.debug_varnames) }

  /* discard anything not already mentioned */
  /DISCARD/ : { *(*) }
}
