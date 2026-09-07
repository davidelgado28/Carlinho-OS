CC = gcc
AS = nasm
LD = ld

CFLAGS = -m32 -ffreestanding -O2 -Wall -Wextra -c
ASFLAGS = -f elf32
LDFLAGS = -m elf_i386 -T linker.ld

OBJS = boot.o kernel.o
BIN = carlinho.bin
ISO = carlinho-os.iso

all: $(ISO)

boot.o: boot.asm
	$(AS) $(ASFLAGS) boot.asm -o boot.o

kernel.o: kernel.c
	$(CC) $(CFLAGS) kernel.c -o kernel.o

$(BIN): $(OBJS) linker.ld
	$(LD) $(LDFLAGS) -o $(BIN) $(OBJS)

$(ISO): $(BIN) grub.cfg
	@grub-file --is-x86-multiboot $(BIN) || (echo "Erro: O arquivo binario nao e Multiboot valido!" && exit 1)
	@mkdir -p isodir/boot/grub
	@cp $(BIN) isodir/boot/
	@cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) isodir
	@rm -rf isodir

clean:
	rm -f $(OBJS) $(BIN) $(ISO)
	rm -rf isodir

.PHONY: all clean
