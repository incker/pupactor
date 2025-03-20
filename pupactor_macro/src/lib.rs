use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, LitStr};

#[proc_macro_derive(ActorMsgHandle, attributes(actor))]
pub fn actor_msg_handle_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident; // enum's name

    let mut actor_idents = Vec::new();

    // Extract name from attr #[actor(FirstTestActor)]
    for attr in input.attrs {
        if attr.path().is_ident("actor") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("kind") {
                    // this parses the `kind`
                    let value = meta.value()?; // this parses the `=`
                    let lit_str: LitStr = value.parse()?; // this parses `"EarlGrey"`
                    actor_idents.push(Ident::new(&lit_str.value(), lit_str.span()));
                    Ok(())
                } else {
                    Err(meta.error("no kind attribute"))
                }
            })
                .unwrap_or_else(|err| {
                    panic!("Failed to parse actor attribute: {}", err);
                });
        }
    }

    let expanded_list: Vec<_> = actor_idents
        .into_iter()
        .map(|actor_ident| {
            // Extract enum variants and generation match case
            let variants = if let Data::Enum(data_enum) = &input.data {
                data_enum.variants.iter().map(|variant| {
                    let variant_name = &variant.ident;
                    match &variant.fields {
                        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                            // let field_type = &fields.unnamed[0].ty;
                            quote! {
                                #enum_name::#variant_name(val) => pupactor::AsyncHandle::async_handle(self, val).await.into(),
                            }
                        }
                        _ => quote! {
                            _ => panic!("Unsupported enum variant or structure"),
                        },
                    }
                }).collect::<Vec<_>>()
            } else {
                panic!("ActorMsgHandle can only be derived for enums");
            };

            // Code generation
            quote! {
                impl pupactor::AsyncHandle<#enum_name> for #actor_ident {
                    #[inline(always)]
                    async fn async_handle(&mut self, value: #enum_name) -> pupactor::ActorCmdRes<Self::Cmd> {
                        match value {
                            #(#variants)*
                        }
                    }
                }
            }
        })
        .collect();

    TokenStream::from(quote! {
        #(#expanded_list)*
    })
}

#[proc_macro_derive(Pupactor, attributes(actor, listener))]
pub fn pupactor_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Search type `cmd`
    let mut cmd_ident = None;
    for attr in input.attrs {
        if attr.path().is_ident("actor") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("cmd") {
                    // this parses the `kind`
                    let value = meta.value()?; // this parses the `=`
                    let lit_str: LitStr = value.parse()?; // this parses `"EarlGrey"`
                    cmd_ident = Some(Ident::new(&lit_str.value(), lit_str.span()));
                    Ok(())
                } else {
                    Err(meta.error("no kind attribute"))
                }
            })
                .unwrap_or_else(|err| {
                    panic!("Failed to parse actor attribute: {}", err);
                });
        }
    }

    // Actor `cmd` msg. By default, is Infallible
    let cmd_ident = match cmd_ident {
        Some(cmd_ident) => {
            quote! { #cmd_ident }
        }
        None => quote! { std::convert::Infallible }
    };

    // Find all properties with attr #[listener]
    let listeners = if let Data::Struct(data_struct) = input.data {
        data_struct
            .fields
            .iter()
            .filter_map(|field| {
                let field_name = field.ident.clone();
                field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("listener"))
                    .map(|_| field_name)
            })
            .collect::<Vec<_>>()
    } else {
        panic!("`Pupactor` can only be derived for structs");
    };

    // Code that handle matches and handle msg
    let match_msg_inside_loop = quote! {
        match msg {
            pupactor::ActorMsg::Msg(msg) => {
                let cmd: pupactor::ActorCmdRes<Self::Cmd> = <Self as pupactor::AsyncHandle<_>>::async_handle(self, msg).await.into();
                if let Err(err) = cmd.0 {
                    if err.is_ok() {
                        return err;
                    } else {
                        break;
                    }
                } else {
                    continue;
                }
            }
            pupactor::ActorMsg::Cmd(cmd) => {
                return Ok(Self::Cmd::from(cmd));
            }
        }
    };

    let internal_loop = match listeners.len() {
        0 => quote! { },
        1 => {
            let field_name = listeners.first().unwrap();
            quote! {
                while let Some(msg) = Listener::next_msg(&mut self.#field_name).await {
                    #match_msg_inside_loop
                }
            }
        }
        _ => {
            let listener_branches = listeners.iter().map(|field_name| {
                quote! {
                    msg = Listener::next_msg(&mut self.#field_name) => {
                        if let Some(msg) = msg {
                            #match_msg_inside_loop
                        } else {
                            break;
                        }
                    }
                }
            });
            quote! {
                loop {
                    tokio::select! {
                        #(#listener_branches)*
                    }
                }
            }
        }
    };

    let expanded = quote! {
        impl pupactor::Actor for #struct_name {
            type Cmd = #cmd_ident;

            async fn infinite_loop(&mut self) -> Result<Self::Cmd, pupactor::Break> {
                #internal_loop
                Err(pupactor::Break)
            }
        }
    };
    TokenStream::from(expanded)
}

/// ActorCmd msg required always implement `From<Infallible>`
#[proc_macro_derive(ActorCmd)]
pub fn actor_cmd_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = quote! {
        impl From<std::convert::Infallible> for #struct_name {
            #[inline(always)]
            fn from(_: std::convert::Infallible) -> Self {
                unreachable!()
            }
        }
    };

    TokenStream::from(expanded)
}
