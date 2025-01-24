use std::iter::Map;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Expr, Ident, Token};
use syn::parse::{Parse, ParseStream};
use syn::token::Token;

mod keyword_and_punctuation {
    use syn::{custom_keyword, custom_punctuation};

    custom_keyword!(def);
    custom_keyword!(end);
}

#[derive(Clone)]
pub struct WidgetCreation {
    widget_identifier: Ident,
    props: Vec<WidgetProperty>
}

pub trait ParseZeroOrMore {
    fn parse_zero_or_more<T>(&self) -> Vec<T> where T: Parse + Clone;
}

impl ParseZeroOrMore for ParseStream<'_> {
    fn parse_zero_or_more<T>(&self) -> Vec<T>
    where
        T: Parse + Clone
    {
        let mut result_vec = Vec::new();

        while let Ok(item) = &self.parse::<T>() {
            result_vec.push(item.clone());
        }

        result_vec
    }
}

impl Parse for WidgetCreation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![@]>()?;

        let widget_name = input.parse::<Ident>()?;
        _ = input.parse::<keyword_and_punctuation::def>()?;

        let props = input.parse_zero_or_more::<WidgetProperty>();

        Ok(WidgetCreation {
            widget_identifier: widget_name,
            props
        })
    }
}

#[derive(Clone)]
struct WidgetProperty {
    ident: Ident,
    value: Expr,
}

impl Parse for WidgetProperty {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse(input)?;
        _ = input.parse::<Token![=]>()?;
        let val = Expr::parse(input)?;
        _ = input.parse::<Token![;]>()?;

        Ok(WidgetProperty { ident, value: val })
    }
}

impl ToTokens for WidgetCreation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let props_res = &self.props;
        let ident = &self.widget_identifier;


        let res = quote! {
            #ident {
                #(#props_res),*,
                ..Default::default()
            }.build()
        };

        tokens.append_all(res);
    }
}


impl ToTokens for WidgetProperty {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let value = &self.value;

        let res = quote! {
            #ident: #value
        };

        tokens.append_all(res);
    }
}

