# Linux Serial Communication Tool Project

These instructions work for Linux and Windows WSL 2 Linux.

1. Connect the Microbit via USB to the computer.

1. Run the following command to determine the TTY name.

    ```sh
    sudo dmesg | grep tty
    ```

    The output should be similar to this. If the device has been plugged in and unplugged multiple times, there may be multiple entries for the USB ACM device.

    ```text
    [    0.194755] printk: legacy console [tty0] enabled
    [  380.728430] cdc_acm 3-2:1.1: ttyACM0: USB ACM device
    ```

    The `ttyACM0` is the device node of the Microbit. Use this device node when running the following commands.

1. The device node is accessed via the file system because everything is a file in Linux.

    ```sh
    ls -l /dev/ttyACM0
    ```

    The output should be similar to this.

    ```text
    crw-rw---- 1 root dialout 166, 0 Oct  5 09:55 /dev/ttyACM0
    ```

1. Run the following command to output data to the Microbit.

    ```sh
    echo 'Hello, world!' > /dev/ttyACM0
    ````

    The 'Hello, world!' string will not be displayed in the terminal, but the orange light on the Microbit will flash each time you run the command.

1. Run the Tio program to establish a serial connection to the Microbit. The default settings for Tio are sufficient.

    ```sh
    tio /dev/ttyACM0
    ```

1. Open a new terminal window, and change into the `projects/06_microbit_send_string` folder. Run the following command to flash the program to the Microbit.

    ```sh
    cargo embed
    ```

1. Review the `tio` console and the text "The quick brown fox jumps over the lazy dog." should appear. Push the reset button on the Microbit and the text should be outputted to the Tio console again and again.

1. To exit Tio, in the Tio terminal window, type `ctrl+t`, followed by `q`.

1. To exit the Rust embedded program, type `ctrl+c`.
