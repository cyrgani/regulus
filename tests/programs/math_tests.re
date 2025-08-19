assert_eq(%(5, 3), 2),
assert_eq(%(3, 5), 3),

import(math),

assert_eq(abs(-1), abs(1)),
assert_eq(abs(-0), abs(0)),
assert_eq(abs(-2), 2),
assert_eq(abs(-9223372036854775807), 9223372036854775807),
__builtin_print_catch(abs(-9223372036854775808)),

assert_eq(gcd(5, 1), 1),
assert_eq(gcd(5, 3), 1),
assert_eq(gcd(16, 4), 4),
assert_eq(gcd(13, 7), 1),
assert_eq(gcd(30, 12), 6),
assert_eq(gcd(12, 30), 6),
__builtin_print_catch(gcd(0, 8)),
__builtin_print_catch(gcd(8, 0)),
__builtin_print_catch(gcd(0, 0)),
assert_eq(gcd(8, 8), 8),
# TODO: test negative numbers
