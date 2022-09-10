use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn;
use syn::{AttributeArgs, Expr, FnArg, GenericArgument, Ident, ItemFn, Lit, LitStr, NestedMeta, parse_macro_input, Pat, PathArguments, PathSegment, PatLit, PatType, ReturnType, Type};
use syn::__private::TokenStream2;
use syn::punctuated::{Pair, Pairs};
use syn::token::Comma;


pub fn impl_adhoc_reqwest_fn(attr:&AttributeArgs,item:&ItemFn) -> TokenStream {
    let name = &item.sig.ident;
    let relative_route = match attr[0].clone() {
        NestedMeta::Meta(_) => panic!("Only str args are supported: i.e \"get\" instead of get, \
          you get it?"),
        NestedMeta::Lit(lit) => match lit {
            Lit::Str(lit) => lit,
            _ => panic!("unsuported func args"),
        }
    };
    if !relative_route.value().as_str().starts_with("/") {
        panic!("All routes must start with /, your base URL must not end with /")
    }

    let method = quote!(.post(format!("{}{}",base,#relative_route)));


    let mut func_generics = Vec::new();
    func_generics.push(quote!(U:std::fmt::Display));

    let (
        arg_vars,
        arg_types,
        args_formatted,
    ) = {
        let mut arg_vars = Vec::new();
        let mut arg_types = Vec::new();
        let mut args_formatted = Vec::new();
        let pairs = item.sig.inputs.pairs().clone();
        for pair in pairs {
            match pair.value().clone() {
                // Our pat type  looks like
                // BinHocN(var1,var2,...,varN):BinHocN<Type1,Type2,...,TypeN>
                FnArg::Typed(pat_ty) => {
                    if let Type::Path(ty) = *pat_ty.ty.clone() {
                        // skip types that don't start with BinHoc
                        if !ty.path.segments.last().expect("type name").ident.clone()
                            .to_string().starts_with("BinHoc") {
                            continue
                        } else {}
                    } else {
                        panic!("expecting path types in function signature")
                    }
                    // Split our type into variable side and type side at the colon
                    let variable_side = pat_ty.pat.clone();
                    match *variable_side {
                        // Pat should be BinHocN(var1,var2,...,varN)
                        // This is the only type we handle because
                        // BinHocN(...) is a tuple struct.
                        Pat::TupleStruct(pat_tup) => {
                            // Update arg vars to the vars inside the variable side
                            arg_vars = pat_tup.pat.elems.pairs().into_iter()
                                .map(|pair| pair.into_tuple())
                                .map(|(var, _)| {
                                    if let Pat::Ident(pat_ident) = var {
                                        pat_ident.ident.clone()
                                    } else {
                                        panic!("var name in pattern match not an Ident")
                                    }
                                }).collect::<Vec<Ident>>();
                        }
                        _ => {}
                    };
                    let type_side = pat_ty.ty.clone();
                    match *type_side {
                        // Get the last segment of the path, which will be BinHocN
                        Type::Path(ty_path) => {
                            let last = ty_path.path.segments.last()
                                .expect("There to be a last segment");
                            let type_args = last.arguments.clone();
                            // if we have a generic arg then we handle it, otherwise we'll skip this code.
                            match type_args {
                                PathArguments::AngleBracketed(gen_args) => {
                                    let gen_args = gen_args.args
                                        .pairs()
                                        .into_iter()
                                        .map(|pair| pair.into_tuple())
                                        .map(|(ty, _comma)| ty.to_owned())
                                        .collect::<Vec<GenericArgument>>();
                                    // For each arg in gen args push arg onto arg_types.
                                    for arg in gen_args {
                                        match arg {
                                            GenericArgument::Type(ty) => {
                                                match ty {
                                                    Type::Path(ty_path) => {
                                                        let last = ty_path.path.segments.last()
                                                            .expect("atleast one segment per type");
                                                        let ident = last.ident.clone();
                                                        let ident = {
                                                            if let PathArguments::AngleBracketed(gen_args)
                                                            = last.arguments.clone() {
                                                                let arg = gen_args.args.first().expect("gen arg in angle brack arg");
                                                                if let GenericArgument::Type(ty) = arg {
                                                                    if let Type::Path(ty_path) = ty {
                                                                        let gen_arg =  ty_path.path.segments.last()
                                                                            .expect("at least one segment per type")
                                                                            .clone()
                                                                            .ident;
                                                                        quote!(#ident<#gen_arg>)
                                                                    } else { panic!("expecting path types only") }
                                                                } else {
                                                                    panic!("Unhandled generic argument enum")
                                                                }
                                                            } else {
                                                                quote!(#ident)
                                                            }
                                                        };
                                                        arg_types.push(ident);
                                                    }
                                                    _ => {
                                                        panic!("expecting path types only")
                                                    }
                                                }
                                            }
                                            _ => {
                                                panic!("Expecting types only")
                                            }
                                        }
                                    }
                                },
                                _ => continue,
                            }
                        },
                        _ => { panic!("Expecting something like path::BinHocN<args...>.") }
                    }
                },
                _ => {panic!("Last field should be typed not reciever.")}
            }
        }
        for (var,ty) in arg_vars.iter().zip(arg_types.iter()) {
            args_formatted.push(
                quote!(#var : #ty)
            );
        }
        (arg_vars,arg_types,args_formatted)
    };

    let func_generics = {
        let mut list = Vec::new();
        let last = func_generics.pop().unwrap();
        for other in func_generics {
            list.push(quote!(#other,))
        }
        list.push(quote!(#last));
        list
    };
    let mod_name = format_ident!("binhoc_client_{}",name);
    let handler_mod_name = format_ident!("binhoc_server_{}",name);
    // arg_types = T
    // arg_vars = var
    // args_formatted = var : T
    let gen = quote! {
        pub mod #mod_name {
            use super::*;
            use bincode::{Decode,Encode};
            #[derive(Encode,Decode)]
            struct BinHoc(#(#arg_types),*);
            pub async fn #name<#(#func_generics)*>(
                client:&reqwest::Client,
                base:U,
                #(#args_formatted),*)
            -> Result<reqwest::Response,anyhow::Error> {
                let body = BinHoc(#(#arg_vars),*);
                let body = bincode::encode_to_vec(
                    body,
                    bincode::config::standard()
                )?;
                Ok(client
                    #method
                    .body(body)
                    .send()
                    .await?)
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        pub mod #handler_mod_name {
            use super::*;
            #item
        }
    };
    gen.into()
}

