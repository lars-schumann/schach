#[doc(hidden)]
pub(crate) trait __ඬඞSeal {}

pub(crate) trait All: Sized + 'static + __ඬඞSeal {
    const ALL: &[Self];
}
#[macro_export]
macro_rules! impl_all {
    (
        $ty:path: [
            $($variant:path),+ $(,)?
        ]
    ) => {
        impl __ඬඞSeal for $ty {}

        impl All for $ty {
            const ALL: &[Self] = &[ $($variant),+ ];
        }

        impl $ty{
            #[doc(hidden)]
            #[allow(dead_code)]
            #[deny(unreachable_patterns)]
            const fn __impl_all_exhaustiveness_check(value: Self) {
                match value {
                    $($variant => (),)+
                }
            }
        }

    };
}
