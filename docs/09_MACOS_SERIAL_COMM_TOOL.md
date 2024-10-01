# macOS Serial Communication Tool Project

1. Connect the Microbit via USB to the computer.

1. Run the following command to determine the USB device path.

    ```sh
    ls /dev/cu.usbmodem*
    ```

    The output should be similar to this.

    ```text
    /dev/cu.usbmodem102
    ```

1. Run the following command to output data to the Microbit.

    ```sh
    echo 'Hello, world!' > /dev/cu.usbmodem102
    ````

    The 'Hello, world!' string will not be displayed in the terminal, but the orange light on the Microbit will flash each time you run the command.

1. Run the Tio program to establish a serial connection to the Microbit. The default settings for Tio are sufficient.

    ```sh
    tio /dev/cu.usbmodem102
    ```

1. Open a new terminal window, and change into the `projects/06_microbit_send_string` folder. Run the following command to flash the program to the Microbit.

    ```sh
    cargo embed
    ```

1. Review the `tio` console and the text "The quick brown fox jumps over the lazy dog." should appear. Push the reset button on the Microbit and the text should be outputted to the Tio console again and again.

1. To exit Tio, in the Tio terminal window, type `ctrl+t`, followed by `q`.

1. To exit the Rust embedded program, type `ctrl+c`.
