import(range),
assert_eq(range(0, 4), list(0, 1, 2, 3)),
assert_eq(range(0, 1), list(0)),
assert_eq(range(-2, 2), list(-2, -1, 0, 1)),
assert_eq(range(0, 0), list()),
assert_eq(catch(range(1, 0)), "RangeError: cannot construct range with start > end"),
assert_eq(catch(range(1, -1)), "RangeError: cannot construct range with start > end"),

assert_eq(..(0, 4), list(0, 1, 2, 3)),
