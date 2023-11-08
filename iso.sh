#!/bin/sh
mkdir -p ./iso_image/boot/grub
cp ./grub.cfg ./iso_image/boot/grub/grub.cfg
cp ./kernel ./iso_image/boot/kernel
grub-mkrescue -o boot.iso iso_image
rm -r ./iso_image
