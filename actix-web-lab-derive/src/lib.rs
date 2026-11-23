//! Experimental macros for Actix Web.

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, parse_macro_input, punctuated::Punctuated, token::Comma};

fn is_copy_from_app_data_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("from_request")
        && attr
            .parse_args::<Ident>()
            .is_ok_and(|ident| ident == "copy_from_app_data")
}

struct ParsedField<'a> {
    ident: &'a Ident,
    ty: &'a syn::Type,
    val_var: Ident,
    fut_var: Ident,
    copy_from_app_data: bool,
}

/// Derive a `FromRequest` implementation for an aggregate struct extractor.
///
/// All fields of the struct need to implement `FromRequest` unless they are marked with annotations
/// that declare different handling is required.
///
/// # Examples
/// ```
/// use actix_web::{Responder, get, http, web};
/// use actix_web_lab::FromRequest;
///
/// #[derive(Debug, FromRequest)]
/// struct RequestParts {
///     // the FromRequest impl is used for these fields
///     method: http::Method,
///     pool: web::Data<u32>,
///     req_body: String,
///
///     // equivalent to `req.app_data::<u64>().copied()`
///     #[from_request(copy_from_app_data)]
///     int: u64,
/// }
///
/// #[get("/")]
/// async fn handler(parts: RequestParts) -> impl Responder {
///     // ...
///     # ""
/// }
/// ```
#[proc_macro_derive(FromRequest, attributes(from_request))]
pub fn derive_from_request(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let data = match input.data {
        syn::Data::Struct(data) => data,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            return quote! {
                compile_error!("Deriving FromRequest is only supported on structs for now.");
            }
            .into();
        }
    };

    let fields = match data.fields {
        syn::Fields::Named(fields) => fields.named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => {
            return quote! {
                compile_error!("Deriving FromRequest is only supported on structs with named fields for now.");
            }
            .into();
        }
    };

    let mut parsed_fields = Vec::with_capacity(fields.len());

    for (idx, field) in fields.iter().enumerate() {
        let mut copy_from_app_data = false;

        for attr in &field.attrs {
            if attr.path().is_ident("from_request") {
                if !is_copy_from_app_data_attr(attr) {
                    return syn::Error::new_spanned(
                        attr,
                        "invalid `from_request` attribute; expected `copy_from_app_data`",
                    )
                    .to_compile_error()
                    .into();
                }

                copy_from_app_data = true;
            }
        }

        parsed_fields.push(ParsedField {
            ident: field.ident.as_ref().unwrap(),
            ty: &field.ty,
            val_var: format_ident!("__field_{idx}"),
            fut_var: format_ident!("__fut_{idx}"),
            copy_from_app_data,
        });
    }

    let field_inits = parsed_fields
        .iter()
        .map(|f| {
            let ParsedField { ident, val_var, .. } = f;
            quote! { #ident: #val_var }
        })
        .collect::<Punctuated<_, Comma>>();

    // i.e., field has no special handling, it's just extracted using its FromRequest impl
    let fut_fields = parsed_fields.iter().filter(|f| !f.copy_from_app_data);

    let field_fut_names_joined = fut_fields
        .clone()
        .map(|f| &f.fut_var)
        .collect::<Punctuated<_, Comma>>();

    let field_post_fut_names_joined = fut_fields
        .clone()
        .map(|f| &f.val_var)
        .collect::<Punctuated<_, Comma>>();

    let field_futs = fut_fields.map(|f| {
        let ParsedField { ty, fut_var, .. } = f;

        quote! {
            let #fut_var = <#ty>::from_request(&req, pl).map_err(Into::into);
        }
    });

    let fields_copied_from_app_data = parsed_fields
        .iter()
        .filter(|f| f.copy_from_app_data)
        .map(|f| {
            let ParsedField { ty, val_var, .. } = f;

            quote! {
                let #val_var = if let Some(st) = req.app_data::<#ty>().copied() {
                    st
                } else {
                    ::actix_web_lab::__reexports::tracing::debug!(
                        "Failed to extract `{}` for `{}` handler. For this extractor to work \
                        correctly, pass the data to `App::app_data()`. Ensure that types align in \
                        both the set and retrieve calls.",
                        ::std::any::type_name::<#ty>(),
                        req.match_name().unwrap_or_else(|| req.path())
                    );

                    return ::std::boxed::Box::pin(async move {
                        ::std::result::Result::Err(
                            ::actix_web_lab::__reexports::actix_web::error::ErrorInternalServerError(
                            "Requested application data is not configured correctly. \
                            View/enable debug logs for more details.",
                        ))
                    })
                };
            }
        });

    let body_fut = if field_post_fut_names_joined.is_empty() {
        quote! {
            ::std::boxed::Box::pin(async move {
                ::std::result::Result::Ok(Self { #field_inits })
            })
        }
    } else {
        quote! {
            ::std::boxed::Box::pin(
                async move { try_join!( #field_fut_names_joined ) }
                    .map_ok(move |( #field_post_fut_names_joined, )| Self { #field_inits })
            )
        }
    };

    let mut generics = input.generics.clone();
    if !input.generics.params.is_empty() {
        let where_clause = generics.make_where_clause();

        for param in input.generics.type_params() {
            let ident = &param.ident;
            where_clause
                .predicates
                .push(syn::parse_quote!(#ident: 'static));
        }

        for f in &parsed_fields {
            let ty = f.ty;
            if f.copy_from_app_data {
                where_clause
                    .predicates
                    .push(syn::parse_quote!(#ty: ::std::marker::Copy));
            } else {
                where_clause
                    .predicates
                    .push(syn::parse_quote!(#ty: ::actix_web::FromRequest));
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics ::actix_web::FromRequest for #name #ty_generics #where_clause {
            type Error = ::actix_web::Error;
            type Future = ::std::pin::Pin<::std::boxed::Box<
                dyn ::std::future::Future<Output = ::std::result::Result<Self, Self::Error>>
            >>;

            fn from_request(req: &::actix_web::HttpRequest, pl: &mut ::actix_web::dev::Payload) -> Self::Future {
                use ::actix_web_lab::__reexports::actix_web::FromRequest as _;
                use ::actix_web_lab::__reexports::futures_util::{FutureExt as _, TryFutureExt as _};
                use ::actix_web_lab::__reexports::tokio::try_join;

                #(#fields_copied_from_app_data)*

                #(#field_futs)*

                #body_fut
            }
        }
    };

    proc_macro::TokenStream::from(output)
}
