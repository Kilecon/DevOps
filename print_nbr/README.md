# HOW TO

```bash
nasm -f elf32 -O3 -o print-nbr.o print-nbr.asm
ld -m elf_i386 -s -N --strip-all --gc-sections -z norelro -o print-nbr print-nbr.o
```