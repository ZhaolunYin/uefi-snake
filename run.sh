#!/usr/bin/env bash
cargo build --target x86_64-unknown-uefi
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/snake.efi esp/EFI/BOOT/BOOTX64.EFI
nix build nixpkgs#OVMF.fd
qemu-system-x86_64 \
    -machine q35 \
    -m 512M \
    -drive if=pflash,format=raw,readonly=on,file=./result-fd/FV/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=./result-fd/FV/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp \
    -serial stdio
