# formerly in the stl as `catch`, then `run_or_string_exception`
def(catch, $x, try_except(x(), e, e)),

assert_eq(catch(2), 2),
assert_eq(
    _(
        catch(/(1, 0)),
        catch(catch(catch("foo")))
    ),
    "foo"
)