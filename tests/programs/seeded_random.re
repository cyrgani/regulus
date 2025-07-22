# if this test fails because the rng implementation changes, update the values here accordingly
import(random),
import(range),

seed(42),
=(base, list(2, 16, null, "abc")),
=(output, list()),
for_in(..(0, 100), i, _(
    =(output, append(output, choose(base))),
)),
assert_eq(
    output,
    list(null, 16, null, 2, null, "abc", 16, null, 2, 16, "abc", "abc", "abc", 16, null, 16, 2, "abc", null, 16, null, 16, "abc", 16, 2, 16, 16, "abc", "abc", null, 16, 2, "abc", 2, null, 16, 2, null, null, "abc", 2, null, 16, "abc", 16, 16, 16, "abc", 16, "abc", null, null, 16, "abc", 2, 2, null, 2, "abc", 2, 16, "abc", 2, 2, "abc", 16, 2, 16, 16, null, 2, "abc", 16, 2, 2, null, 16, 2, 16, null, null, null, 16, 2, null, 2, null, 2, 2, "abc", null, null, 16, 16, null, 2, 16, null, "abc", 16),
),
for_in(..(0, 100), i, _(
    assert_eq(randrange(0, 1), 0),
    assert_eq(randrange(-2, -1), -2),
)),
assert_eq(catch(randrange(5, 5)), "RangeError: called randrange with an empty range"),
assert_eq(catch(randrange(-1, -1)), "RangeError: called randrange with an empty range"),
assert_eq(catch(randrange(-3, -5)), "RangeError: called randrange with an empty range"),

assert_eq(choose("Hello, world!"), "d"),
assert_eq(choose("Hello, world!"), ","),
assert_eq(catch(choose(list())), "RangeError: called randrange with an empty range"),

=(shuffled_alphabet, "YLIEPKRQBFWVNHSAMCZJXTUDGO"),

assert_eq(shuffle("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), shuffled_alphabet),
assert_eq(len(shuffled_alphabet), 26),
assert_eq(shuffle(""), ""),
assert_eq(shuffle(list()), list())
