import(fraction),

=(x, Fraction(2, 1)),
assert_eq(fraction_to_int(x), 2),
assert_eq(fraction_to_int(Fraction(3, 2)), 1),
assert_eq(fraction_to_int(Fraction(5, 2)), 2),
assert_eq(fraction_to_int(Fraction(-3, 2)), -1),
assert_eq(fraction_to_int(Fraction(-1, 2)), 0),
assert_eq(catch(fraction_to_int(Fraction(0, 0))), "OverflowError: overflow occured during /!"),
