# this just exists because a user may have their own file `prelude.re`, which should not be imported as the prelude.
# a user who has a file `__builtin_prelude.re` is much less likely and it is also a reserved name by convention
import(prelude),
