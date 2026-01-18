#ifndef CREATE_H
#define CREATE_H

#include "object.h"
#include "core.h"

Object* createObject(ObjectType* type, VMState* parentVM);

ObjectType* createPrimitiveObjectType(unsigned int byteSize);
ObjectType* createStructObjectType(unsigned int byteSize);

VMState* createVMState(unsigned int memory);

#endif