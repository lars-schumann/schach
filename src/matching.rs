macro_rules! perfect {
    (   [$L_TY:ty : $R_TY:ty],
        $L_TO_R_NAME:ident,
        $L_FROM_R_NAME:ident,
        $(
            $l:path = $r:path,
        )+
    ) => {
        impl $L_TY{
            const fn $L_TO_R_NAME(self) -> $R_TY{
                match self {
                    $(
                        $l => $r,
                    )+
                }
            }

            const fn $L_FROM_R_NAME(value: $R_TY) -> Self{
                match value {
                $(
                    $r => $l,
                )+
            }
            }
        }
    };
}

macro_rules! imperfect {
    (   [$L_TY:ty => $R_TY:ty],
        $L_TO_R_NAME:ident,
        $L_FROM_R_NAME:ident,
        $(
            $l:path = $r:path,
        )+
    ) => {
        impl $L_TY{
            const fn $L_TO_R_NAME(self) -> $R_TY{
                match self {
                    $(
                        $l => $r,
                    )+
                }
            }

            const fn $L_FROM_R_NAME(value: $R_TY) -> Option<Self>{
                match value {
                $(
                    $r => Some($l),
                )+
                _ => None,
            }
            }
        }
    };
}

pub(crate) use imperfect;
pub(crate) use perfect;
