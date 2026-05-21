# update the `stl_import_shadowing` test if the following line changes
import(range),

def(quicksort, seq, _(
    =(l, len(seq)),
    if(>=(l, 2), _(
        =(pivot_idx, /(l, 2)),
        =(pivot, index(seq, pivot_idx)),
        =(seq, remove_at(seq, pivot_idx)),
        =(left, list()),
        =(right, list()),
        for_in(seq, el, ifelse(
            <=(el, pivot),
            =(left, append(left, el)),
            =(right, append(right, el)),
        )),
        =(left, quicksort(left)),
        =(right, quicksort(right)),
        =(seq, extend(append(left, pivot), right)),
    )),
    seq
)),

# Returns whether the given sequence is sorted in ascending order.
def(is_sorted, seq, _(
    =(s, true),
    if(>=(len(seq), 2),
        for_in(range(0, -(len(seq), 1)), i, if(
            >(index(seq, i), index(seq, +(i, 1))),
            =(s, false)
        )),
    ),
    s
)),
