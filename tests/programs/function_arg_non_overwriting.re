def(f, x, _(
    # this used to leak the x = 0 arg to the outer scope
    overwrite(0),
    x
)),

def(overwrite, x, null),

assert_eq(f(1), 1),