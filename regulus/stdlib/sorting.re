# update the `stl_import_shadowing` test if the following line changes
import(range),

def(quicksort, seq, _(
    =(l, len(seq)),
    ifelse(<(l, 2), seq, _(
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
        extend(append(left, pivot), right),
    )),
)),

# Returns whether the given sequence is sorted in ascending order.
def(is_sorted, seq, _(
    ifelse(<(len(seq), 2), true, _(
        =(s, true),
        =(i, 0),
        =(l, -(len(seq), 1)),
        =(cur, first(seq)),
        while(&&(s, !=(i, l)), _(
            =(i, +(i, 1)),
            =(next, index(seq, i)),
            ifelse(
                >(cur, next),
                =(s, false),
                =(cur, next),
            ),
        )),
        s
    )),
)),
