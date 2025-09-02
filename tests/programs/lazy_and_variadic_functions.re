def(e, error("Bug", "unreachable")),

def(f, [$args], _(
    =(x, index(args, 1)),
    x()
)),

assert_eq(f(e(), 2, print("s")), 2),
# TODO: extend test
