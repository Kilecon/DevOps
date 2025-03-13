# WIK-DPS-TP01-B

nasm -f elf64 -O2 toto.asm -o toto.o
ld -s -o toto toto.o --strip-all
upx toto


nasm -f elf32 -O3 -o print-nbr.o print-nbr.asm
ld -m elf_i386 -s -N --strip-all --gc-sections -z norelro -o print-nbr print-nbr.o