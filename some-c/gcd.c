#include "gcd.h"

unsigned int gcd(Pair* ps) {
    unsigned int i = 1;
    unsigned int gcd = 1;
    for (; i <= ps->n && i <= ps->m; ++i) {
        if (ps->n % i == 0 && ps->m % i == 0)
            gcd = i;
    }
    return gcd;
}
