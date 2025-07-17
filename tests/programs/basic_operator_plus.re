type(I, val, =(+, fn(
    self, other, I(+(.(self, val), .(other, val)))
))),

# regular method call
=(a, I(2)),
=(b, @(a, +, I(4))),
assert_eq(.(b, val), 6),

# + is an operator
=(a, I(2)),
=(b, +(a, I(4))),
assert_eq(.(b, val), 6),
