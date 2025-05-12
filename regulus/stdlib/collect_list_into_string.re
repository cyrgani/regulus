# Converts a list of elements into a single string without separators, casting them if necessary.
# TODO maybe deprecated
def(collect_list_into_string, seq, _(
    =(final_string, ""),
    for_in(seq, el, =(final_string, strconcat(final_string, string(el)))),
    final_string
))
