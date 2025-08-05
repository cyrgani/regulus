_(
    import(random),
    not_random(),
    print("expected error:", run_or_string_exception(random_u16())),
)