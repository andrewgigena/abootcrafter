# Abootcrafter

**Abootcrafter** is a Rust-based tool designed to manipulate android boot images like a real blacksmith. It allows you to display information about boot images, extract their components, update existing boot images, or create new ones based on provided configurations and files.

## Features

- **Info**: Display detailed information about a boot image, including kernel, ramdisk, and second stage sizes, along with configuration details.
- **Extract**: Extract the kernel, ramdisk, and second stage from a boot image into separate files.
- **Update**: Update an existing boot image by replacing components (kernel, ramdisk, second stage) and modifying configuration settings.
- **Create**: Create a new boot image from provided kernel and ramdisk files, with optional second stage and configuration settings.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/your-username/abootcrafter.git
    cd abootcrafter
    ```

2. **Build the Project**:
    ```bash
    cargo build --release
    ```

   The binary will be located in `target/release/abootcrafter`.

3. **Run the Program**:
    You can run the program directly from the command line:
    ```bash
    ./target/release/abootcrafter --help
    ```

## Usage

### Display Information about a Boot Image

```bash
abootcrafter info -i path/to/boot.img
```

### Extract Components from a Boot Image

```bash
abootcrafter extract -i path/to/boot.img -o path/to/output/directory
```

- **`-i` or `--input-boot-file`**: Path to the input boot image file.
- **`-o` or `--output-dir`**: (Optional) Directory where the extracted components will be saved. Defaults to `out`.

### Update an Existing Boot Image

```bash
abootcrafter update -i path/to/boot.img -C path/to/config.toml -k path/to/new/kernel.img -r path/to/new/ramdisk.img -s path/to/new/second.img --cmdline "new cmdline"
```

- **`-i` or `--input-boot-file`**: Path to the input boot image file.
- **`-C` or `--config-file`**: (Optional) Path to a TOML configuration file.
- **`-k` or `--kernel-file`**: (Optional) Path to a new kernel image file.
- **`-r` or `--ramdisk-file`**: (Optional) Path to a new ramdisk image file.
- **`-s` or `--second-file`**: (Optional) Path to a new second stage image file.
- **`--cmdline`**: (Optional) New command line parameters.

### Create a New Boot Image

```bash
abootcrafter create -o path/to/new/boot.img -k path/to/kernel.img -r path/to/ramdisk.img -C path/to/config.toml --cmdline "new cmdline"
```

- **`-o` or `--output-boot-file`**: Path to the output boot image file.
- **`-k` or `--kernel-file`**: Path to the kernel image file.
- **`-r` or `--ramdisk-file`**: Path to the ramdisk image file.
- **`-s` or `--second-file`**: (Optional) Path to the second stage image file.
- **`-C` or `--config-file`**: (Optional) Path to a TOML configuration file.
- **`--cmdline`**: (Optional) Command line parameters.

## Configuration File

The configuration file is a TOML file that can contain various settings for the boot image. Here is an example of a configuration file:

```toml
boot_size = 10485760
page_size = 2048
kernel_addr = 0x10008000
ramdisk_addr = 0x11000000
second_addr = 0x12000000
tags_addr = 0x10000100
cmdline = "console=ttyMSM0,115200n8 androidboot.hardware=qcom"
```

- **`boot_size`**: (Optional) Total size of the boot image.
- **`page_size`**: Page size used in the boot image.
- **`kernel_addr`**: Address where the kernel will be loaded.
- **`ramdisk_addr`**: Address where the ramdisk will be loaded.
- **`second_addr`**: Address where the second stage will be loaded.
- **`tags_addr`**: Address where the tags will be loaded.
- **`cmdline`**: Command line parameters for the boot image.

## Examples

1. **Display Information**:
    ```bash
    abootcrafter info -i boot.img
    ```

2. **Extract Components**:
    ```bash
    abootcrafter extract -i boot.img -o extracted
    ```

3. **Update Boot Image**:
    ```bash
    abootcrafter update -i boot.img -C config.toml -k new_kernel.img -r new_ramdisk.img --cmdline "console=ttyMSM0,115200n8"
    ```

4. **Create New Boot Image**:
    ```bash
    abootcrafter create -o new_boot.img -k kernel.img -r ramdisk.img -C config.toml --cmdline "console=ttyMSM0,115200n8"
    ```

## Next features
- [~] Add support for all [boot image headers](https://source.android.com/docs/core/architecture/bootloader/boot-image-header#implementing-versioning)
- [ ] Add ramdisk subcommands (info, recompress (in-place), unpack, repack, addfile?, removefile?, etc)
- [ ] Add device tree subcommands (info, remove, add, replace)
- [ ] Add signature subcommands (info, remove, replace, generate)
- [ ] Add kernel subcomands (info, extract-config)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to open issues or pull requests to improve this tool.

## Authors

- [Andrew Gigena](https://github.com/andrewgigena)


