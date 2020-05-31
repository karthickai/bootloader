MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* We're only recognizing part of the flash here */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  /* We're only the tightly coupled RAM here to avoid nasty effect */
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
