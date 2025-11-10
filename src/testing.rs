macro_rules! env_var {
    () => {
        "SCHACH_EXPENSIVE_TEST_OPT_IN"
    };
}

macro_rules! skip_if_no_expensive_test_opt_in {
    () => {
        if option_env!(crate::testing::env_var!()).is_none()
            && std::env::var(crate::testing::env_var!()).is_err()
        {
            println!(
                "skipped because this is expensive, set env var {} to run this",
                crate::testing::env_var!()
            );
            return;
        }
    };
}

pub(crate) use env_var;
pub(crate) use skip_if_no_expensive_test_opt_in;
