// Imagine a single #[register] attribute.

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Item, ItemFn, Path, Stmt, parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
pub fn register(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as Item);
    //let input = parse_macro_input!(input as Path);
    register_internal(input, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn register_internal(input: StdTokenStream, item: Item) -> syn::Result<TokenStream> {
    match item {
        Item::Fn(function) => {
            let function_attrs = function.attrs;
            let function_sig = function.sig;
            let function_vis = function.vis;
            let mut function_block = function.block;
            function_block
                .stmts
                .push(Stmt::Expr(syn::parse2(quote! {Some(())})?, None));

            Ok(quote! {
                #(#function_attrs)*
                #function_vis #function_sig {
                    || -> Option<()> #function_block ();
                }
            })
        }
        _ => {
            return Err(syn::Error::new(
                item.span(),
                "That item can currently not be registered.",
            ));
        }
    }
}