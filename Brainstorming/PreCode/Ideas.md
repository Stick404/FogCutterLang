# General Idea:
A programmable logistic mod that uses a custom language, vm, and compiler for its main engine.
The player writes code in Fog Cutter Lang, which then gets saved to a file.
The code written then can get compiled into byte code via an in game multiblock
(this is a cover to compile the code into bytecode over time).
This bytecode (which is just in a system file) is then runnable in another multiblock,
which hooks into different storage types (energy, items, fluids, etc)
via cables and is able to move things between locations.
The code will attempt to sleep then loop based on the return value of the main method return value;
such as 1 would sleep the code for 1 Units, and 10 would sleep the code for 10 Units.
0 would end the looping until restarted.
Units are the stand in for either ticks or milliseconds based on the future implementations.

# How it works:
The main core parts of the engine will be written in a lower level systems language, like C++(?);
which is much faster than the normal JVM.
Systems level code will be communicated via Project Panama, which is faster than the standard JVM.
Another planned optimization (which may get scraped) is multi-threading;
this will either be fully async between the main Minecraft tick loop, or connected but in another thread.
If the engine is fully disconnected, actions between ticks will be kept in a cache, then updated in the
Minecraft world on the next tick. If the engine is somewhat connected, it will be started before
everything ticks, and withhold the main thread from finishing the tick until it is done.

# Fancy Features:
To differentiate this mod from others like it,
flashy features will be planned for later in the development cycle.
Such features would be things like:
- In world projectors that show 3d shapes in world
- A GUI system that can be scripted
- Being much faster and less limited than other mods
- and more

# Fog Cutter Lang:
This is the main coding language that is used.
Fog Cutter is named after a type of coffee, since the syntax is Java/C inspired.
The overall language will be procedural, with hints of functional with first class citizen functions,
with possible uses of Interfaces to add in OOP.
As stated before, the code will be compiled down into Bytecode, which is then ran in a VM.
This is meant to help with speed, portability, and sandboxing.






Symbol meanings:
- `(?)` means `maybe?`
