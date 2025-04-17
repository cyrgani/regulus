# Applies the second argument function to each element of the first argument list and returns
# the updated list.
def(map, seq, function, _(
    =(new_list, list()),
    for_in(seq, el, _(
        =(new_list, append(new_list, function(el)))
    )),
    new_list
)),
