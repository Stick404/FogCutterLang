#include "headers/core.h"
#include "headers/object.h"
#include "headers/create.h"
#include "headers/free.h"
#include <stdlib.h>

int main(){
    VMState* vMState = malloc(sizeof(struct VMState));
    ObjectType* basicByte = createPrimitiveObjectType(1);
    Object* basicObject = createObject(basicByte, vMState);
   
    clearVM(vMState);
    return 0;
}