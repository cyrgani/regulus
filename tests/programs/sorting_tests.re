import(sorting),
assert_eq(
    bubblesort(list(7, 1, 4, 2, 6, 1, 2, 5, 8, 1)),
    list(1, 1, 1, 2, 2, 4, 5, 6, 7, 8)
),
assert_eq(bubblesort(list()), list()),
assert_eq(bubblesort(list(1)), list(1)),
assert_eq(bubblesort(list(2, 1)), list(1, 2)),

import(random),
assert_eq(bubblesort(shuffle(range(0, 400))), range(0, 400)),
