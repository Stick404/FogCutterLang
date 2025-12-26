# Random Ramblings:
**ALL OF THESE CAN AND LIKELY WILL CHANGE**
- Most things can be abstracted away, since this is an emulation of a real CPU, not a real CPU
- Having the program and R/W memory share a space seems dangerous but also like an interesting idea
- Likely have 4 registers (r0-3)
-ram Call Stack has a defined space in ram, if it overflows this it gets a StackOverFlow
- May have specific outputs/registers (rather than memory space) to write things to files.
This is to enforce file safety and checking
- Will have an ALU that uses r0-1 for math, and r2 for outputs
- Jmp and methods like it will use r0-1 and r3
- The system will be signed 64bit based
- Native support for doubles and longs only
- Overflows will be caught, and will jump to a specific memory slot for error handling/crashs(maybe)
- Errors will write the error code into a register
- An "error" of 1 written to the error register ends a program without calling the crash code
- No error handling code will just end the program with the code
- Registers will be addressed with `r"string"` in the bytecode

### Full list of Registers and what they do:
- r0, writable register used for math/comparisons
- r1, writable register used for math/comparisons
- r2, writable register that is the output of the ALU
- r3, an extra register used for Jump actions

For extra non-standard registers:
- rC, an r/w and the Program Counter used for directly setting the memory section to read from
- rE, an r/w register that is used for storing error codes
- rS, an r/w Stack Pointer register, used for the Call Stack
- rR, an r/w pointer to the Error Return, the code in memory that will be run based on an error
- rT, an r/w number that will tell the VM when to tick next in either Milliseconds or Ticks

### A Clock Cycle:
- Pull instructions from program memory based on rC into rI0-2
- Read rI, and run the Opcode in rI0, with rI1 and rI2 as inputs
- Check if there is an error in rE (IE: not 0/1) and jump to the memory defined in rR
  - If rE contains 1, end the program safely
- Increment rC by 3 if the program has not been halted yet
- Start the next Clock Cycle

### Basic/Planed Instruction Set:
- `Mov, x1, x2`, Moves value x1 to x2, x1 be either value or register, x2 is always a register
- `MovMemIn, x1, x2`, Moves a value from x1 register or value into memory at x2
- `MovMemOut, x1, x2`, Moves a value from memory at x1 register or value into x2 registry
- ALU math Ops, preforms the stated action. All take x1 x2, and outputs into r2. If x1 or x2 is empty (0) it pulls from r0 and r1. Always outputs to r2.
These will also be available for Doubles with the prefix `Dub`; such as `DubAdd`, `DubDiv`, etc. Doubles will be treated as Longs and vice versa if the wrong Op is used
  - `Add, x1, x2` Adds x1 and x2 together
  - `Sub, x1, x2` Subs x2 from x1
  - `Mul, x1, x2` Mults x1 by x2
  - `Div, x1, x2` Dives x1 by x2
  - `Abs, x1` Gets the absolute of x1. Ignores x2
- Jump Ops, preforms the stated action. All take x1 x2, and jumps to the location in r3. If x1 or x2 is empty (is 0) it pulls from r0 and r1 respectively
  - `Jmp` Jumps directly, no condition. Ignores x1, x2
  - `JmpGr, x1, x2` Jumps if x1 is greater than x2
  - `JmpLs, x1, x2` Jumps if x1 is less than x2
  - `JmpEq, x1, x2` Jumps if x1 is equal to x2
  - `JumpGrEq, x1, x2` Jumps if x1 is equal too/greater than x2
  - `JumpLsEq, x1, x2` Jumps if x1 is equal too/less than x2
- `NoOp`, Preforms no action. Ignores x1, x2
- `Call, x1`, Adds the current location to the Stack Pointer, and jumps unconditionally to the location in x1. Ignores x2
- `Ret`, Pops a location off of the Stack Pointer, and moves the program to there. Ignores x1 and x2

**THIS CAN AND WILL LIKELY CHANGE**
# Architecture:
`TODO:`
The emulated CPU will be based on Von Neumann Architecture, where the program and memory exist in a shared space.
There will also be 4 active registers for the CPU to use, r0, r1, r2, and r3;
potently more directly writeable register-like spaces for specific actions (such as the Program Counter, Stack Pointer, etc).
As well, the Function Call Stack will be directly in memory, where if it leaves the allocated bounds; it errors the program.
Errors are handled via 2 custom registers, rE and rR.
rE holds the Error Code of the program, and rR holds a pointer to the section of the program that holds error handling code.
If an error were to happen in rR, the VM will shutdown the program; however more will be explained later.


Each register has a direct use, r0-1 is used by both the ALU for math, and by Jumps for comparisons.
r2 is used by the ALU to write the output of math operations.
r3 is the final register, and used by Jumps to select the section in memory to jump to.
Each of the r0-3 registers are fully read and writable.
As stated before, there is also more registers, they are as follows.
rC, which is the Program **C**ounter (read/writeable), and states where in memory the program should execute from; this is incremented after every CPU tick.
rE, which is the **E**rror register (read/writeable), and states the error code of the program; if the rE is 0, the program keeps running; if the rE is 1, the program cleanly finishes; if the rE is not either 0 or 1, it jumps the code to the next register.
rR, which is the Error **R**eturn (read/writeable), this points to a section in memory where the program should go to if an error is reached; this section of code is not for removing the error, but to safely shutdown the program. If an error were to happen after rR has been triggered, the VM will end the program.
Moving from error handling, rS is the **S**tack Pointer (read/writeable), when Opcode `Call` is used, this register has the current location of the program saved to its memory address pointer, then it increments.
If Opcode `Ret` is used, the current location that rS is pointing to is used to set the rC, then rS is decremented a memory slot.
Finally, for Registers, rT which is the **T**ick Register (read/writeable), this tells the VM how many units of time the program should sleep before running again, baring any errors.


All of this is inspired by Stephen Gream's [articles](https://www.stephengream.com/writing-a-vm-part-one/) on writing a VM with some modifications
