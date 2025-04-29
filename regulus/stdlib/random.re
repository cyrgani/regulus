import(math),

global(__stl_rng_state),
=(__stl_rng_state, now_nanos_part()),

# DEPRECATED
def(random_u16, _(
    # source: https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
    =(a, 75),
    =(c, 74),
    =(m, 65537),
    =(X, now_nanos_part()),
    %(+(*(a, X), c), m)
))

# returns a random i64
def(rand, _(
    # source: https://en.wikipedia.org/wiki/Xorshift#Example_implementation
    =(x, __stl_rng_state),
    =(x, ^(x, <<(x, 13))),
    =(x, ^(x, >>(x, 7))),
    =(x, ^(x, <<(x, 17))),
    =(__stl_rng_state, x),
    x
))

# returns a random i64 in low..high
# raises if low >= high
def(randrange, low, high, _(
    =(diff, -(high, low)),
    if(<=(diff, 0), error("called randrange with an empty range")),
    +(low, abs(%(rand(), diff)))
))