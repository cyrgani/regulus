# update the `stl_import_shadowing` test if the following line changes
import(range),

# reference implementation:
# fn quicksort<T: PartialOrd + Copy>(mut seq: Vec<T>) -> Vec<T> {
#     if seq.len() >= 2 {
#         let pivot_idx = seq.len() / 2;
#         let pivot = seq[pivot_idx];
#         let mut left = Vec::new();
#         let mut right = Vec::new();
#         for i in 0..seq.len() {
#             if pivot_idx != i {
#                 let el = seq[i];
#                 if el <= pivot {
#                     left.push(el);
#                 } else {
#                     right.push(el);
#                 }
#             }
#         }
#         left = quicksort(left);
#         right = quicksort(right);
#         let step = left.len();
#         for i in 0..step {
#             seq[i] = left[i];
#         }
#         seq[step] = pivot;
#         for i in 0..right.len() {
#             seq[i + 1 + step] = right[i];
#         }
#     }
#     seq
# }
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

# Returns whether the given sequence is sorted in ascending order.
def(is_sorted, seq, _(
    =(s, true),
    if(>=(len(seq), 2),
        for_in(range(0, -(len(seq), 1)), i, _(
            if(
                >(index(seq, i), index(seq, +(i, 1))),
                =(s, false)
            )
        )),
    ),
    s
)),
