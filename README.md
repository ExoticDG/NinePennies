A relitivly simple Rust program I made that takes an input and gives an output. Why do math when you can do Rust.

The AI_Rewrites folder contains a Python script verstion, writen by Google Gemini (I was too lasy to do Python today.) so this program can run on a TI 84 Plus CE Python. 

The Python edition of the TI 84 Plus has a seperate 32 Bit ARM Cortex MCU (Atmel ATSAMD21E18A) that is dedicated to running Python. It communicates with the 8 Bit main MCU (eZ80) via a serial interface. The main MCU runns the calculator and TI-Basic programs. 

This is why this program is only ported to Python insted of TI-Basic as well. 8 Bit intigers are too small for this program. This highest value for an unnusigned 8 Bit intiger is 255. The signed range is -127 to 127.

(Signed intigers are intigers with either a posive or a negitve)

For 32 Bit intigers the unnasigned maximum is 4,294,967,295. Signed intigers can be -2,147,483,64 up to 2,147,483,647. 

This program ustilises mainly 16 bit unnasigned intigers which alot a maximum value of 65,535.

