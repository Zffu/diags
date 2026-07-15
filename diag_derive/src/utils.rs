use syn::{Attribute, Expr, Lit, Meta};

pub(crate) fn get_attr_val(attr: &Attribute) -> String {
    match &attr.meta {
        Meta::NameValue(nv) => {
            if let Expr::Lit(expr) = &nv.value {
                if let Lit::Str(s) = &expr.lit {
                    return s.value();
                }
            }

            panic!("Not a correct value")
        }

        _ => panic!("Not a correct value"),
    }
}
