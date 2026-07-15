use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

use crate::{fmt::parse_fmt_str, utils::get_attr_val};

mod fmt;
mod utils;

#[proc_macro_derive(Diagnostic, attributes(message, primary_span, note, span, help))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let mut message_str = quote! {}.into();

    let mut notes = vec![];
    let mut help = vec![];

    let mut builder = vec![];

    for attribute in input.attrs {
        if attribute.path().is_ident("note") {
            notes.push(parse_fmt_str(get_attr_val(&attribute)));
        }

        if attribute.path().is_ident("help") {
            help.push(parse_fmt_str(get_attr_val(&attribute)));
        }

        if attribute.path().is_ident("message") {
            message_str = parse_fmt_str(get_attr_val(&attribute));
        }
    }

    let fields = match input.data {
        Data::Struct(st) => st.fields,
        _ => panic!(),
    };

    for field in fields {
        let ident = field.ident.unwrap();

        let mut note = None;

        for attr in field.attrs {
            if attr.path().is_ident("note") {
                note = Some(parse_fmt_str(get_attr_val(&attr)));
            }

            let span = if note.is_none() {
                quote! { self.#ident }
            } else {
                let note = note.clone().unwrap();
                quote! { self.#ident.label(#note) }
            };

            if attr.path().is_ident("primary_span") {
                builder.push(quote! {
                    .primary_span(#span)
                });

                note = None;
            }

            if attr.path().is_ident("span") {
                builder.push(quote! {
                    .span(#span)
                });

                note = None;
            }
        }
    }

    for note in notes {
        builder.push(quote! {
            .note(#note)
        })
    }

    for help in help {
        builder.push(quote! {
            .help(#help)
        })
    }

    quote! {
        impl IntoDiagnostic for #name {
            fn into(self) -> Diagnostic {
                Diagnostic::new(diags::DiagnosticCode::new(diags::Level::Error, 0), #message_str)#(#builder)*
            }
        }
    }
    .into()
}
