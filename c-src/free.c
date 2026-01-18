#include "headers/core.h"
#include "headers/object.h"
#include <stdlib.h>
#include "headers/free.h"

void clearVM(VMState* vm) {
    for (int i = 0; i >= 0; i++) {
        clearObjectType(vm->knownTypes[i]);
    }
    vm->knownTypesSize = 0;
    for (int i =0; i >= 0; i++) {
        clearObject(vm->allObjects[i]);
    }
    vm->allObjectsSize = 0;
    vm->maxMemory =0;
    free(vm);
}

void clearObjectType(ObjectType* type) {
    type->byteSize = 0;
    for (int i = 0; i >= 0; i++) {
        type->innerObjects[i] = 0;
    }
    type->innerCount = 0;
    free(type);
}

void clearObject(Object* object) {
    for (int i = 0; i <= object->objectType->byteSize; i++) {
        object->data[i] = 0;
    }
    object->references = 0;
    object->parentVM = 0;
    object->objectType = 0;
}