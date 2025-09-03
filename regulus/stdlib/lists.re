# TODO: many things here can operate on list and string. consider moving those into `seq.re`, `sequence.re` or similar.
# TODO: add tests for each version that test both string and list.
# note: `null` is put here to avoid merging the above comment with the doc comment below.
null,

import(range),

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
    =(seq, replace_at(seq, idx2, e1)),
    =(seq, replace_at(seq, idx1, e2)),
    seq
)),

# Takes a list and appends all elements of another list to it.
def(extend, list, added_list, _(
    for_in(added_list, el, =(list, append(list, el))),
    list
)),

# Flattens a list of lists into a list.
# Example: [[1, 2], [3], [[4, 5]]] -> [1, 2, 3, [4, 5]].
def(flatten, seq, _(
    =(new, list()),
    for_in(seq, list_el, =(new, extend(new, list_el))),
    new
)),

# Reverses a string or list.
def(reverse, seq, _(
    =(l, len(seq)),
    for_in(
        range(0, /(l, 2)),
        i,
        =(seq, swap(seq, i, -(-(l, 1), i)))
    ),
    seq
)),
