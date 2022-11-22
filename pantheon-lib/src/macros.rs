#[macro_export]
macro_rules! ops {
    ($char_name:ident; $( $(!$keyword:ident)? $var:ident $rank:expr => $op:expr;)*) => {
        $($char_name.$var.register($rank, Box::new(
            $($keyword)? |$var: &mut _, ops_macro_character: &Character| {
                pantheon::reexports::macros::expand_carriers!($op);
                Ok(())
            }
        )));*
    }
}

#[macro_export]
macro_rules! eval {
    ($char_name:ident.$var:ident) => {
        &*$char_name.$var.evaluate(&$char_name)?
    };
}
