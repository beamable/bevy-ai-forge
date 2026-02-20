extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Ident, Lit, LitStr, Token, Type};

/// A helper structure to parse the attribute arguments.
/// We expect the attribute to have the syntax:
///   #[beam_command(YourType)]
///
/// Since `type` is a reserved keyword in Rust, we parse it as an identifier and then
/// store its string literal value in the field `ty`.
struct BeamCommandDeriveArgs {
    event_type: Ident,
    success_type: Type,
    failure_type: Type,
    open_api_call: Expr,
}

impl Parse for BeamCommandDeriveArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let event_type: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let success_type: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let failure_type: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let open_api_call: Expr = input.parse()?;
        Ok(BeamCommandDeriveArgs {
            event_type,
            success_type,
            failure_type,
            open_api_call,
        })
    }
}

/// The derive macro implementation.
/// It looks for a helper attribute `#[beam_command(...)]` and uses its argument
/// to generate an implementation of `BeamCommand`.
#[proc_macro_derive(BeamCommand, attributes(beam_command))]
pub fn beam_command(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let Some(attr) = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("beam_command"))
    else {
        return syn::Error::new(
            input.ident.span(),
            "Missing attribute: #[beam_command(...)]",
        )
        .to_compile_error()
        .into();
    };
    let args: BeamCommandDeriveArgs = match attr.parse_args() {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };

    // Get the name of the type the macro is applied to.
    let name = input.ident;
    let result_type_success = args.success_type;
    let result_type_error = args.failure_type;
    let call = args.open_api_call;

    let (entity_assign, call_expr, args_type, call) =
        match types_from_struct_definition_data(&input.data) {
            None => (
                quote! { let token = self.0.clone(); },
                quote! { #name::call((), &request_client, &mut *s, token); },
                syn::parse_quote! { () },
                quote! { #call(&config).await},
            ),
            Some(s) => (
                quote! { let token = self.1.clone(); },
                quote! { #name::call(self.0.clone(), &request_client, &mut *s, token); },
                s,
                quote! { #call(&config, data).await},
            ),
        };
    let event_completed_name = args.event_type.clone();

    let task_name = syn::Ident::new(
        &format!("{}Resource", args.event_type),
        args.event_type.span(),
    );

    // Generate the implementation.
    let expanded = quote! {
            impl BeamRequest<#result_type_success, #result_type_error> for #name {
                type Config = apis::configuration::Configuration;
                type ArgsType = #args_type;

                 fn config_from(client: &ReqwestClient) -> Self::Config {
                    let api_key = Some(apis::configuration::ApiKey {
                        key: client.x_beam_scope.clone(),
                        prefix: None,
                    });
                    let bearer_access_token = client.access_token.clone();
                    apis::configuration::Configuration {
                        user_agent: Some("Bevy-0.15".to_owned()),
                        client: (**client).clone(),
                        api_key,
                        bearer_access_token,
                        ..Default::default()
                    }
                }
                fn call<T>(
                    data: Self::ArgsType,
                    request_client: &ReqwestClient,
                    handler: &mut T,
                    entity: Entity
                ) where T : BeamRequestResource<#result_type_success, #result_type_error>{
                    let thread_pool = bevy::tasks::IoTaskPool::get();
                    let config = Self::config_from(&request_client);
                    let gamer_tag = request_client.gamer_tag.clone();

                    let (tx, task) = crossbeam_channel::bounded(1);

                    thread_pool
                        .spawn(async move {
                            #[cfg(target_family = "wasm")]
                            let r = #call;

                            #[cfg(not(target_family = "wasm"))]
                            let r =
                                async_compat::Compat::new(async { #call })
                                    .await;
                            tx.send(r).ok();

                        })
                        .detach();
                    handler.add(task,gamer_tag, entity);
                }
            }

        impl bevy::ecs::system::Command for #name {
            fn apply(self, world: &mut World) {
                #entity_assign
                let request_client = Self::make_request_client(&world, token);
                if let Some(mut s) = world.get_resource_mut::<#task_name>() {
                    #call_expr
                }
            }
        }

        #[derive(EntityEvent, Debug, Deref, DerefMut)]
        pub struct #event_completed_name(
            #[deref] pub Result<#result_type_success, #result_type_error>,
            #[event_target] pub Entity,
        );

        impl BeamEventFactory<#result_type_success, #result_type_error> for #event_completed_name {
            fn for_entity(value: Result<#result_type_success, #result_type_error>, entity: Entity) -> Self {
                #event_completed_name(value, entity)
            }
        }

        #[derive(Resource, Deref, DerefMut, Default)]
        pub struct #task_name(pub Vec<BeamReceiver<#result_type_success, #result_type_error>>);

        impl BeamRequestResource<#result_type_success, #result_type_error> for #task_name {
            type Event = #event_completed_name;
        }

        impl #event_completed_name {
            pub fn register(app: &mut ::bevy::app::App) {
                app.init_resource::<#task_name>()
                    .add_systems(Update, #task_name::handle_requests);
            }
        }
    };

    // Return the generated code.
    TokenStream::from(expanded)
}

fn types_from_struct_definition_data(data: &Data) -> Option<Type> {
    match data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Unnamed(ref fields_unnamed) => {
                if fields_unnamed.unnamed.len() == 2 {
                    Some(fields_unnamed.unnamed[0].clone().ty)
                } else {
                    None
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// The derive macro implementation.
/// It looks for a helper attribute `#[beam_notify(...)]` and uses its argument
/// to generate an implementation of `BeamNotify`.
#[proc_macro_derive(BeamNotify, attributes(beam_notify))]
pub fn beam_notify_command(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let Some(attr) = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("beam_notify"))
    else {
        return syn::Error::new(
            input.ident.span(),
            "Missing attribute: #[beam_notify(\"notification.id\")]",
        )
        .to_compile_error()
        .into();
    };
    let error = syn::Error::new_spanned(attr, "Expected #[beam_notify(\"notification.id\")]")
        .to_compile_error()
        .into();
    // Parse the attribute to ensure it's a string literal

    let meta: LitStr = match attr.parse_args() {
        Ok(Expr::Lit(expr)) => {
            if let Lit::Str(lit_str) = &expr.lit {
                lit_str.clone()
            } else {
                return error;
            }
        }
        _ => {
            return error;
        }
    };

    let command_str = meta.value();

    // You can now use `command_str` as needed to generate your code
    let name = &input.ident;

    let gen = quote! {
        impl NetworkMessage for #name {
            const NAME: &'static str = #command_str;

            fn deserialize_message<T>(bytes: &[u8]) -> Option<T>
            where
                T: NetworkMessage,
            {
                serde_json::from_slice::<T>(bytes).ok()
            }
        }
    };

    gen.into()
}
