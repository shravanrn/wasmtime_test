#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

void* malloc_and_write() {
    //Disable if you want to stop the debugger here
    // uint32_t a = *((uint32_t*) 0);
    uint32_t* p = (uint32_t*) malloc(4);
    printf("Webassembly writing index: %p\n", p);
    *p = 42;
    return p;
}

int main(int argc, char** argv) {
    return (int) malloc_and_write();
}