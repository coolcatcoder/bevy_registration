use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput, Meta};
use quote::quote;

/// Initiates events, resources, and reflected structs.
/// Place above the derives.
/// Example:
/// ```
/// #[init]
/// #[derive(Resource, Default)]
/// struct TestResource;
///```
#[proc_macro_attribute]
pub fn init(_arguments: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let input = item.clone();
    // A tad dodgy, but when an attribute macro is on an item, it will have the same input as a derive macro.
    let input = parse_macro_input!(input as DeriveInput);

    let mut output: TokenStream = item.into();

    let struct_ident = input.ident;

    input.attrs.into_iter().for_each(|attribute| {
        let Some(ident) = attribute.path().get_ident() else {
            return;
        };

        if ident != "derive" {
            return;
        }

        let Meta::List(derives) = attribute.meta else {
            // Derive should always be a Meta::List so this won't ever return.
            return;
        };

        let derives = derives.tokens.to_string();

        if derives.contains("Resource") {
            output.extend(quote! {
                app!(|app| {
                    app.init_resource::<#struct_ident>();
                });
            });
        }

        if derives.contains("Event") {
            output.extend(quote! {
                app!(|app| {
                    app.add_event::<#struct_ident>();
                });
            });
        }

        if derives.contains("Reflect") {
            output.extend(quote! {
                app!(|app| {
                    app.register_type::<#struct_ident>();
                });
            });
        }
    });

    output.into()
}