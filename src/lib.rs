use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Ident, Token, Type, Visibility};

struct AsyncStatic {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    init: Expr,
}

impl Parse for AsyncStatic {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![static]>()?;
        input.parse::<Token![ref]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let init: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(AsyncStatic {
            visibility,
            name,
            ty,
            init,
        })
    }
}

#[proc_macro]
pub fn async_static(input: TokenStream) -> TokenStream {
    let AsyncStatic {
        visibility,
        name,
        ty,
        init,
    } = parse_macro_input!(input as AsyncStatic);

    let init_future = quote_spanned! {init.span()=>
        once_cell::sync::Lazy::new(||std::sync::Mutex::new(Box::pin(async { #init })))
    };

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #visibility struct #name;

        impl std::future::Future for #name {
            type Output = &'static #ty;
            #[inline(always)]
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> std::task::Poll<Self::Output> {
                static ONCE: once_cell::sync::OnceCell<#ty> = once_cell::sync::OnceCell::new();
                static FUT: once_cell::sync::Lazy<std::sync::Mutex<std::pin::Pin<Box<dyn Send + std::future::Future<Output = #ty>>>>> = #init_future;

                // this is racy, but that's OK: it's just a fast case
                if let Some(v) = ONCE.get() {
                    return std::task::Poll::Ready(v);
                }
                if let Ok(mut fut) = FUT.try_lock() {
                    match fut.as_mut().poll(cx) {
                        std::task::Poll::Ready(value) => {
                            if ONCE.set(value).is_err() {
                                return std::task::Poll::Pending;
                            }
                        }
                        std::task::Poll::Pending => {
                            return std::task::Poll::Pending;
                        }
                    };
                    std::task::Poll::Ready(ONCE.get().unwrap())
                } else {
                    std::task::Poll::Pending
                }
            }
        }
    };

    TokenStream::from(expanded)
}
