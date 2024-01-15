`rustBoot` support for [stm32h743zi](https://www.st.com/en/evaluation-tools/nucleo-h743zi.html) nucleo board, we have one example. It has a custom led configuration. If you're using a different version of the board, you'll probably need to edit `firmware and hal implementations` to accomodate for differences. Just make sure you **dont change** the name of files/folders or the folder structure, as `cargo xtask` looks for these file/folder names.

- In order to test this example you'll need a couple of things - `wolfcrypt, probe-run or probe-rs-cli, python3, stm32cube-Programmer installed`
- If you've managed to install all of them, you can use below commands to build and sign all 3 packages (i.e. bootloader + bootfw + updatefw) onto the board.
    - Command for build rustBoot
    `cargo stm32h743 build rustBoot-only`

    - Command for build packages
    `cargo stm32h743 build pkgs-for`

    - Command for sign packages
    `cargo stm32h743 sign pkgs-for 1234 1235`

- In order to flash all 3 binarise (i.e. bootloader + bootfw + updatefw) I've used `probe-rs-cli`.
    - To flash bootloader use this command
      - `probe-rs-cli.exe download --format elf --base-address 0x08000000  --chip STM32H743ZITx .\boards\target\thumbv7em-none-eabihf\release\stm32h743` 
    - To flash firmwares use these command
      - `probe-rs-cli.exe download --format Bin --base-address 0x08020000  --chip STM32H743ZITx .\boards\sign_images\signed_images\stm32h743_bootfw_v1234_signed.bin`, and
      - `probe-rs-cli.exe download --format Bin --base-address 0x08120000  --chip STM32H743ZITx .\boards\sign_images\signed_images\stm32h743_updtfw_v1235_signed.bin`

- In order to confirm that its working, I've configured the `bootfw to blink green` for a few seconds, trigger an update and then reset. Upon reset, the bootloader verifies the update and swaps the contents of boot and update partitions. If everything checks out, it boots into the update, `blinks a red led` and finally sets the confirmation flag to indicate that the update was successful.