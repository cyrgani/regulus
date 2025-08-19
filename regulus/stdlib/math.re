def(abs, x, ifelse(>=(x, 0), x, -(0, x))),

# TODO: better errors when passing 0 as an argument
def(gcd, a, b, ifelse(
    >(a, b),
    gcd(b, a),
    ifelse(
        ==(%(b, a), 0),
        a,
        gcd(%(b, a), a)
    )
)),
