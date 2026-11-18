// C comparison: use-after-free
//
// Memory is freed through one pointer, then read through another.
// Compiles fine. Undefined behavior at runtime.
//
//   gcc -fsanitize=address -g use_after_free.c -o use_after_free.out && ./use_after_free.out

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *s1 = malloc(6);
    strcpy(s1, "hello");

    char *s2 = s1;

    free(s2);

    printf("s1 = %s\n", s1); // use-after-free: reading freed memory

    return 0;
}
