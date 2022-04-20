#!/bin/bash
~/qemu/app/bin/qemu-system-aarch64 \
-serial mon:stdio \
-M arm-generic-fdt \
-device loader,file=./$1,cpu-num=4 \
-device loader,addr=0XFF5E023C,data=0x80008fde,data-len=4 \
-device loader,addr=0xff9a0000,data=0x80000218,data-len=4 \
-hw-dtb ./qemu/dtb/sample.dtb \
-m 2G \
-display none \
-drive file=./sd/qemu_sd.img,if=sd,index=1,format=raw \
