#!/usr/bin/env bash
cargo build --target x86_64-unknown-uefi --release
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/snake.efi esp/EFI/BOOT/BOOTX64.EFI
nix build nixpkgs#OVMF.fd
rm qemu.log
qemu-system-x86_64 \
    -machine q35 \
    -m 512M \
    -drive if=pflash,format=raw,readonly=on,file=./result-fd/FV/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=./result-fd/FV/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp \
    -serial stdio \
    -D qemu.log \
    -d guest_errors
