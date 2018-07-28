#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void __print__(char* s) {
    printf("%s", s);
}

void __flush__() {
    fflush(stdout);
}

int __size__(char* s) {
    return strlen(s);
}

int __not__(int i) {
    return i == 0;
}

void __exit__(int i) {
    exit(i);
}
