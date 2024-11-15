use tests_generator_macro::make_tests;

mod utils;

pub const OVERWRITE_STREAM_FILES: Option<&'static str> = option_env!("OVERWRITE_STREAM_FILES");

make_tests! {}
