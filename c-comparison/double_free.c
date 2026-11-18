// C comparison: double free
//
// Two pointers to the same heap block, both freed.
// This compiles with no error. The bug only shows up at runtime.
//
// Build & run normally (behavior is undefined, may crash or appear to work):
//   gcc double_free.c -o double_free.out && ./double_free.out
//
// Build & run with AddressSanitizer to see the bug caught at runtime:
//   gcc -fsanitize=address -g double_free.c -o double_free.out && ./double_free.out

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *s1 = malloc(6);
    strcpy(s1, "hello");

    char *s2 = s1; // copies the pointer: two pointers, one block

    printf("s2 = %s\n", s2);

    free(s1);
    free(s2); // double free: undefined behavior

    return 0;
}
