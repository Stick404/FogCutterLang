# Definitions
This file contains the defitnitions to all the Fog Cutter specific short hands used in both the code and documentation:

* Fog Cutter (FC), the "root" project name that hosts a VM, Compiler, Low Level language, and High Level language

* FC-VM (Fog Cutter Virtual Machine), the Byte Code interpreter that reads Byte Code and runs the programs. Currently as of writing on its 4th iteration

* FC-HL (Fog Cutter High Language), also likely will be known as just "Fog Cutter." This is the higher level language that will get compiled into FC-LL

* FC-LL (Fog Cutter Low Language), the Byte Code its self, and the stuff that is read by the FC-VM. The output of passing FC-HL into the FC-C

* FC-C (Fog Cutter Compiler), the Compiler. FC-C will take a simple String representing a program file, and processes it into FC-LL bytes

* The Bus, this is the main way for FC to interact with outside 
systems. The Bus lets FC programs tell the FC-VM to send byte arrays and Objects to the program hosting the FC-VM

* Objects, this is the data stored inside of a FC program. Objects are anything that holds data and that can be passed around inside of a FC program.

