# Concatenates any number of values into one string and returns it.
# Arguments are casted to strings before concatenating.
def(strconcat, [args], _(
    =(s, ""),
    for_in(args, arg, =(s, +(s, string(arg)))),
    s
)),