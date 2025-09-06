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