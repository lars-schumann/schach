macro_rules! skip_if_no_expensive_test_opt_in {
    () => {
        if option_env!("SCHACH_EXPENSIVE_TEST_OPT_IN").is_none()
            && std::env::var("SCHACH_EXPENSIVE_TEST_OPT_IN").is_err()
        {
            println!(
                "skipped because this is expensive, set env var SCHACH_EXPENSIVE_TEST_OPT_IN to run this"
            );
            return;
        }
    };
}

pub(crate) use skip_if_no_expensive_test_opt_in;
