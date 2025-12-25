# Random Ramblings:
- Most things can be abstracted away, since this is an emulation of a real CPU, not a real CPU
- Having the program and R/W memory share a space seems dangerous but also like an interesting idea
- Likely have 4 registers (r0-3)
- Call Stack has a defined space, if it overflows this it gets a StackOverFlow
- May have specific outputs/registers (rather than memory space) to write things to files.
This is to enforce file safety and checking
- Will have an ALU that uses r0-1 for math, and r2 for outputs
- Jmp and methods like it will use r2 and r3
- The system will be 64bit based
- Native support for doubles and longs only
- Overflows will be caught, and will jump to a specific memory slot for error handling/crashs(maybe)
- Errors will write the error code into a register
- An "error" of 1 written to the error register ends a program without calling the crash code
- No error handling code will just end the program with the code

**ALL OF THESE CAN AND LIKELY WILL CHANGE** (both to top and below)
## Full list of Registers and what they do:
- r0, writable register used for math/comparisons
- r1, writable register used for math/comparisons
- r2, writable register that is the output of the ALU
- r3, an extra register used for Jump actions

For extra non-standard registers:
- rC, an r/w and the Program Counter used for directly setting the memory section to read from
- rE, a read-only register that is used for storing error codes
- rI0-2, r/w registers used for running the next instructions (Op, Arg 1, Arg 2)
- rS, an r/w Stack Pointer register, used for the Call Stack
- rR, an r/w pointer to the Error Return, the code in memory that will be ran based on an error

**THIS CAN AND WILL LIKELY CHANGE**
# Architecture:
The emulated CPU will be based on Von Neumann Architecture, where the program and memory exist in a shared space.
There will also be 4 active registers for the CPU to use, r0, r1, r2, and r3;
potently more directly writeable register-like spaces for specific actions (such as the Program Counter, Stack Pointer, etc).
As well, the Function Call Stack will be directly in memory, where if it leaves the allocated bounds; errors.


All of this is inspired by Stephen Gream's [articles](https://www.stephengream.com/writing-a-vm-part-one/) on writing a VM with some modifications