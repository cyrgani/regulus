import("range"),

# reference implementation:
# fn bubblesort<T: PartialOrd + Copy>(mut seq: Vec<T>) -> Vec<T> {
#     for i in 0..seq.len() {
#         for j in i..seq.len() {
#             if seq[i] > seq[j] {
#                 let tmp = seq[i];
#                 seq[i] = seq[j];
#                 seq[j] = tmp;
#             }
#         }
#     }
#     seq
# }
def(bubblesort, seq, _(
    =(l, len(seq)),
    for_in(range(0, l), i, _(
        for_in(range(i, l), j, _(
            =(i_val, index(seq, i)),
            =(j_val, index(seq, j)),
            if(>(i_val, j_val), _(
                =(seq, overwrite_at_index(seq, i, j_val)),
                =(seq, overwrite_at_index(seq, j, i_val)),
            ))
        ))
    )),
    seq
))
