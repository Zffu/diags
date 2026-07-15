use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn parse_fmt_str(str: String) -> TokenStream {
    let mut inside = false;
    let mut current = String::new();
    let mut placeholders = vec![];

    for c in str.chars() {
        match c {
            '{' => {
                inside = true;
                current.clear();
            }

            '}' if inside => {
                inside = false;
                placeholders.push(current.clone());
            }

            _ if inside => {
                current.push(c);
            }

            _ => {}
        }
    }

    if placeholders.is_empty() {
        return quote! {
            #str
        }
        .into();
    }

    let mut generated = vec![];

    for placeholder in placeholders {
        let ident = format_ident!("{}", placeholder);

        generated.push(quote! {
            #ident = self.#ident
        })
    }

    quote! {
        format!(#str, #(#generated)*)
    }
    .into()
}
