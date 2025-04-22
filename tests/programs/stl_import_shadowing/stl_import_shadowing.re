# previously, there was a bug where `import` would have resolved the `import(range)` in `stl/sorting.re` to the
# local `range.re` rather than the intended `stl/range.re`
import(sorting),
assert_eq(bubblesort(list(1, 4, 2)), list(1, 2, 4)),