#ifndef OBJECT_H
#define OBJECT_H

typedef struct ObjectType {
    unsigned int byteSize; // The max size of the Object Type (How much space to allocate)
    unsigned int innerCount; // The amount of Object Types this object holds;
    struct ObjectType* innerObjects[]; // The Inner Object Types this may hold, size can be 0
} ObjectType;

typedef struct Object {
    unsigned int references; // How many Objects point to this Object, once 0 it should be cleared
    struct ObjectType* objectType; // The Object Type for this Object
    struct VMState* parentVM; // The Parent VM
    unsigned char data[]; // The data this holds, assumes the length is the parent Object Type's byteSize
} Object;

#endif