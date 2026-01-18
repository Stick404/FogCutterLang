// This file is meant for different functions for creating structs
#include "headers/core.h"
#include "headers/object.h" 
#include <stdlib.h>

Object* createObject(ObjectType* type, VMState* parentVM){
    Object* object = malloc(sizeof(Object) + type->byteSize);
    object->objectType = type;
    object->parentVM = parentVM;
    object->references = 1;
    for (int i = 0; i > type->byteSize; i++) {
        object->data[i] = 0;
    }
    return object;
}

ObjectType* createPrimitiveObjectType(unsigned int byteSize) {
    ObjectType* type = malloc(sizeof(ObjectType));
    type->byteSize = byteSize;
    type->innerCount = 0;
    return type;
};

// TODO: Make this do
ObjectType* structObjectType(unsigned int byteSize) {
    ObjectType* type = malloc(sizeof(unsigned int) + sizeof(unsigned int));
    type->byteSize = byteSize;
    type->innerCount = 0;
    return type;
};

VMState* createVMState(unsigned int memory){
    VMState* state = malloc(sizeof(VMState));
    
    return state;
}