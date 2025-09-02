# Selects a branch of code to execute based on the given variable.
#
# The first argument must be the value to compare against; this is mandatory to pass.
#
# After that, any number of pairs of arguments (including none) can be passed.
# For such a pair, the first value will be compared against the initial first argument.
# If they are equal, the second argument of the pair will be executed, its value returned and
# `switch` ends.
#
# Additionally, a single last argument may be given which will be used as a fallback return
# value if all values above did not equal the initial first argument.
#
# If no such last argument is given and no value pair matched the initial argument, an
# exception is raised.
#
# Note that the first argument will only be evaluated once, so the following program will output
# 0 instead of 1:
#
# _(
#     =(x, 0)
#     switch(x,
#         # replace the value of `x` with 1
#         _(=(x, 1), 1), print(1),
#         0, print(0)
#     )
# )
def(switch, base, [$arms], _(
    =(was_found, false),
    while(>(len(arms), 0), _(
        ifelse(
            ==(len(arms), 1),
            # fallback,
            _(
                =(retval, index(arms, 0)),
                =(arms, list()),
                =(was_found, true),
            ),
            # new arm
            _(
                =(comp, index(arms, 0)),
                ifelse(
                    ==(base, comp()),
                    _(
                        =(retval, index(arms, 1)),
                        =(arms, list()),
                        =(was_found, true),
                    ),
                    _(
                        =(arms, remove_at(arms, 0)),
                        =(arms, remove_at(arms, 0)),
                    )
                )
            )
        )
    )),
    ifelse(
        was_found,
        retval(),
        error("Switch", "no `switch` arm matched and no fallback found")
    )
)),
