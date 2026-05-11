use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated,
    token::Comma, ItemMod, LitStr, Meta, Path, Token,
};

// ── Parse middleware = [fn1, fn2] ─────────────────────────────────────────────

struct MiddlewareArgs {
    middlewares: Punctuated<Path, Comma>,
}

impl Parse for MiddlewareArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        Ok(MiddlewareArgs {
            middlewares: content.parse_terminated(Path::parse, Token![,])?,
        })
    }
}

// ── #[routes(state = AppState)] ───────────────────────────────────────────────

#[proc_macro_attribute]
pub fn routes(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_mod = parse_macro_input!(input as ItemMod);
    let mod_name = &input_mod.ident;

    // Parse state type
    let state_str = args.to_string().replace("state =", "").trim().to_string();
    let state_type: syn::Type = syn::parse_str(&state_str).unwrap();

    let content = match &input_mod.content {
        Some((_, items)) => items.clone(),
        None => vec![],
    };

    let mut route_registrations = vec![];
    let mut fn_definitions = vec![];

    for item in &content {
        if let syn::Item::Fn(func) = item {
            let fn_name = &func.sig.ident;
            let mut route_path: Option<LitStr> = None;
            let mut http_method: Option<String> = None;
            let mut middlewares: Vec<Path> = vec![];

            for attr in &func.attrs {
                // get/post/put/delete/patch
                let method = if attr.path().is_ident("get") { Some("get") }
                    else if attr.path().is_ident("post") { Some("post") }
                    else if attr.path().is_ident("put") { Some("put") }
                    else if attr.path().is_ident("delete") { Some("delete") }
                    else if attr.path().is_ident("patch") { Some("patch") }
                    else { None };

                if let Some(m) = method {
                    http_method = Some(m.to_string());

                    // parse path and optional middleware = [...]
                    if let Meta::List(list) = &attr.meta {
                        let tokens = list.tokens.clone();
                        // try parse as "path" or "path, middleware = [...]"
                        let parsed = syn::parse::Parser::parse2(|input: ParseStream| {
                            let path: LitStr = input.parse()?;
                            let mut mws = vec![];
                            if input.peek(Token![,]) {
                                let _: Token![,] = input.parse()?;
                                let ident: syn::Ident = input.parse()?;
                                if ident == "middleware" {
                                    let _: Token![=] = input.parse()?;
                                    let mw_args: MiddlewareArgs = input.parse()?;
                                    mws = mw_args.middlewares.into_iter().collect();
                                }
                            }
                            Ok((path, mws))
                        }, tokens);

                        if let Ok((path, mws)) = parsed {
                            route_path = Some(path);
                            middlewares = mws;
                        }
                    }
                }
            }

            if let (Some(path), Some(method)) = (route_path, http_method) {
                let method_ident: proc_macro2::TokenStream = method.parse().unwrap();

                // build route with or without middleware
                let route = if middlewares.is_empty() {
                    quote! {
                        .route(#path, #method_ident(#fn_name))
                    }
                } else {
                    let layers = middlewares.iter().map(|mw| {
                        quote! {
                            .route_layer(middleware::from_fn_with_state(
                                state.clone(),
                                #mw
                            ))
                        }
                    });
                    quote! {
                        .route(#path, #method_ident(#fn_name))
                        #(#layers)*
                    }
                };

                route_registrations.push(route);
            }

            // keep the function definition
            let mut clean_fn = func.clone();
            clean_fn.attrs.retain(|a| {
                !a.path().is_ident("get") &&
                !a.path().is_ident("post") &&
                !a.path().is_ident("put") &&
                !a.path().is_ident("delete") &&
                !a.path().is_ident("patch")
            });
            fn_definitions.push(quote! { #clean_fn });
        }
    }

    let expanded = quote! {
        pub mod #mod_name {
            use super::*;
            use axum::{
                Router,
                routing::{get, post, put, delete, patch},
                middleware,
            };

            #(#fn_definitions)*

            pub fn router(state: #state_type) -> Router<#state_type> {
                Router::new()
                    #(#route_registrations)*
            }
        }
    };

    TokenStream::from(expanded)
}