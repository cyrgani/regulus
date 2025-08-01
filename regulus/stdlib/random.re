import(math),

global(__stl_rng_state),
=(__stl_rng_state, __builtin_now_nanos_part()),

# returns a random i64
def(rand, _(
    # source: https://en.wikipedia.org/wiki/Xorshift#Example_implementation
    =(x, __stl_rng_state),
    =(x, ^(x, <<(x, 13))),
    =(x, ^(x, >>(x, 7))),
    =(x, ^(x, <<(x, 17))),
    =(__stl_rng_state, x),
    x
)),

# returns a random i64 in low..high
# raises if low >= high
def(randrange, low, high, _(
    =(diff, -(high, low)),
    if(<=(diff, 0), error("Range", "called randrange with an empty range")),
    +(low, abs(%(rand(), diff)))
)),

# returns a random element of the given sequence
def(choose, seq, _(
    =(idx, randrange(0, len(seq))),
    index(seq, idx)
)),

def(seed, val, _(
    =(__stl_rng_state, val)
)),

# returns a shuffled version of the given sequence
def(shuffle, seq, _(
    for_in(..(0, len(seq)), i, _(
        =(seq, swap(seq, i, randrange(i, len(seq)))),
    )),
    seq
))
