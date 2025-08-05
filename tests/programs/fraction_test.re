import(fraction),

=(x, Fraction(2, 1)),
assert_eq(frac_to_int(x), 2),
assert_eq(frac_to_int(Fraction(3, 2)), 1),
assert_eq(frac_to_int(Fraction(5, 2)), 2),
assert_eq(frac_to_int(Fraction(-3, 2)), -1),
assert_eq(frac_to_int(Fraction(-1, 2)), 0),
assert_eq(catch(frac_to_int(Fraction(0, 0))), "DivideByZeroError: attempted to divide by zero"),

=(a, Fraction(2, 3)),
=(b, Fraction(4, 7)),

# TODO: fractions should be compared with `frac_assert_eq`, otherwise `1/2` != `2/4`
def(frac_assert_eq, f1, f2, _(
    # TODO: frac_compare is not yet implemented
    assert_eq(frac_compare(f1, f2), 0)
)),

assert_eq(frac_neg(a), Fraction(-2, 3)),
assert_eq(frac_add(a, b), Fraction(26, 21)),
assert_eq(frac_sub(a, b), Fraction(2, 21)),
assert_eq(frac_mul(a, b), Fraction(8, 21)),
assert_eq(frac_div(a, b), Fraction(14, 12)),

# TODO: add far more tests, division by zero, overflow, all other functions in the module ...
