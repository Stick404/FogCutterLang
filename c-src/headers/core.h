#ifndef CORE_H
#define CORE_H

#include "object.h"
#include <limits.h>

typedef struct VMState {
    unsigned long maxMemory; // The Max amount of Memory assigned to the VM
    int knownTypesSize; // The amount of ObjectTypes known
    ObjectType* knownTypes[SHRT_MAX]; // All the known/registered types within the VMState
    int allObjectsSize; // The amount of all Objects within the VM
    Object* allObjects[]; // Stores all Objects within the VM
} VMState;

#endif