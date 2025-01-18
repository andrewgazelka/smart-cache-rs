use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use sha2::{Digest, Sha256};
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat, ReturnType, Type};

fn hash_token_stream(tokens: &proc_macro2::TokenStream) -> [u8; 32] {
    // Convert TokenStream to a string representation
    let token_string = tokens.to_string();

    // Create a new SHA-256 hasher
    let mut hasher = Sha256::new();

    // Update hasher with token string bytes
    hasher.update(token_string.as_bytes());

    // Finalize and return the hash as bytes
    hasher.finalize().into()
}

fn check_for_mutable_refs(
    fn_inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> Result<(), syn::Error> {
    for arg in fn_inputs {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };

        let Type::Reference(type_ref) = &*pat_type.ty else {
            continue;
        };

        let Some(mutability) = &type_ref.mutability else {
            continue;
        };

        return Err(syn::Error::new_spanned(
            mutability,
            "cached functions must be pure - mutable references are not allowed",
        ));
    }
    Ok(())
}

fn get_param_type(ty: &Type) -> &Type {
    if let Type::Reference(type_ref) = ty {
        &type_ref.elem
    } else {
        ty
    }
}

#[proc_macro_attribute]
pub fn cached(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Check for mutable references and return the original function with error if found
    if let Err(err) = check_for_mutable_refs(&input_fn.sig.inputs) {
        let compiler_err = err.to_compile_error();

        return quote! {
            #input_fn

            #compiler_err
        }
        .into();
    }

    let mut input_fn = input_fn;

    let mut fn_with_name_inner = input_fn.clone();
    fn_with_name_inner.sig.ident = Ident::new("inner", Span::call_site());

    let fn_with_name_inner_tokens = quote! {
        #fn_with_name_inner
    };

    let inner_fn_hash = hash_token_stream(&fn_with_name_inner_tokens);

    // Convert the [u8; 32] to a literal array expression
    let inner_fn_hash_literal = quote! {
        [
            #(#inner_fn_hash,)*
        ]
    };

    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = match &input_fn.sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    let param_names: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some(&pat_ident.ident)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let param_types: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => Some(get_param_type(&pat_type.ty)),
            _ => None,
        })
        .collect();

    let new_block = quote! {{
        #fn_with_name_inner

        use rkyv::{with::InlineAsBox, Archive, Deserialize, Serialize};

        #[derive(Archive, Serialize, Deserialize, Debug)]
        struct CacheKey<'a> {
            #(
                #[rkyv(with = InlineAsBox)]
                #param_names: &'a #param_types,
            )*
            _function_hash: [u8; 32],
        }

        let key = CacheKey {
            #(#param_names: &#param_names,)*
            _function_hash: #inner_fn_hash_literal,
        };
        println!("{key:?}");
        let key_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&key).unwrap();

        if let Some(cached_result) = smart_cache::get_cached(&*key_bytes) {
            let cached_result = &*cached_result;
            let cached_result: &rkyv::Archived<#fn_output> = rkyv::access::<_, rkyv::rancor::Error>(cached_result).unwrap();
            let cached_result: #fn_output = rkyv::deserialize::<#fn_output, rkyv::rancor::Error>(cached_result).unwrap();
            return cached_result;
        }

        let result = inner(#(#param_names,)*);

        let value_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&result).unwrap();
        let _ = smart_cache::set_cached(&key_bytes, &value_bytes);

        result
    }};

    input_fn.block = syn::parse2(new_block).unwrap();

    TokenStream::from(quote! {
        #input_fn
    })
}
