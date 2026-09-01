#!/bin/bash
set -e

ISO_DIR="iso_root"
rm -rf $ISO_DIR
mkdir -p $ISO_DIR/boot/grub
mkdir -p $ISO_DIR/usr/bin

echo "[Carlinhos OS] Copiando arquivos do sistema e binários..."

cp build/shell/carlinhos-shell $ISO_DIR/usr/bin/
cp iso/boot/grub/grub.cfg $ISO_DIR/boot/grub/

if [ -f "/boot/vmlinuz" ]; then
    cp /boot/vmlinuz $ISO_DIR/boot/kernel.bin
else
    touch $ISO_DIR/boot/kernel.bin 
fi

echo "[Carlinhos OS] Gerando imagem ISO final"
grub-mkrescue -o carlinho-os.iso $ISO_DIR

echo "[Carlinhos OS] Imagem 'carlinho-os.iso' gerada com sucesso."
