use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::struct_def::WidgetCreation;

mod struct_def;

#[proc_macro]
pub fn widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as WidgetCreation);

    (quote! {
        #input
    }).into()
}

#[proc_macro_derive(
    OptionWrap
)]
pub fn wrap(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;

    (quote! {
        impl #ident {
            pub fn wrap(self) -> Option<Self>
            where
                Self: Sized 
            {
                Some(self)
            }
        }
    }).into()
}