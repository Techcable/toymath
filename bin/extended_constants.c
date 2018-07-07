#include <stdio.h>
#include <stdbool.h>
#include <math.h>

#define EXPECTED_SIZE 10

void print_hex_array(const char* buf, int len);
void print_repr(const char *name, long double in);

int main() {
    long double in;
    if (sizeof(long double) < EXPECTED_SIZE) {
        fprintf(
            stderr,
            "Expected %d bytes but got %lu\n",
            EXPECTED_SIZE, sizeof(long double)
        );
        return 1;
    }
    printf("Desired constant: ");
    fflush(stdout);
    if (scanf("%Lf[^\n]", &in) != 1) {
        fprintf(stderr, "Unable to read long double from input");
        return 1;
    }
    print_repr("input", in);
    print_repr("pi", acos(-1.0));
    return 0;
}
void print_repr(const char *name, long double in) {
    printf("Binary representation of %s (%Lf): ", name, in);
    print_hex_array((char*) &in, EXPECTED_SIZE);
    printf("\n");
}

void print_hex_array(const char* buf, int len) {
   printf("[");
   bool first = true;
   for (int i = 0; i < len; i++) {
       if (!first) printf(", ");
       printf("0x%02X", (unsigned char)buf[i]);
       first = false;
   }
   printf("]");
}
