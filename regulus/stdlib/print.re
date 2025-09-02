import(range),

# Evaluates all given arguments and prints them to stdout.
# All arguments are separated with a single space.
# No trailing space is added after the last element.
# After all arguments have been printed, a newline is also printed.
# Returns `null`.
#
# If you need more precise control over the output, use `write` instead.
def(print, [args], _(
    =(l, len(args)),
    for_in(range(0, l), i, _(
        write(index(args, i)),
        if(!=(i, -(l, 1)), write(" "))
    )),
    write(endl()),
))
