#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>

void __prints__(char* s) {
    printf("%s", s);
}

void __printi__(int i) {
    printf("%i", i);
}

void __flush__() {
    fflush(stdout);
}

char* __getchar__() {
    char* c = malloc(sizeof(char) * 2);
    scanf("%c", c);
    fflush(stdin);
    c[1] = '\0';
    return c;
}

int __ord__(char* s) {
    switch (s[0]) {
    case '1': return 1;
    case '2': return 2;
    case '3': return 3;
    case '4': return 4;
    case '5': return 5;
    case '6': return 6;
    case '7': return 7;
    case '8': return 8;
    case '9': return 9;
    default: return -1;
    }
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
