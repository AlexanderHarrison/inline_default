/// Derive Default within struct definition.
/// See README for details.
#[macro_export]
macro_rules! inline_default {
    (
        $(
            $(#[$s_at:meta])* // attributes
            $s_v:vis struct $s_nm:ident //struct name, vis
            $(< $($s_lt:lifetime,)* $( $s_gen:ident $(: $s_tr:ident)? ),* $(,)? >)? // generics
            { $( $(#[$f_at:meta])* $f_v:vis $f_nm:ident : $f_ty:ty $(= $def:expr)? ),* $(,)? } // definition
        )*
    ) => {
        $(
            $(#[$s_at])*
            $s_v struct $s_nm $(< $($s_lt ,)* $($s_gen $(: $s_tr)? ),* >)? {
                $( $(#[$f_at])* $f_v $f_nm : $f_ty ),*
            }

            macro_rules! __default_expand {
                ($t:ty, $d:expr) => { $d };
                ($t:ty,) => { <$t as Default>::default() }
            }

            impl $(< $($s_lt ,)* $($s_gen $(: $s_tr)?),* >)? Default for $s_nm $(< $($s_lt ,)* $($s_gen),* >)? {
                fn default() -> Self {
                    $(let $f_nm = __default_expand!( $f_ty, $( $def )? );)*
                    Self { $($f_nm,)* }
                }
            }
        )*
    }
}
