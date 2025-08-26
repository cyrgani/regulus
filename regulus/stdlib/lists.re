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

# Returns a new list of all the elements of the first argument list where the second argument 
# function returned `true` when called with the element as its only argument.
def(filter, seq, function, _(
    =(new_list, list()),
    for_in(seq, el, _(
        if(
            function(el), 
            =(new_list, append(new_list, el))
        )
    )),
    new_list
)),

# Swaps the values at two indices of a list or string and returns the new sequence.
# The arguments are: list or string, first index, second index.
#
# The indices may be equal, in which case the returned sequence will not be changed.
# If the indices are out of bounds or invalid, an exception is raised.
def(swap, seq, idx1, idx2, _(
    =(e1, index(seq, idx1)),
    =(e2, index(seq, idx2)),
    =(seq, overwrite_at_index(seq, idx2, e1)),
    =(seq, overwrite_at_index(seq, idx1, e2)),
    seq
))
