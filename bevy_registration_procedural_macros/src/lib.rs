use foldhash::HashMap;

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    DeriveInput, Expr, Ident, ItemFn, Meta, Token, bracketed, parenthesized, parse::Parse,
    parse_macro_input, punctuated::Punctuated, token,
};

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
                bevy_registration::app!(|app| {
                    app.init_resource::<#struct_ident>();
                });
            });
        }

        if derives.contains("Event") {
            output.extend(quote! {
                bevy_registration::app!(|app| {
                    app.add_event::<#struct_ident>();
                });
            });
        }

        if derives.contains("Reflect") {
            output.extend(quote! {
                bevy_registration::app!(|app| {
                    app.register_type::<#struct_ident>();
                });
            });
        }
    });

    output.into()
}

/// Annotates a system, and adds it to a schedule.
/// It looks for the schedule in the crate root, so make sure it is accessible there.
/// Example:
/// ```
/// #[system(Update)]
/// fn some_system() {}
///```
/// To use it with [schedule!] you put :: between each path segment.
/// Example:
/// ```
/// #[system(Update::Test)]
/// fn some_system() {}
/// ```
#[proc_macro_attribute]
pub fn system(arguments: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as ItemFn);

    let ident = input.sig.ident;

    let mut output: TokenStream = item.into();

    let schedule = Ident::new(
        &arguments.to_string().replace(' ', "").replace("::", "_"),
        Span::call_site(),
    );

    output.extend(quote! {
        bevy_registration::app!(|app| {
            app.add_systems(crate::#schedule, #ident);
        });
    });

    output.into()
}

struct Schedule {
    attributes: Vec<(Ident, Expr)>,
    ident: Ident,
    subschedules: Punctuated<Schedule, Token![,]>,
}

impl Schedule {
    fn add_to_app(
        &mut self,
        define_structs: &mut TokenStream,
        app: &mut TokenStream,
        mut path: Vec<Ident>,
    ) {
        path.push(self.ident.clone());
        let path_quote = quote! {#(#path)_*};

        let attributes_template: HashMap<&str, Option<&Expr>> = ["run_every"]
            .into_iter()
            .map(|attribute| (attribute, None))
            .collect();

        let subschedules_attributes: Vec<HashMap<&str, Option<&Expr>>> = self
            .subschedules
            .iter()
            .map(|schedule| {
                let mut attributes = attributes_template.clone();
                schedule.attributes.iter().for_each(|(ident, expr)| {
                    let Some(attribute) = attributes.get_mut(ident.to_string().as_str()) else {
                        define_structs.extend(
                            syn::Error::new(ident.span(), "This attribute doesn't exist.")
                                .into_compile_error(),
                        );
                        return;
                    };

                    if attribute.is_none() {
                        *attribute = Some(expr);
                    } else {
                        define_structs.extend(
                            syn::Error::new(ident.span(), "You have already used this attribute.")
                                .into_compile_error(),
                        );
                    }
                });

                attributes
            })
            .collect();

        let subschedules_idents = self.subschedules.iter().map(|schedule| &schedule.ident);

        define_structs.extend(quote! {
            #(
                bevy_registration::paste! {
                    #[derive(bevy::ecs::schedule::ScheduleLabel, Hash, Debug, Eq, PartialEq, Clone, Default)]
                    pub struct [<#path_quote _ #subschedules_idents>];
                }
            )*
        });

        let subschedules_runners = self.subschedules.iter().zip(subschedules_attributes).map(|(schedule, attributes)| {
            let ident = &schedule.ident;

            if let Some(expr) = attributes.get("run_every").unwrap() {
                quote! {
                    |world: &mut bevy::prelude::World, mut time_passed: bevy::prelude::Local<std::time::Duration>| {
                        *time_passed += world.resource::<bevy::time::Time>().delta();

                        //warn_once!("TODO: The resource Time does not currently work in fixed timestep schedules.");
                        let time = world.remove_resource::<bevy::time::Time>().unwrap();
                        let mut fixed_time = bevy::time::Time::<bevy::prelude::Fixed>::from_duration(#expr).as_generic();
                        fixed_time.advance_by(#expr);
                        world.insert_resource(fixed_time);

                        while *time_passed >= #expr {
                            *time_passed -= #expr;
                            world.run_schedule([<#path_quote _ #ident>]::default());
                        }

                        world.remove_resource::<bevy::time::Time>().unwrap();
                        world.insert_resource(time);
                    }
                }
            } else {
                quote! {
                    bevy_registration::run_schedule::<[<#path_quote _ #ident>]>
                }
            }
        });

        app.extend(quote! {
            bevy_registration::paste! {
                app.add_systems(
                    [<#path_quote>],
                    bevy::prelude::IntoSystemConfigs::chain((#(#subschedules_runners),*)),
                );
            }
        });

        self.subschedules.iter_mut().for_each(|schedule| {
            if schedule.subschedules.is_empty() {
                return;
            }
            schedule.add_to_app(define_structs, app, path.clone());
        });
    }
}

impl Parse for Schedule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attributes = vec![];

        while input.peek(token::Bracket) {
            let attribute;
            bracketed!(attribute in input);

            let ident = attribute.parse()?;

            let expression;
            parenthesized!(expression in attribute);

            let expression = expression.parse()?;

            attributes.push((ident, expression));
        }

        let ident = input.parse()?;

        let subschedules = if input.peek(token::Paren) {
            let subschedules;
            parenthesized!(subschedules in input);

            subschedules.parse_terminated(Self::parse, Token![,])?
        } else {
            Default::default()
        };

        Ok(Self {
            attributes,
            ident,
            subschedules,
        })
    }
}

/// Creates a schedule heirarchy.
/// Schedules in the heirarchy may have these attributes:
/// * `run_every(Duration)` Runs the schedule every time the duration passes. May run the schedule multiple times.
/// ## Example:
/// ```
/// schedule! {
///     Update(
///         [run_every(Duration::from_secs_f32(1.))]
///         Test(
///             First,
///             Second,
///             Third,
///         ),
///     )
/// }
///
/// #[system(Update::Test::First)]
/// fn first() {
///     info!("1");
/// }
///
/// #[system(Update::Test::Second)]
/// fn second() {
///     info!("2");
/// }
///
/// #[system(Update::Test::Third)]
/// fn third() {
///     info!("3");
/// }
///
/// #[system(Update::Test)]
/// fn random() {
///     info!("random");
/// }
/// ```
/// Every 1 second, the Test schedule will run. First, Second, and Third always run in that order, with random being able to run at any point.
#[proc_macro]
pub fn schedule(input: StdTokenStream) -> StdTokenStream {
    let mut input = parse_macro_input!(input as Schedule);

    if !input.attributes.is_empty() {
        return syn::Error::new(input.ident.span(), "Base schedule cannot have attributes.")
            .into_compile_error()
            .into();
    }

    let mut app = quote! {};
    let mut define_structs = quote! {};

    input.add_to_app(&mut define_structs, &mut app, vec![]);

    quote! {
        #define_structs

        bevy_registration::app!(|app| {
            #app
        });
    }
    .into()
}
