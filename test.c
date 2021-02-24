#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

void* malloc_and_write() {
    //Disable if you want to stop the debugger here
    // __builtin_trap();
    // You can get the value of heap by examining registers and memory at this trap
    // On my machine, the gdb command is "x /1gx $rdi + 0xc8"
    uint32_t* p = (uint32_t*) malloc(4);
    printf("Webassembly writing index: %p\n", p);
    *p = 42;
    return p;
}

int main(int argc, char** argv) {
    return (int) malloc_and_write();
}