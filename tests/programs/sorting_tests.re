import(sorting),
assert_eq(
    quicksort(list(7, 1, 4, 2, 6, 1, 2, 5, 8, 1)),
    list(1, 1, 1, 2, 2, 4, 5, 6, 7, 8)
),
assert_eq(quicksort(list()), list()),
assert_eq(quicksort(list(1)), list(1)),
assert_eq(quicksort(list(2, 1)), list(1, 2)),

import(random),
assert_eq(quicksort(shuffle(range(0, 1000))), range(0, 1000)),

assert(is_sorted(range(0, 300))),
assert(is_sorted(list())),
assert(is_sorted(list(1))),
assert(is_sorted(list(1, 2))),
assert(is_sorted(list(2, 2, 2, 2))),
assert(!(is_sorted(list(2, 1)))),
assert(is_sorted(quicksort(list(1, 4, 6, 23, 1, 4, 6, 1, 0, -1, 6, 2))))
