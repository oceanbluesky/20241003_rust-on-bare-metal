# Install Bare Metal Project Tools

This document provides instructions for installing the tools needed to work with the Microbit in a bare metal environment on Linux, macOS, and Windows. There are three parts to the installation:

1. Install the Rust toolchain.
1. Install and configure operating system specific tools.
1. Verify the Microbit connection.

## Install Project Tools

1. Install the `llvm-tools` to convert the compiled Rust code into a binary file that we can load onto the Microbit.

    ```sh
    rustup component add llvm-tools
    ```

    The `component` command manages optional parts of the Rust toolchain. The `add` subcommand installs a component. The `llvm-tools` component provides the `llvm-objcopy` tool, which we will use to convert the compiled Rust code into a binary file that we can load onto the Microbit.

1. Install the `thumbv7em-none-eabihf` target.

    ```sh
    rustup target add thumbv7em-none-eabihf
    ```

    The thumbv7em-none-eabihf target in Rust is used for compiling code for ARM Cortex-M processors with the following characteristics:

    - **thumbv7em**: Specifies the ARM architecture version (ARMv7-M) and the instruction set (Thumb-2).
    - **none**: Indicates that there is no operating system (bare-metal).
    - **eabi**: Stands for the ARM Embedded Application Binary Interface, which is a standard for how functions are called and how data is represented in memory.
    - **hf**: Indicates hardware floating-point support.

    This target is typically used for embedded systems programming where you are writing software to run directly on the hardware without an operating system.

1. Install the `cargo-binutils` crate to provide the cargo integrated command wrapper for `llvm-tools`.

    ```sh
    cargo install --locked cargo-binutils --vers 0.3.3
    ```

    The cargo `install` command is used to install Rust binaries from crates. It downloads the specified crate from crates.io, compiles it, and installs the resulting binary to the local cargo bin directory (usually ~/.cargo/bin).

    The `cargo-binutils` crate provides the `cargo objcopy` command, which we will use to convert the compiled Rust code into a binary file that we can load onto the Microbit.

    The `cargo objcopy` command is a wrapper around the `llvm-objcopy` tool provided by the `llvm-tools` component. It simplifies the process of converting the compiled Rust code into a binary file that we can load onto the Microbit.

1. Install the `probe-rs-tools` crate to provide tools for working with the Microbit.

    ```sh
    cargo install --locked probe-rs-tools --vers '^0.24'
    ```

    The `probe-rs-tools` crate is an embedded debugging and target interaction toolkit. It enables its user to program and debug microcontrollers via a debug probe. It helps withs:

    - Flashing firmware to ARM and RISC-V targets. More architectures are in the works.
    - Reading and writing memory, running, halting, setting and reading breakpoints and resetting the target via SWD and JTAG.
    - Running firmware on the target and getting back logs via RTT and defmt and printing a stacktrace on panic.
    - Debugging the target via VS Code with running RTT logs, memory inspection and more.

    **Note** - if the install fails on Ubuntu because `libudev` cannot be found, run the following command:

    ```sh
    sudo apt install -y libudev-dev
    ```

    **Note** - if the install fails on Windows ensure `CMake` is installed via the Visual Studio Installer. Also, you will probably need to use the Developer Powershell for VS 2022 terminal.

1. Verify the installation of `cargo embed`.

    ```sh
    cargo embed --version
    ```

    The `cargo embed` command is a wrapper around the `probe-rs` tool provided by the `probe-rs` crate. It simplifies the process of loading the compiled Rust code onto the Microbit.

## Operating System Specific Instructions

The following links provide instructions for installing the tools on different operating systems:

- [Linux (Debian/Ubuntu)](#linux-ubuntu)
- [macOS](#macos)
- [Windows](#windows)
- [Windows WSL 2 (Ubuntu)](#windows-wsl-2-ubuntu)

### Linux (Ubuntu)

1. Install `gdb-multiarch` and `tio` to debug the Microbit.

    ```sh
    sudo apt-get install -y gdb-multiarch tio usbutils
    ```

    The `gdb-multiarch` package provides the `gdb-multiarch` command, which is a version of the GNU Debugger that supports multiple architectures. We will use it to debug the Microbit.

    The `tio` package provides the `tio` command, which is a terminal emulator that we will use to communicate with the Microbit.

1. Create/Edit the file `/etc/udev/rules.d/69-microbit.rules`. Add the following content to it. Save the file.

    ```text
    # CMSIS-DAP for microbit

    ACTION!="add|change", GOTO="microbit_rules_end"

    SUBSYSTEM=="tty", ENV{ID_VENDOR_ID}=="0d28", ENV{ID_MODEL_ID}=="0204", TAG+="uaccess"
    SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", TAG+="uaccess"

    LABEL="microbit_rules_end"
    ```

    The rules file tells the system to give the user access to the Microbit when it is plugged in.

    The `ATTR{idVendor}` and `ATTR{idProduct}` values are the USB vendor and product IDs for the Microbit. The `TAG+="uaccess"` directive tells the system to give the user access to the Microbit. The `SUBSYSTEM=="usb"` rule enables `probe-rs` and `cargo embed` to flash the Microbit.

    The `ENV{ID_VENDOR_ID}` and `ENV{ID_MODEL_ID}` values are the USB vendor and product IDs for the Microbit. The `TAG+="uaccess"` directive tells the system to give the user access to the Microbit. The `SUBSYSTEM=="tty"` rule enables `tio` to communicate with the Microbit.

1. Open a terminal, and run the following command.

    ```sh
    sudo udevadm control --reload
    ```

    The `udevadm` command is used to control the udev daemon, which manages device nodes in the `/dev` directory. The `control --reload` subcommand tells the udev daemon to reload its rules.

1. If any boards are plugged in, unplug and plug them back in. Alternatively, run the following command in the terminal.

    ```sh
    sudo udevadm trigger
    ```

    The `udevadm` command is used to control the udev daemon, which manages device nodes in the `/dev` directory. The `trigger` subcommand tells the udev daemon to reapply its rules.

1. Connect the Microbit to the computer using a USB cable.

1. In the terminal, run the following command.

    ```sh
    lsusb | grep -i "NXP ARM mbed"
    ```

    The output will look something like this:

    ```text
    Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
    ```

    The `lsusb` command lists USB devices connected to the system. The `grep -i "NXP ARM mbed"` command filters the output to show only lines that contain the string "NXP ARM mbed". If the Microbit is connected, you should see a line that contains the string "NXP ARM mbed".

1. In the terminal, run the following command. Replace `001` and `065` with the bus and device numbers from the previous step.

    ```sh
    ls -l /dev/bus/usb/001/065
    ```

    Verify the permissiond are set correctly. The output will look something like this:

    ```text
    crw-rw-r--+ 1 nobody nobody 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
    ```

    The permissions should be crw-rw-r--+, note the + at the end, then see your access rights by running the following command.

1. Run the following command in the terminal to verify your access rights. Replace `001` and `065` with the bus and device numbers from the previous step.

    ```sh
    getfacl /dev/bus/usb/001/065
    ```

    You should see your username in the list above with the rw- permissions, if not, then check your udev rules and try re-loading them with:

    ```sh
    sudo udevadm control --reload
    ```

    ```sh
    sudo udevadm trigger
    ```

### macOS

1. Install

    ```sh
    brew install arm-none-eabi-gdb
    ```

    The `arm-none-eabi-gdb` package provides the `arm-none-eabi-gdb` command, which is a version of the GNU Debugger that supports the ARM architecture. We will use it to debug the Microbit.

    Test the installation by running the following command.

    ```sh
    arm-none-eabi-gdb --version
    ```

1. Install `tio` to communicate with the Microbit.

    ```sh
    brew install tio
    ```

    The `tio` package provides the `tio` command, which is a terminal emulator that we will use to communicate with the Microbit.

    Test the installation by running the following command.

    ```sh
    tio --version
    ```

1. Install `lsusb` to list USB devices.

    ```sh
    brew install lsusb
    ```

    The `lsusb` package provides the `lsusb` command, which lists USB devices connected to the system.

    Test the installation by running the following command.

    ```sh
    lsusb -V
    ```

### Windows

1. Install `arm-none-eabi-gcc` for Windows from [https://developer.arm.com/downloads/-/gnu-rm](https://developer.arm.com/downloads/-/gnu-rm). Select all options when configuring the tool.

    The `arm-none-eabi-gcc` package provides the `arm-none-eabi-gcc` command, which is a version of the GNU Compiler Collection that supports the ARM architecture. We will use it to compile the Rust code for the Microbit.

1. Close all terminal windows, and open a new terminal window to confirm you can run the debugger.

    ```sh
    arm-none-eabi-gdb --version
    ```

1. Install `putty` for from [https://www.chiark.greenend.org.uk/~sgtatham/putty/](https://www.chiark.greenend.org.uk/~sgtatham/putty/)

    The `putty` package provides the `putty` command, which is a terminal emulator that we will use to communicate with the Microbit.

### Windows WSL 2 (Ubuntu)

1. Open a WSL Ubuntu terminal, and install `gdb-multiarch` and `tio` to debug the Microbit.

    ```sh
    sudo apt-get install -y gdb-multiarch tio usbutils acl
    ```

    The `gdb-multiarch` package provides the `gdb-multiarch` command, which is a version of the GNU Debugger that supports multiple architectures. We will use it to debug the Microbit.

    The `tio` package provides the `tio` command, which is a terminal emulator that we will use to communicate with the Microbit.

    The `usbutils` package provides the `lsusb` command, which lists USB devices connected to the system.

    The `acl` package provides the `getfacl` command, which lists file access control lists.

1. Create/Edit the file `/etc/udev/rules.d/69-microbit.rules`. Add the following content to it. Save the file.

    ```text
    # CMSIS-DAP for microbit

    ACTION!="add|change", GOTO="microbit_rules_end"

    SUBSYSTEM=="tty", ENV{ID_VENDOR_ID}=="0d28", ENV{ID_MODEL_ID}=="0204", TAG+="uaccess", MODE="666"
    SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", TAG+="uaccess", MODE="666"

    LABEL="microbit_rules_end"
    ```

    The rules file tells the system to give the user access to the Microbit when it is plugged in.

    The `ATTR{idVendor}` and `ATTR{idProduct}` values are the USB vendor and product IDs for the Microbit. The `TAG+="uaccess"` directive tells the system to give the user access to the Microbit. The `SUBSYSTEM=="usb"` rule enables `probe-rs` and `cargo embed` to flash the Microbit.

    The `ENV{ID_VENDOR_ID}` and `ENV{ID_MODEL_ID}` values are the USB vendor and product IDs for the Microbit. The `TAG+="uaccess"` directive tells the system to give the user access to the Microbit. The `SUBSYSTEM=="tty"` rule enables `tio` to communicate with the Microbit.

1. Open a terminal, and run the following command.

    ```sh
    sudo udevadm control --reload
    ```

    The `udevadm` command is used to control the udev daemon, which manages device nodes in the `/dev` directory. The `control --reload` subcommand tells the udev daemon to reload its rules.

1. If any boards are plugged in, unplug and plug them back in. Alternatively, run the following command in the terminal.

    ```sh
    sudo udevadm trigger
    ```

    The `udevadm` command is used to control the udev daemon, which manages device nodes in the `/dev` directory. The `trigger` subcommand tells the udev daemon to reapply its rules.

1. Connect the Microbit to the computer using a USB cable.

1. Open a PowerShell terminal, and run the following command.

    ```sh
    winget install usbipd
    ```

1. List connected USB devices.

    ```sh
    usbipd list
    ```

    The output will look something like this:

    ```text
    Connected:
    BUSID  VID:PID    DEVICE                                                        STATE
    2-2    0d28:0204  USB Mass Storage Device, USB Serial Device (COM3), USB In...  Shared
    2-7    5986:2170  BisonCam,NB Pro                                               Not shared
    2-10   8087:0036  Intel(R) Wireless Bluetooth(R)                                Not shared
    ```

    The Microbit VID:PID is "0d28:0204". Note the `BUSID` for the Microbit.

1. Open a PowerShell terminal with Administrator Privleges, then attach the USB device to the WSL 2 VM.

    ```sh
    usbipd bind --busid 2-2
    ```

    Replace `2-2` with the `BUSID` for your Microbit. The attachment must be done each time the Microbit is connected and disconnected from the computer.

1. Open a PowerShell terminal with Administrator Privleges, then attach the USB device to the WSL 2 VM.

    ```sh
    usbipd attach --wsl -b 2-2
    ```

    Replace `2-2` with the `BUSID` for your Microbit. The attachment must be done each time the Microbit is connected and disconnected from the computer.


1. In the terminal, run the following command.

    ```sh
    lsusb | grep -i "NXP ARM mbed"
    ```

    The output will look something like this:

    ```text
    Bus 001 Device 065: ID 0d28:0204 NXP ARM mbed
    ```

    The `lsusb` command lists USB devices connected to the system. The `grep -i "NXP ARM mbed"` command filters the output to show only lines that contain the string "NXP ARM mbed". If the Microbit is connected, you should see a line that contains the string "NXP ARM mbed".

1. In the terminal, run the following command. Replace `001` and `065` with the bus and device numbers from the previous step.

    ```sh
    ls -l /dev/bus/usb/001/065
    ```

    Verify the permissiond are set correctly. The output will look something like this:

    ```text
    crw-rw-r--+ 1 nobody nobody 189, 64 Sep  5 14:27 /dev/bus/usb/001/065
    ```

    The permissions should be crw-rw-r--+, note the + at the end, then see your access rights by running the following command.

1. Run the following command in the terminal to verify your access rights. Replace `001` and `065` with the bus and device numbers from the previous step.

    ```sh
    getfacl /dev/bus/usb/001/065
    ```

    You should see your username in the list above with the rw- permissions, if not, then check your udev rules and try re-loading them with:

    ```sh
    sudo udevadm control --reload
    ```

    ```sh
    sudo udevadm trigger
    ```

## Verify Connection to Microbit

1. Ensure your Microbit is connected to your computer via USB.

1. In the terminal, run the following command.

    ```sh
    probe-rs list
    ```

    You should see a list of connected probes, including the Microbit.
