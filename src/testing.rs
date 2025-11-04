pub(crate) const EXPENSIVE_TESTS_ENV_VAR: &str = "SCHACH_EXPENSIVE_TESTS";

macro_rules! bail_if_no_expensive_test_opt_in {
    () => {
        if option_env!("SCHACH_EXPENSIVE_TESTS").is_none() {
            println!(
                "skipped because this is expensive, set env var {} to run this",
                crate::testing::EXPENSIVE_TESTS_ENV_VAR
            );
            return;
        }
    };
}

pub(crate) use bail_if_no_expensive_test_opt_in;
