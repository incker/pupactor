extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, LitStr};

#[proc_macro_derive(ActorMsgHandle, attributes(actor))]
pub fn actor_msg_handle_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident; // Имя enum'а

    // Извлекаем имя структуры из атрибута #[actor(FirstTestActor)]
    let mut actor_ident = None;
    for attr in input.attrs {
        if attr.path().is_ident("actor") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("kind") {  // this parses the `kind`
                    let value = meta.value()?;   // this parses the `=`
                    let lit_str: LitStr = value.parse()?;  // this parses `"EarlGrey"`
                    actor_ident = Some(Ident::new(&lit_str.value(), lit_str.span()));
                    Ok(())
                } else {
                    Err(meta.error("no kind attribute"))
                }
            }).unwrap_or_else(|err| {
                panic!("Failed to parse actor attribute: {}", err);
            });
        }
    }

    let actor_ident = actor_ident.expect("Expected actor name in #[actor(...)] attribute");

    // Извлечение вариантов enum и генерация соответствующих match-веток
    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // let field_type = &fields.unnamed[0].ty;
                    quote! {
                        #enum_name::#variant_name(val) => AsyncHandle::async_handle(self, val).await.into(),
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

    // Генерация кода
    let expanded = quote! {
        impl AsyncHandle<#enum_name> for #actor_ident {
            #[inline(always)]
            async fn async_handle(&mut self, value: #enum_name) -> ActorCommand<Self::ShutDown> {
                match value {
                    #(#variants)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(Pupactor, attributes(actor, listener))]
pub fn pupactor_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Найдем атрибут actor, чтобы получить тип ShutDown
    let mut shutdown_ident = None;
    for attr in input.attrs {
        if attr.path().is_ident("actor") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("shutdown") {  // this parses the `kind`
                    let value = meta.value()?;   // this parses the `=`
                    let lit_str: LitStr = value.parse()?;  // this parses `"EarlGrey"`
                    shutdown_ident = Some(Ident::new(&lit_str.value(), lit_str.span()));
                    Ok(())
                } else {
                    Err(meta.error("no kind attribute"))
                }
            }).unwrap_or_else(|err| {
                panic!("Failed to parse actor attribute: {}", err);
            });
        }
    }
    let shutdown_ident = shutdown_ident.expect("Expected an `actor(shutdown = \"...\")` attribute");


    // todo
    // let shutdown_ident = shutdown_ident.unwrap_or_else(|| {
    //     let lit_str: LitStr = "Infallible".parse().unwrap();  // this parses `"EarlGrey"`
    //     shutdown_ident = Some(Ident::new(&lit_str.value(), lit_str.span()));
    // });

    // Находим все поля с атрибутом #[listener]
    let listeners = if let Data::Struct(data_struct) = input.data {
        data_struct.fields.iter().filter_map(|field| {
            let field_name = field.ident.clone();
            field.attrs.iter().find(|attr| attr.path().is_ident("listener")).map(|_| field_name)
        }).collect::<Vec<_>>()
    } else {
        panic!("`Pupactor` can only be derived for structs");
    };


    /*
        // Извлечение вариантов enum и генерация соответствующих match-веток
    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field_type = &fields.unnamed[0].ty;
                    quote! {
                        #enum_name::#variant_name(val) => AsyncHandle::async_handle(self, val).await.into(),
                    }
                }
                _ => quote! {
                    _ => panic!("Unsupported enum variant or structure"),
                },
            }
        }).collect::<Vec<_>>()
    } else {
        panic!("MyMacro
     */


    // Генерация кода для каждого listener
    let listener_branches = listeners.iter().map(|field_name| {
        quote! {
            msg = Listener::next_msg(&mut self.#field_name) => {
                if let Some(msg) = msg {
                    match msg {
                        ActorMsg::Msg(msg) => {
                            let command: ActorCommand<Self::ShutDown> = <Self as AsyncHandle<_>>::async_handle(self, msg).await.into();
                            if let Err(err) = command.0 {
                                let _ = err?;
                                break;
                            } else {
                                continue;
                            }
                        }
                        ActorMsg::Shutdown(shutdown) => {
                            return Err(Self::ShutDown::from(shutdown));
                        }
                    }
                } else {
                    break;
                }
            }
        }
    });

    // Генерация полного кода
    let expanded = quote! {
        impl Actor for #struct_name {
            type ShutDown = #shutdown_ident;

            async fn infinite_loop(&mut self) -> Result<Break, Self::ShutDown> {
                loop {
                    select! {
                        #(#listener_branches)*
                    }
                }
                Ok(Break)
            }
        }
    };
    TokenStream::from(expanded)
}

/// ActorShutdown msg required always implement `From<Infallible>`
#[proc_macro_derive(ActorShutdown)]
pub fn actor_shutdown_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident; // Имя структуры

    let expanded = quote! {
        impl From<Infallible> for #struct_name {
            #[inline(always)]
            fn from(_: Infallible) -> Self {
                unreachable!()
            }
        }
    };

    TokenStream::from(expanded)
}
