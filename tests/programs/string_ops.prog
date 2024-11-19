_(
    import("collect_list_into_string"),
    assert_eq("asdf", strconcat("as", "d", "f")),
    assert_eq("1nullhello, world
true", collect_list_into_string(list(1, null, "hello, world
", true))),
    assert_eq("", strconcat()),
    assert_eq("X", strconcat("X")),
    assert_eq("Y", strconcat("", "Y", "")),
)