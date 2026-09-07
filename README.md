# Carlinho OS

A minimalist 32-bit (x86) educational operating system designed as a tribute to vintage BLIS/COBOL mainframe operating systems. **Carlinho OS** boots directly into VGA text mode, operating entirely on bare-metal without standard libraries or Linux dependencies, presenting a retro business/data processing interface.

## Creator

* **David Carlos Miranda Delgado**
* *Student of the Integrated High School Technical Course at IFSULDEMINAS - Campus Poços de Caldas*

---

## Architecture & How It Works

Carlinho OS follows a classic 32-bit x86 architecture boot sequence using the Multiboot standard:

1. **Bootstrap (`boot.asm`)**: Written in NASM Assembly, it provides the Multiboot header, sets up a 16 KiB stack, and jumps directly to the C kernel entry point (`kernel_main`).
2. **Core Kernel (`kernel.c`)**: A freestanding C program that bypasses any standard C library. It maps directly to the VGA text buffer at memory address `0xB8000`, clears the screen with a classic mainframe blue background, and renders a business-oriented text menu and status bar.
3. **Linker Script (`linker.ld`)**: Organizes the binary sections (`.text`, `.rodata`, `.data`, `.bss`) starting at the standard `1MB` virtual memory address required by Multiboot.
4. **GRUB Bootloader (`grub.cfg`)**: Configures the GRUB bootloader to automatically load the kernel binary into memory.
5. **Automation & CI/CD (`Makefile` & GitHub Actions)**: The build pipeline compiles the assembly and C source files with cross-compilation flags (`-m32`, `-ffreestanding`), links them using `ld`, verifies Multiboot compliance with `grub-file`, and packages everything into a bootable ISO image via `grub-mkrescue`.

---

## System Requirements

* **CPU**: 1 vCPU (Single-Core; runs entirely on the bootstrap processor).
* **RAM**: 16 MB to 32 MB is more than enough.
* **Storage**: The generated bootable ISO (`carlinho-os.iso`) is approximately **2 MB to 5 MB**.

---

## Project Structure

```text
.
├── .github/
│   └── workflows/
│       └── build.yml      # GitHub Actions CI workflow for automated ISO compilation
├── boot.asm               # Multiboot assembly entry point (32-bit)
├── kernel.c               # Core C kernel with VGA text-mode business interface
├── linker.ld              # GNU Linker script (loads at 1MB)
├── grub.cfg               # GRUB configuration file
└── Makefile               # Build automation script
```

---

## Building Locally

### Prerequisites
To build the ISO locally on an Ubuntu/Debian-based system, install the required toolchain:
```bash
sudo apt-get update
sudo apt-get install -y gcc-multilib nasm xorriso grub-pc-bin mtools
```

### Compilation
Run the `make` command to compile and generate `carlinho-os.iso`:
```bash
make
```

To clean up build artifacts:
```bash
make clean
```

---

## Running in an Emulator (QEMU)

You can easily test the generated ISO image using QEMU:
```bash
qemu-system-i386 -cdrom carlinho-os.iso -m 32
```

---

## CI/CD Pipeline

The project includes a fully automated GitHub Actions workflow (`.github/workflows/build.yml`) that triggers on every push or pull request. It spins up an Ubuntu environment, installs all cross-compilation and packaging dependencies, builds the ISO, and uploads `carlinho-os.iso` as a downloadable artifact.
