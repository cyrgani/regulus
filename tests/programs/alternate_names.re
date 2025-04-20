_(
    import(aliases),
    run(
        assign(x, -(5, 3)),
        assert_eq(+(2, x), 4),
    )
)
