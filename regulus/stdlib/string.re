# Concatenates any number of strings into one and returns it.
# Other values are not implicitly casted and cause an exception.
# TODO: add a version that does this implicit casting
def(strconcat, [args], _(
    =(s, ""),
    for_in(args, arg, =(s, +(s, arg))),
    s
)),