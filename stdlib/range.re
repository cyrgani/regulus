# Returns a list of all integers from `start` to `end`, including `start` but not `end`.
# Rust equivalent: `start..end`
def(range, start, end, _(
    if(>(start, end), error("cannot construct range with start > end")),
    =(i, start),
    =(l, list()),
    while(<(i, end), _(
        =(l, append(l, i)),
        =(i, +(i, 1)),
    )),
    l
)),

# TODO: make a decision which syntax is better
#=(range, fn(start, end, _(
#    if(>(start, end), error("cannot construct range with start > end")),
#    =(i, start),
#    =(l, list()),
#    while(<(i, end), _(
#        =(l, append(l, i)),
#        =(i, +(i, 1)),
#    )),
#    l
#))),

=(.., range)