=(catch, run_or_string_exception),
assert_eq(catch(2), 2),
assert_eq(
    _(
        catch(/(1, 0)),
        catch(catch(catch("foo")))
    ),
    "foo"
)