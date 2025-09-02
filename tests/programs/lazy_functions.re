def(ignore, $a, null),
ignore(print("hidden")),

def(use_twice_without_calling, $a, _(
    a, a, null
)),
use_twice_without_calling(print("also hidden")),

def(use_once, $a, a()),
use_once(print("once #1")),

# TODO: This should only cause `a` to be evaluated once, not twice.
def(use_twice, $a, _(
    a(), a(), null
)),
use_twice(print("once #2")),
