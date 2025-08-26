import(lists),
=(data, list(2, 4, 5)),
assert_eq(len(data), 3),
assert_eq(data, list(2, 4, 5)),
assert(!(==(data, list(4, 2, 5)))),
assert_eq(index(data, 1), 4),
for_in(data, x, print(x)),
def(halve, el, /(el, 2)),
assert_eq(list(1, 5, 7), map(list(2, 10, 14), halve)),

assert_eq(len("abc"), len(list(1, 2, 3))),

assert_eq(overwrite_at_index(list("a", "b", "c"), 2, "b"), list("a", "b", "b")),
assert_eq(overwrite_at_index("abc", 2, "b"), "abb"),

assert_eq(swap("abc", 1, 2), "acb"),
assert_eq(swap(list(1, null), 0, 0), list(1, null)),

for_in(list(0, 1, 2, 3), i, print(i)),
assert_eq(for_in(list(0, 1, 2), i, 0), null),

assert_eq(list(2, 4, 6), filter(list(1, 2, 3, 4, 6), fn(el, ==(%(el, 2), 0))))
