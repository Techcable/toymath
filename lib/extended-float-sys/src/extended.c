#define _GNU_SOURCE 1
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <math.h>

typedef uint8_t ExtendedFloat[10];

#define EXPECTED_SIZE 16
#define VERIFY_SIZE() if (sizeof(long double) != EXPECTED_SIZE) { \
    fprintf(stderr, "Invalid sizeof(long double), expected %d but got %lu\n", EXPECTED_SIZE, sizeof(long double)); \
    abort(); \
}
#define BINARY_OP(name, return_type, code) extern return_type name(const ExtendedFloat *first_ptr, const ExtendedFloat *second_ptr) {\
    VERIFY_SIZE(); \
    long double first = *((long double*) first_ptr); \
    long double second = *((long double*) second_ptr); \
    code \
}
#define INPLACE_BINARY_OP(name, code) extern void name(ExtendedFloat *first_ptr, const ExtendedFloat *second_ptr) {\
    VERIFY_SIZE(); \
    long double first = *((long double*) first_ptr); \
    long double second = *((long double*) second_ptr); \
    long double result = code; \
    *((long double*) first_ptr) = result; \
}
#define UNARY_OP(name, return_type, code) extern return_type name(ExtendedFloat *first_ptr) {\
    VERIFY_SIZE(); \
    long double first = *((long double*) first_ptr); \
    code; \
}

#define INPLACE_UNARY_OP(name, code) extern void name(ExtendedFloat *first_ptr) {\
  VERIFY_SIZE(); \
  long double first = *((long double*) first_ptr); \
  long double result = code; \
  *((long double*) first_ptr) = result; \
}

INPLACE_BINARY_OP(extended_add, first + second);
INPLACE_BINARY_OP(extended_sub, first - second);
INPLACE_BINARY_OP(extended_mul, first * second);
INPLACE_BINARY_OP(extended_div, first / second);
INPLACE_BINARY_OP(extended_mod, fmodl(first, second));
INPLACE_BINARY_OP(extended_min, fminl(first, second));
INPLACE_BINARY_OP(extended_max, fmaxl(first, second));
INPLACE_BINARY_OP(extended_pow, powl(first, second));
INPLACE_BINARY_OP(extended_hypot, hypotl(first, second));


INPLACE_UNARY_OP(extended_sqrt, sqrtl(first));
INPLACE_UNARY_OP(extended_abs, fabsl(first));
INPLACE_UNARY_OP(extended_signum, -first);
INPLACE_UNARY_OP(extended_ceil, ceill(first));
INPLACE_UNARY_OP(extended_floor, floorl(first));
INPLACE_UNARY_OP(extended_round, roundl(first));
INPLACE_UNARY_OP(extended_trunc, truncl(first));
INPLACE_UNARY_OP(extended_neg, -first);
INPLACE_UNARY_OP(extended_exp, expl(first));
INPLACE_UNARY_OP(extended_exp_m1, expm1l(first));
INPLACE_UNARY_OP(extended_exp2, exp2l(first));
INPLACE_UNARY_OP(extended_ln, logl(first));
INPLACE_UNARY_OP(extended_ln_1p, log1pl(first));
INPLACE_UNARY_OP(extended_log2, log2l(first));
INPLACE_UNARY_OP(extended_log10, log10l(first));
INPLACE_UNARY_OP(extended_cbrt, cbrtl(first));
INPLACE_UNARY_OP(extended_sin, sinl(first));
INPLACE_UNARY_OP(extended_cos, cosl(first));
INPLACE_UNARY_OP(extended_tan, tanl(first));
INPLACE_UNARY_OP(extended_asin, asinl(first));
INPLACE_UNARY_OP(extended_acos, acosl(first));
INPLACE_UNARY_OP(extended_atan, atanl(first));
INPLACE_UNARY_OP(extended_sinh, sinhl(first));
INPLACE_UNARY_OP(extended_cosh, coshl(first));
INPLACE_UNARY_OP(extended_tanh, tanhl(first));
INPLACE_UNARY_OP(extended_asinh, asinhl(first));
INPLACE_UNARY_OP(extended_acosh, acoshl(first));
INPLACE_UNARY_OP(extended_atanh, atanhl(first));

UNARY_OP(extended_isfinite, bool, return isfinite(first))
UNARY_OP(extended_isnan, bool, return isnan(first));
UNARY_OP(extended_isinf, bool, return isinf(first));
UNARY_OP(extended_isnormal, bool, return isnormal(first));
UNARY_OP(extended_signbit, unsigned int, return signbit(first));

BINARY_OP(extended_eq, bool, return first == second;);
BINARY_OP(extended_cmp, int, {
    if (first == second) {
        return 0;
    } else if (first > second) {
        return 1;
    } else if (first < second) {
        return -1;
    } else {
        return 2;
    }
});

extern void extended_mul_add(ExtendedFloat *first_ptr, const ExtendedFloat *second_ptr, const ExtendedFloat *third_ptr) {
    VERIFY_SIZE();
    long double first = *((long double*) first_ptr);
    long double second = *((long double*) second_ptr);
    long double third = *((long double*) third_ptr);
    long double result = fmal(first, second, third);
    *((long double*) first_ptr) = result;
}

extern void extended_modf(ExtendedFloat *first_ptr, ExtendedFloat *iptr) {
    VERIFY_SIZE();
    long double first = *((long double*) first_ptr);
    long double result = modfl(first, (long double*) iptr);
    *((long double*) first_ptr) = result;
}

extern int extended_print(const ExtendedFloat *first_ptr, int width, int precision, char **out) {
    VERIFY_SIZE();
    long double first = *((long double*) first_ptr);
    return asprintf(out, "%*.*Lg", width, precision, first);
}
extern void extended_parse(ExtendedFloat *out_ptr, const char *data, char **end) {
    VERIFY_SIZE();
    long double result = strtold(data, end);
    *((long double*) out_ptr) = result;
}

extern void extended_convert_from_f64(ExtendedFloat *out_ptr, double data) {
    VERIFY_SIZE();
    long double result = (long double) data;
    *((long double*) out_ptr) = result;
}
extern void extended_convert_from_f32(ExtendedFloat *out_ptr, float data) {
    VERIFY_SIZE();
    long double result = (long double) data;
    *((long double*) out_ptr) = result;
}
extern void extended_convert_from_i64(ExtendedFloat *out_ptr, int64_t data) {
    VERIFY_SIZE();
    long double result = (long double) data;
    *((long double*) out_ptr) = result;
}
extern void extended_convert_from_u64(ExtendedFloat *out_ptr, uint64_t data) {
    VERIFY_SIZE();
    long double result = (long double) data;
    *((long double*) out_ptr) = result;
}
extern double extended_convert_into_f64(const ExtendedFloat *first_ptr) {
    VERIFY_SIZE();
    long double first = *((long double*) first_ptr);
    return (double) first;
}
extern float extended_convert_into_f32(const ExtendedFloat *first_ptr) {
    VERIFY_SIZE();
    long double first = *((long double*) first_ptr);
    return (float) first;
}
extern int64_t extended_convert_into_i64(const ExtendedFloat *first_ptr) {
    VERIFY_SIZE();
    // TODO: This may be unsafe (although the stdlib does it too)
    long double first = *((long double*) first_ptr);
    return (int64_t) first;
}
extern uint64_t extended_convert_into_u64(const ExtendedFloat *first_ptr) {
    VERIFY_SIZE();
    // TODO: This may be unsafe (although the stdlib does it too)
    long double first = *((long double*) first_ptr);
    return (uint64_t) first;
}
