# TODO: spans are quite broken
import(lists),
=(l, list(2, 5, 6, "a", list(2, 4))),
assert_eq(first(l), 2),
assert_eq(last(l), list(2, 4)),
assert_eq(catch(last("")), "IndexError: invalid list index: out of range integral type conversion attempted"),
assert_eq(catch(first(list())), "programs/first_last.re:8:9: IndexError: sequence index out of bounds"),
assert_eq(catch(last(list())), "IndexError: invalid list index: out of range integral type conversion attempted"),
assert_eq(catch(first("")), "programs/first_last.re:8:9: IndexError: sequence index out of bounds"),
assert_eq(last("a"), first("a")),
assert_eq(last("a"), "a"),
