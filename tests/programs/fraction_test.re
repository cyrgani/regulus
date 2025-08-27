import(fraction),

=(x, Fraction(2, 1)),
assert_eq(@(x, to_int), 2),
assert_eq(@(Fraction(3, 2), to_int), 1),
assert_eq(@(Fraction(5, 2), to_int), 2),
assert_eq(@(Fraction(-3, 2), to_int), -1),
assert_eq(@(Fraction(-1, 2), to_int), 0),
__builtin_print_catch(@(Fraction(0, 0), to_int)),

=(a, Fraction(2, 3)),
=(b, Fraction(4, 7)),

assert_eq(@(a, neg), Fraction(-2, 3)),
assert_eq(+(a, b), Fraction(26, 21)),
assert_eq(-(a, b), Fraction(2, 21)),
assert_eq(*(a, b), Fraction(8, 21)),
assert_eq(/(a, b), Fraction(14, 12)),
assert_eq(Fraction(4, 2), Fraction(6, 3)),
assert_eq(Fraction(3, 2), Fraction(-6, -4)),

# TODO: add far more tests, division by zero, overflow, all other functions in the module ...
