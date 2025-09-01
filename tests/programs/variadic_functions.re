=(sum, vfn(_(
    =(s, 0),
    while(>(va_count(), 0), =(s, +(s, va_next()))),
    s
))),

assert_eq(sum(), 0),
assert_eq(sum(1), 1),
assert_eq(sum(1, 2, 4), 7),
assert_eq(sum(-1, 0), -1),
