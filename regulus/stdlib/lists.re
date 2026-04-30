import(range),

# Constructs a new list containing all the given arguments.
def(list, [elements], _(
    =(l, __builtin_list_api(0)),
    for_in(elements, el, =(l, append(l, el))),
    l
)),

# Returns the length of the given list.
def(len, l, __builtin_list_api(1, l)),

# Returns the value in the first list argument at the second integer argument.
# Raises an exception if the index is out of bounds.
def(index, l, idx, __builtin_list_api(2, l, idx)),

# Removes the element at the given list index.
# The first argument is the list, the second the index.
# If the index is out of bounds, an exception is raised.
# Returns the updated list.
def(remove_at, l, idx, __builtin_list_api(3, l, idx)),

# Insert a value at an index into a list.
# Argument order: list, index, element.
# The index must be positive and not larger than the length of the list.
# That means that inserting at exactly `len(list)` is allowed.
def(insert, l, idx, elem, __builtin_list_api(4, l, idx, elem)),

# Appends the second argument at the back of the list given as first argument and returns
# the new list.
def(append, l, elem, insert(l, len(l), elem)),

# Replaces an element at a list index with another and returns the updated list.
# The first argument is the list, the second the index and the third the new value.
# If the index is out of bounds, an exception is raised.
def(replace_at, l, idx, elem, _(
    =(l, remove_at(l, idx)),
    =(l, insert(l, idx, elem)),
    l
)),

# Applies the second argument function to each element of the first argument list and returns
# the updated list.
def(map, seq, function, _(
    =(new_list, list()),
    for_in(seq, el, _(
        =(new_list, append(new_list, function(el)))
    )),
    new_list
)),

# Returns the first element of the given list, raising an exception if it is empty.
def(first, seq, _(
    index(seq, 0),
)),

# Returns the last element of the given list, raising an exception if it is empty.
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

# Swaps the values at two indices of a list and returns the new sequence.
# The arguments are: list, first index, second index.
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

# Reverses a list.
def(reverse, seq, _(
    =(l, len(seq)),
    for_in(
        range(0, /(l, 2)),
        i,
        =(seq, swap(seq, i, -(-(l, 1), i)))
    ),
    seq
)),
