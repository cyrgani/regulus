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
        =(left, list()),
        =(right, list()),
        for_in(range(0, l), i, _(
            if(!=(pivot_idx, i), _(
                =(el, index(seq, i)),
                ifelse(
                    <=(el, pivot),
                    =(left, append(left, el)),
                    =(right, append(right, el)),
                )
            ))
        )),
        =(left, quicksort(left)),
        =(right, quicksort(right)),
        =(step, len(left)),
        for_in(
            range(0, step),
            i,
            =(seq, replace_at(seq, i, index(left, i)))
        ),
        =(seq, replace_at(seq, step, pivot)),
        for_in(
            range(0, len(right)),
            i,
            =(seq, replace_at(
                seq,
                +(+(i, 1), step),
                index(right, i)
            ))
        ),
    )),
    seq
)),


```