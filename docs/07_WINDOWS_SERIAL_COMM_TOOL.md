# Windows Serial Communication Tool Project

1. Disconnect the Microbit from your computer if it is connected.

1. Open a PowerShell terminal windows, and run the following command.

    ```sh
    mode
    ```

    Review the output from the `mode` command. Once, the Microbit is connected, we will be looking for the new device which is added to the output when the `mode` command is executed again.

    ```text
    Status for device CON:
    ----------------------
        Lines:          33
        Columns:        133
        Keyboard rate:  31
        Keyboard delay: 1
        Code page:      437    
    ```

1. Connect the Microbit to your computer via the USB cable, and run the `mode` command again.

    ```sh
    mode
    ```

    The output will look similar to this. Copy and paste the information for the Microbit.

    ```text
    Status for device COM3:
    -----------------------
        Baud:            115200
        Parity:          None
        Data Bits:       8
        Stop Bits:       1
        Timeout:         OFF
        XON/XOFF:        OFF
        CTS handshaking: OFF
        DSR handshaking: OFF
        DSR sensitivity: OFF
        DTR circuit:     OFF
        RTS circuit:     ON


    Status for device CON:
    ----------------------
        Lines:          33
        Columns:        133
        Keyboard rate:  31
        Keyboard delay: 1
        Code page:      437
    ```

1. Run the `putty` program.

1. Click the `Serial` connection type under the `Connection` tree element.

1. Update the `Serial line to connect to` field to the COM port of the Microbit listed in the output of the `mode` command.

1. Set the other Putty serial connection options as shown:

    - Speed (baud) - 115200
    - Data bits - 8
    - Stop bits - 1
    - Parity - None
    - Flow control - None

    These options should align with the settings from the `mode` command output for the Microbit COM port.

1. In the side tree, click the `Session` element, then click `Serial` under the `Connection type`.

1. Click `Open`.

1. A Putty terminal window will open. Click the on the terminal and type into it. Nothing will be displayed, but the orange light will blink with each keystroke. Do not close the Putty terminal window.

1. Open a PowerShell terminal window, change into the `06_microbit_send_string` folder.

1. Run `cargo embed` to flash the Microbit and run the application. If needed, include the `--probe VID:PID` option.

    ```sh
    cargo embed
    ```

    When the program runs it will immediately output the text "The quick brown fox jumps over the lazy dog." over the serial port and it will be display in Putty.

1. If you observed the orange light blink when typing into the Putty terminal and you received the "The quick brown fox jumps over the lazy dog." text in the Putty terminal window when flashing the program to the Microbit then the serial communication is working correctly.