# embeddedsw-rs-samples

## Requirements
Make sure you have installed the dependencies:
- [Xilinx QEMU](https://github.com/Xilinx/qemu)
- Vitis v2021.1(64-bit) or Vitis v2021.2 (64-bit)

You prepare the following files:
- Linker script  
    Please put a linker script named `lscript.ld` on lscripts directory.
- XSA file  
    Please put a XSA file  on xsa directory.
- DTB file  
    Please put a DTB file named `sample.dtb` on dtb directory.
- SD card image  
    Please put a sd card image named `sd_card.img` on sd directory. This sd card image must be FAT. A `test.dat` file must be included in the first logical volume, and contain 10 characters.

## Examples
### Hello World
```
XSA_PATH=xsa-file-path cargo run -vv -bin hello_world
```

### Allocator
```
XSA_PATH=xsa-file-path cargo run -vv -bin xalloc
```

### XScuGic
```
XSA_PATH=xsa-file-path cargo run -vv -bin xscugic
```

### SD Card
```
XSA_PATH=xsa-file-path cargo run -vv -bin sd_card
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.