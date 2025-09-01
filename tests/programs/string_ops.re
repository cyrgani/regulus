_(
    # Converts a list of elements into a single string without separators, casting them if necessary.
    # (moved from the stl to this test)
    def(collect_list_into_string, seq, _(
        =(final_string, ""),
        for_in(seq, el, =(final_string, strconcat(final_string, string(el)))),
        final_string
    )),
    assert_eq("asdf", strconcat("as", "d", "f")),
    assert_eq("1nullhello, world
true", collect_list_into_string(list(1, null, "hello, world
", true))),
    assert_eq("", strconcat()),
    assert_eq("X", strconcat("X")),
    assert_eq("Y", strconcat("", "Y", "")),
)