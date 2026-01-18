#ifndef FREE_H
#define FREE_H
#include "object.h"

// This header declares all methods of clearing Objects. Once an Object is cleared, its pointer is freed.
// **IMPORTANTLY: THESE DO NOT CHECK IF IT *SHOULD* BE CLEARED**

// Clears this VMState, including all Objects, Object Types, and other variables in it
void clearVM(VMState* vm);
// Clears this Object Type, does not clear sub types
void clearObjectType(ObjectType* object);
// Only Clears the Object/data, not the Object Type
void clearObject(Object* object);


#endif