# Regulus
Regulus is a simple, interpreted language with very simple syntax and zero dependencies.

It is currently work in progress.

## Example
```
import(range),

# sorts the given list in ascending order using bubblesort
def(bubblesort, seq, _(
    =(l, len(seq)),
    for_in(range(0, l), i, _(
        for_in(range(i, l), j, _(
            =(i_val, index(seq, i)),
            =(j_val, index(seq, j)),
            if(>(i_val, j_val), _(
                =(seq, replace_at(seq, i, j_val)),
                =(seq, replace_at(seq, j, i_val)),
            ))
        ))
    )),
    seq
))

```