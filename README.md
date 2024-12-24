# abootcrafter

**abootcrafter** is a Rust-based tool designed to manipulate android boot images like a real blacksmith. It allows you to display information about boot images, extract their components, update existing boot images, or create new ones based on provided configurations and files.

[![Build status](https://github.com/andrewgigena/abootcrafter/workflows/ci/badge.svg)](https://github.com/andrewgigena/abootcrafter/actions)
[![Crates.io](https://img.shields.io/crates/v/abootcrafter)](https://lib.rs/crates/abootcrafter)

## Features

- **Info**: Display detailed information about a boot image, including kernel, ramdisk, and second stage sizes, along with configuration details.
- **Extract**: Extract the kernel, ramdisk, and second stage from a boot image into separate files.
- **Update**: Update an existing boot image by replacing components (kernel, ramdisk, second stage) and modifying configuration settings.
- **Create**: Create a new boot image from provided kernel and ramdisk files, with optional second stage and configuration settings.

## Installation

**[Archives of precompiled binaries for abootcrafter are available for Windows,
macOS, Linux and Android.](https://github.com/andrewgigena/abootcrafter/releases)** Linux, Android and
Windows binaries are static executables.

abootcrafter can also be installed with `cargo`.

```
$ cargo install abootcrafter
```

Alternatively, one can use [`cargo
binstall`](https://github.com/cargo-bins/cargo-binstall) to install a abootcrafter
binary directly from GitHub:

```
$ cargo binstall abootcrafter
```


## Usage

### Display Information about a Boot Image

```bash
abootcrafter info bootimg --input-boot-file <INPUT_BOOT_FILE>
```

- **`--input-boot-file` or `-i`**: Path to the input boot image file.

### Extract Components from a Boot Image

```bash
abootcrafter extract bootimg --input-boot-file <INPUT_BOOT_FILE> --output-dir <OUTPUT_DIR>
```

- **`--input-boot-file` or `-i`**: Path to the input boot image file.
- **`--output-dir` or `-o`**: (Optional) Directory where the extracted components will be saved. If not specified, the components will be extracted to a default directory.

### Update an Existing Boot Image

```bash
abootcrafter update bootimg --input-boot-file <INPUT_BOOT_FILE> [OPTIONS]
```

- **`--input-boot-file` or `-i`**: Path to the input boot image file.
- **`--kernel-file` or `-k`**: (Optional) Path to a new kernel image file.
- **`--ramdisk-file` or `-r`**: (Optional) Path to a new ramdisk image file.
- **`--second-file` or `-s`**: (Optional) Path to a new second stage image file.
- **`--recovery-dtbo-file`**: (Optional) Path to a new recovery DTBO image file (v1 to v2 only).
- **`--dtb-file`**: (Optional) Path to a new DTB image file (v2 only).
- **`--cmdline`**: (Optional) New command line parameters.
- **`--extra-cmdline`**: (Optional) Extra command line parameters.

### Create a New Boot Image

#### Version 0 (< Android 9)

```bash
abootcrafter create bootimg-v0 --output-boot-file <OUTPUT_BOOT_FILE> --kernel-file <KERNEL_FILE> --ramdisk-file <RAMDISK_FILE> [OPTIONS]
```

- **`--output-boot-file` or `-o`**: Output boot image file.
- **`--kernel-file` or `-k`**: Kernel file to use for creating the boot image.
- **`--ramdisk-file` or `-r`**: Ramdisk file to use for creating the boot image.
- **`--second-file` or `-s`**: (Optional) Second file to use for creating the boot image.
- **`--page-size`**: (Optional) Page size to use for creating the boot image [default: 2048] [possible values: 2048, 4096, 8192, 16384].
- **`--kernel-addr`**: (Optional) Physical load address of the kernel [default: 0x00008000].
- **`--ramdisk-addr`**: (Optional) Physical load address of the ramdisk [default: 0x01000000].
- **`--second-addr`**: (Optional) Physical load address of the second [default: 0x00000000].
- **`--tags-addr`**: (Optional) Physical load address of the tags [default: 0x00000100].
- **`--os-version`**: (Optional) Android OS Version of the boot image [default: ].
- **`--name`**: (Optional) Product name of the boot image [default: ].
- **`--cmdline`**: (Optional) Kernel command line of the boot image [default: ].
- **`--id`**: (Optional) timestamp / checksum / sha1 / etc [default: ].
- **`--extra-cmdline`**: (Optional) Extra kernel command line of the boot image [default: ].

#### Version 1 (== Android 9)

```bash
abootcrafter create bootimg-v1 --output-boot-file <OUTPUT_BOOT_FILE> --kernel-file <KERNEL_FILE> --ramdisk-file <RAMDISK_FILE> [OPTIONS]
```

- **`--output-boot-file` or `-o`**: Output boot image file.
- **`--kernel-file` or `-k`**: Kernel file to use for creating the boot image.
- **`--ramdisk-file` or `-r`**: Ramdisk file to use for creating the boot image.
- **`--second-file` or `-s`**: (Optional) Second file to use for creating the boot image.
- **`--recovery-dtbo-file` or `-D`**: (Optional) Recovery DTBO file to use for creating the boot image.
- **`--page-size`**: (Optional) Page size to use for creating the boot image [default: 2048] [possible values: 2048, 4096, 8192, 16384].
- **`--kernel-addr`**: (Optional) Physical load address of the kernel [default: 0x00008000].
- **`--ramdisk-addr`**: (Optional) Physical load address of the ramdisk [default: 0x01000000].
- **`--second-addr`**: (Optional) Physical load address of the second [default: 0x00000000].
- **`--tags-addr`**: (Optional) Physical load address of the tags [default: 0x00000100].
- **`--os-version`**: (Optional) Android OS Version of the boot image [default: ].
- **`--name`**: (Optional) Product name of the boot image [default: ].
- **`--cmdline`**: (Optional) Kernel command line of the boot image [default: ].
- **`--id`**: (Optional) timestamp / checksum / sha1 / etc [default: ].
- **`--extra-cmdline`**: (Optional) Extra kernel command line of the boot image [default: ].
- **`--recovery-dtbo-offset`**: (Optional) Offset of the recovery DTBO partition [default: 0x0000000000000000].

#### Version 2 (== Android 10)

```bash
abootcrafter create bootimg-v2 --output-boot-file <OUTPUT_BOOT_FILE> --kernel-file <KERNEL_FILE> --ramdisk-file <RAMDISK_FILE> [OPTIONS]
```

- **`--output-boot-file` or `-o`**: Output boot image file.
- **`--kernel-file` or `-k`**: Kernel file to use for creating the boot image.
- **`--ramdisk-file` or `-r`**: Ramdisk file to use for creating the boot image.
- **`--second-file` or `-s`**: (Optional) Second file to use for creating the boot image.
- **`--recovery-dtbo-file` or `-D`**: (Optional) Recovery DTBO file to use for creating the boot image.
- **`--dtb-file` or `-d`**: (Optional) Device tree file to use for creating the boot image.
- **`--page-size`**: (Optional) Page size to use for creating the boot image [default: 2048] [possible values: 2048, 4096, 8192, 16384].
- **`--kernel-addr`**: (Optional) Physical load address of the kernel [default: 0x00008000].
- **`--ramdisk-addr`**: (Optional) Physical load address of the ramdisk [default: 0x01000000].
- **`--second-addr`**: (Optional) Physical load address of the second [default: 0x00000000].
- **`--tags-addr`**: (Optional) Physical load address of the tags [default: 0x00000100].
- **`--os-version`**: (Optional) Android OS Version of the boot image [default: ].
- **`--name`**: (Optional) Product name of the boot image [default: ].
- **`--cmdline`**: (Optional) Kernel command line of the boot image [default: ].
- **`--id`**: (Optional) timestamp / checksum / sha1 / etc [default: ].
- **`--extra-cmdline`**: (Optional) Extra kernel command line of the boot image [default: ].
- **`--recovery-dtbo-offset`**: (Optional) Offset of the recovery DTBO partition [default: 0x0000000000000000].
- **`--dtb-addr`**: (Optional) Physical load address of the device tree [default: 0x0000000000000000].

#### Version 3 (>= Android 11)

```bash
abootcrafter create bootimg-v3 --output-boot-file <OUTPUT_BOOT_FILE> --kernel-file <KERNEL_FILE> --ramdisk-file <RAMDISK_FILE> [OPTIONS]
```

- **`--output-boot-file` or `-o`**: Output boot image file.
- **`--kernel-file` or `-k`**: Kernel file to use for creating the boot image.
- **`--ramdisk-file` or `-r`**: Ramdisk file to use for creating the boot image.
- **`--os-version`**: (Optional) Android OS Version of the boot image [default: ].
- **`--cmdline`**: (Optional) Kernel command line of the boot image [default: ].

#### Version 4 (>= Android 12)

```bash
abootcrafter create bootimg-v4 --output-boot-file <OUTPUT_BOOT_FILE> --kernel-file <KERNEL_FILE> --ramdisk-file <RAMDISK_FILE> [OPTIONS]
```

- **`--output-boot-file` or `-o`**: Output boot image file.
- **`--kernel-file` or `-k`**: Kernel file to use for creating the boot image.
- **`--ramdisk-file` or `-r`**: Ramdisk file to use for creating the boot image.
- **`--os-version`**: (Optional) Android OS Version of the boot image [default: ].
- **`--cmdline`**: (Optional) Kernel command line of the boot image [default: ].

## Roadmap
- [x] Add support for all [boot image headers](https://source.android.com/docs/core/architecture/bootloader/boot-image-header#implementing-versioning)
- [ ] Add ramdisk subcommands (info, recompress (in-place), unpack, repack, addfile?, removefile?, etc)
- [ ] Add device tree subcommands (info, remove, add, replace)
- [ ] Add signature subcommands (info, remove, replace, generate)
- [ ] Add kernel subcomands (info, extract-config)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contributing

Contributions are welcome! Please feel free to open issues or pull requests to improve this tool.

## Authors

- [Andrew Gigena](https://github.com/andrewgigena)


