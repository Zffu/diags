#[macro_export]
macro_rules! return_diag {
    ($t: expr) => {
        $t.into_diag().emit();
        return ();
    };
}
