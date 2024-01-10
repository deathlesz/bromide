use proc_macro::TokenStream;

use darling::FromVariant;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Fields, parse_macro_input, spanned::Spanned as _};

#[derive(Default, FromVariant)]
#[darling(default, attributes(response))]
struct Attributes {
    status_code: Option<u16>,
    error_code: Option<String>,
}

#[proc_macro_derive(ResponseError, attributes(response))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident: name, data, generics, .. } = parse_macro_input!(input);

    let impl_block;
    let mut match_expr;

    match data {
        Data::Enum(enum_data) => {
            match_expr = TokenStream2::new();

            for variant in enum_data.variants {
                let variant_name = &variant.ident;

                let fields = match variant.fields {
                    Fields::Unit => quote_spanned!( variant.span()=> ),
                    Fields::Unnamed(_) => quote_spanned!( variant.span()=> (..)),
                    Fields::Named(_) => quote_spanned!( variant.span()=> {..})
                };

                let attributes = Attributes::from_variant(&variant).unwrap_or_default();

                let response = match (attributes.status_code, attributes.error_code) {
                    (None, None) =>
                        quote_spanned!( variant.span()=> "-1".into_response()),
                    (None, Some(error_code)) =>
                        quote_spanned!( variant.span()=> #error_code.into_response()),
                    (Some(status_code), None) =>
                        quote_spanned!( variant.span()=> axum::http::StatusCode::from_u16(#status_code).expect("invalid status_code").into_response()),
                    (Some(status_code), Some(error_code)) =>
                        quote_spanned!( variant.span()=> (axum::http::StatusCode::from_u16(#status_code).expect("invalid status_code"), #error_code).into_response()),
                };

                match_expr.extend(quote_spanned! { variant.span()=>
                    Self::#variant_name #fields => #response,
                });
            }

            impl_block = quote! {
                fn into_response(self) -> axum::response::Response {
                    match self {
                        #match_expr
                    }
                }
            };
        }
        _ => return syn::Error::new(proc_macro2::Span::call_site(), "ResponseError should be derived only on enums")
            .to_compile_error()
            .into()
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl IntoResponse for #impl_generics #name #type_generics #where_clause {
            #impl_block
        }
    };

    TokenStream::from(expanded)
}
