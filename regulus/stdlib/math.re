def(abs, x, ifelse(>=(x, 0), x, -(0, x))),

# TODO: blocked on `function_locals_same_name_leaked.re` getting resolved
#def(gcd, a, b, ifelse(
#    >(a, b),
#    gcd(b, a),
#    ifelse(
#        ==(%(b, a), 0),
#        a,
#        gcd(%(b, a), a)
#    )
#)),
