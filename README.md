# Regulus
Regulus is a simple, interpreted language with very simple syntax and zero dependencies.

It is currently work in progress.

## Example
```
import(range),

# sorts the given list in ascending order using quicksort
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
```