## Fog Cutter Version 5
As V4 progresses, there are core issues that have made them selves known; those being:
* Memory fragmentation
* Memory Management possibly
* How functions and function returns are handled
* The Operand/Call Stack

Some fixes that could be made are:

* Looking into how heap structures function
* Make frames more comprehensive
* Rework local variables and OpCodes
* Possibly using a Garbage Collector rather than Reference Counting

The possible fixes are heavily inspired by the JVM: https://docs.oracle.com/javase/specs/jvms/se26/jvms26.pdf
Look into Crafting Interpreters for more information: https://craftinginterpreters.com


Memory Models to look into:
* Byte Array
    * Quickest, but also kind of painful
    * Each Object is `[Vtable, data]`
    * `VTable` are `[TypeID, MethodPointer]`
* Object List
    * Currently what V4 is doing, but requires a lot of pointers
    * Kind of pointer chasing, but also annoying
* Slab Allocate? (IE: Each Object gets their own space?)
    * Each Object gets their own "space"
    * Feels fucky, but Kernels do it