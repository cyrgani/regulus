# allowed
assert_eq("2", string(2)),
assert_eq("2", string("2")),
assert_eq(2, int("2")),
assert_eq(2, int(string(2))),
assert_eq(true, bool(1)),
assert_eq(string(int(bool(int("2")))), "1"),

# disallowed
assert_eq(catch(int("a")), "programs/casting.re:10:21: TypeError: Unable to cast a to int"),
