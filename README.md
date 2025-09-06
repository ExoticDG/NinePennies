A relatively simple Rust program I made that takes an input and gives an output. Why do math when you can do Rust.

The AI_Rewrites folder contains a Python script version, written by Google Gemini 2.5 Pro within GitHub Copilot in Visual Studio Code (I was too lazy to do Python today.) so this program can run on a TI 84 Plus CE Python. 

The Python isn't fully AI, I changed how it worded some of the CLI text back to what it originally was in the Rust version. I would have changed it all but I dident want to mess with how it works. If you want to change things back to what is said in the Rust program, or improve either program, feel free to. 

The Python edition of the TI 84 Plus has a separate 32 Bit ARM Cortex MCU (Atmel ATSAMD21E18A) that is dedicated to running Python. It communicates with the 8 Bit main MCU (eZ80) via a serial interface. The main MCU runns the calculator and TI-Basic programs. 

This is why this program is only ported to Python insted of TI-Basic as well. 8 Bit intigers are too small for this program. This highest value for an unnusigned 8 Bit intiger is 255. The signed range is -127 to 127.

(Signed intigers are intigers with either a posive or a negitve)

For 32 Bit intigers the unnasigned maximum is 4,294,967,295. Signed intigers can be -2,147,483,64 up to 2,147,483,647. 

This program ustilises mainly 16 bit unnasigned intigers which alot a maximum value of 65,535.

## Build / Run

### Computer -- Rust --

**ADD RUST SETUP**

cargo run

### Computer -- Python --

1. Install Python if not already installed

2. In the terminal, ensure you are within the `\AI_Rewrites\Python\` folder in this program. If not, use `cd` until there. If already in the main directory ending for this program then you can just use `cd AI_Rewrites\Python\`

3. In the terminal, run `python NinePennies.py`

### TI 84 Plus CE Python

1. Download the NinePennies.py script

2. Download and Install *[TI Connect™ CE](https://education.ti.com/en/products/computer-software/ti-connect-ce-sw)*

3. Connect the TI-84 Plus CE Python to the computer via the included mini USB-B to USB-A cable

4. Run TI Connect™ CE and ensure the calculator is connected

5. In the TI Connect™ CE software, open the `Calculator Explorer Workspace` (the second tab)

![TI Boot Screen](https://github.com/ExoticDG/NinePennies/blob/9350548323f4c958fdde1adb9a7a95fa1c0e6b7c/assests/TI%20Connect%E2%84%A2%20CE%20Boot%20Screen.png)

6. Click on `Add content from computer to calculators`

![TI Calc Explorer Window](https://github.com/ExoticDG/NinePennies/blob/9350548323f4c958fdde1adb9a7a95fa1c0e6b7c/assests/TI%20Connect%E2%84%A2%20CE%20Cacl%20Exploror%20Window.png)

7. Navigate to the Python program

8. Upload and name it

![TI Name 1](https://github.com/ExoticDG/NinePennies/blob/9350548323f4c958fdde1adb9a7a95fa1c0e6b7c/assests/TI%20Connect%E2%84%A2%20CE%20Name.png)

![TI Name 2](https://github.com/ExoticDG/NinePennies/blob/9350548323f4c958fdde1adb9a7a95fa1c0e6b7c/assests/TI%20Connect%E2%84%A2%20CE%20Name%202.png)

![TI Name 3](https://github.com/ExoticDG/NinePennies/blob/9350548323f4c958fdde1adb9a7a95fa1c0e6b7c/assests/TI%20Connect%E2%84%A2%20CE%20Name%203.png)

![TI Upload](https://github.com/ExoticDG/NinePennies/blob/9350548323f4c958fdde1adb9a7a95fa1c0e6b7c/assests/TI%20Connect%E2%84%A2%20CE%20Upload.png)

9. On the calculator, press the `apps` key

10. Navigate down to `Python` and select it

11. Navigate down to the listing that matches the name you input earlier and select it to run.
