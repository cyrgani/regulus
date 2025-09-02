# Constructs a new list containing all the given arguments.
def(list, [elements], _(
    =(l, __builtin_new_list()),
    for_in(elements, el, =(l, append(l, el))),
    l
)) 
