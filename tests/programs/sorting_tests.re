import(sorting),
assert_eq(
    quicksort(list(7, 1, 4, 2, 6, 1, 2, 5, 8, 1)),
    list(1, 1, 1, 2, 2, 4, 5, 6, 7, 8)
),
assert_eq(quicksort(list()), list()),
assert_eq(quicksort(list(1)), list(1)),
assert_eq(quicksort(list(2, 1)), list(1, 2)),

import(random),
assert_eq(quicksort(shuffle(range(0, 400))), range(0, 400)),
