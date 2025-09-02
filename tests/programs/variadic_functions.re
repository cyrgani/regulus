=(sum, vfn(_(
    =(s, 0),
    while(>(va_count(), 0), =(s, +(s, va_next()))),
    s
))),

assert_eq(sum(), 0),
assert_eq(sum(1), 1),
assert_eq(sum(1, 2, 4), 7),
assert_eq(sum(-1, 0), -1),

def(sum2, [args], _(
    =(s, 0),
    for_in(args, el, =(s, +(s, el))),
    s
)),

assert_eq(sum2(), 0),
assert_eq(sum2(1), 1),
assert_eq(sum2(1, 2, 4), 7),
assert_eq(sum2(-1, 0), -1),
=(x, 3),
assert_eq(sum2(3, x), 6),

def(non_va_sum, seq, _(
    =(s, 0),
    for_in(seq, el, =(s, +(s, el))),
    s
)),

def(mul_sum, factor, [sum_vals], _(
    # TODO: it would be nice to be able to use `sum2` instead of `non_va_sum`
    *(factor, non_va_sum(sum_vals))
)),

assert_eq(mul_sum(2, 5, 3), 16),
assert_eq(mul_sum(4), 0),
assert_eq(mul_sum(3, 1), 3),
__builtin_print_catch(mul_sum()),
