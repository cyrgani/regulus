# Applies the second argument function to each element of the first argument list and returns
# the updated list.
def(map, seq, function, _(
    =(new_list, list()),
    for_in(seq, el, _(
        =(new_list, append(new_list, function(el)))
    )),
    new_list
)),

# Returns the first element of the given list or string, raising an exception if it is empty.
def(first, seq, _(
    index(seq, 0),
)),

# Returns the last element of the given list or string, raising an exception if it is empty.
def(last, seq, _(
    index(seq, -(len(seq), 1)),
)),
