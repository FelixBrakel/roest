use proc_macro::TokenStream;
use quote::quote;
use syn::{Type, Ident, Generics};
use proc_macro_crate::crate_name;
use proc_macro2::Span;

pub fn generate_gpu_variant_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let name = std::env::var("CARGO_PKG_NAME").unwrap();

    let gl_path = if name == "gl_renderer" {
        quote! {
            crate
        }
    } else {
        let name = crate_name("gl_renderer")
            .expect("gl_renderer has not been defined in Cargo.toml");
        let crate_ident = Ident::new(&name, Span::call_site());
        quote! {
            #crate_ident
        }
    };

    let proxy_struct = generate_gpu_proxy_struct(ident, &ast.data, &gl_path, generics);

    let proxy_struct_impl = generate_gpu_proxy_struct_impl(ident, &ast.data, &gl_path, generics);

    let gen = quote! {
        #proxy_struct

        #proxy_struct_impl
    };

    gen.into()
}

fn generate_gpu_proxy_struct_impl(name: &syn::Ident, data: &syn::Data, gl_path: &proc_macro2::TokenStream, generics: &Generics) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (set_calls, construct_calls) = generate_gpu_proxy_struct_calls(data, gl_path);
    let struct_name = syn::Ident::new(&format!("GPU{}", name), name.span());
    quote! {
        impl #impl_generics #gl_path::GPUAggregate for #struct_name #ty_generics #where_clause {
            type Input = #name #ty_generics #where_clause;

            fn from_name(program: &#gl_path::Program, name: &str, ub: std::sync::Arc<#gl_path::uniform_buffer::UniformBlock>) -> Self {
                #struct_name {
                    #(#construct_calls),*
                }
            }

            fn set(&self, data: &#name) {
                #(#set_calls);*
            }
        }

        impl #impl_generics #gl_path::GPUVariant for #name #ty_generics #where_clause {
            type Variant = #struct_name;
            type ArrayVariant = #gl_path::GPUAggregateArray<#name>;
        }
    }
}

fn generate_gpu_proxy_struct_calls(data: &syn::Data, gl_path: &proc_macro2::TokenStream) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let set_calls = match &data {
        syn::Data::Enum(_) => panic!("GPUVariant can not be implemented for enums"),
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => panic!("GPUVariant can not be implemented for Unit structs"),
            syn::Fields::Unnamed(_) => panic!("GPUVariant can not be implemented  for Tuple structs"),
            syn::Fields::Named(fields) => fields.named
                .iter()
                .map(generate_gpu_proxy_set_call)
                .collect()
        },
        syn::Data::Union(_) => unimplemented!()
    };

    let construct_calls = match &data {
        syn::Data::Enum(_) => panic!("GPUVariant can not be implemented for enums"),
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => panic!("GPUVariant can not be implemented for Unit structs"),
            syn::Fields::Unnamed(_) => panic!("GPUVariant can not be implemented  for Tuple structs"),
            syn::Fields::Named(fields) => fields.named
                .iter()
                .map(|field| (generate_gpu_proxy_construct_call(field, gl_path)))
                .collect()
        },
        syn::Data::Union(_) => unimplemented!()
    };

    (set_calls, construct_calls)
}

fn generate_gpu_proxy_set_call(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = match &field.ident {
        Some(name) => name,
        None => panic!("Unnamed fields not supported")
    };

    let gen = quote! {
        self.#field_name.set(&data.#field_name)
    };

    gen.into()
}

fn generate_gpu_proxy_construct_call(field: &syn::Field, gl_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let field_name = match &field.ident {
        Some(name) => name,
        None => panic!("Unnamed fields not supported")
    };

    let ty = get_variant_ty(field, gl_path);

    let gen = match &field.ty {
        Type::Array(arr) => {
            let arr_len = &arr.len;
            quote! {
            #field_name: #ty::from_name(program, &format!("{}.{}", name, stringify!(#field_name)), #arr_len, ub.clone())
            }
        }
        _ => quote! {
            #field_name: #ty::from_name(program, &format!("{}.{}", name, stringify!(#field_name)), ub.clone())
        }
    };

    gen.into()
}


fn generate_gpu_proxy_struct(ident: &syn::Ident, data: &syn::Data, gl_path: &proc_macro2::TokenStream, generics: &Generics) -> proc_macro2::TokenStream {
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let struct_name = syn::Ident::new(&format!("GPU{}", ident), ident.span());

    let fields = generate_gpu_proxy_struct_fields(data, gl_path);

    quote! {
        pub struct #struct_name #ty_generics #where_clause {
            #(#fields)*
        }
    }
}

fn generate_gpu_proxy_struct_fields(data: &syn::Data, gl_path: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    match &data {
        syn::Data::Enum(_) => panic!("GPUVariant can not be implemented for enums"),
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => panic!("GPUVariant can not be implemented for Unit structs"),
            syn::Fields::Unnamed(_) => panic!("GPUVariant can not be implemented  for Tuple structs"),
            syn::Fields::Named(fields) => fields.named
                .iter()
                .map(|field| { generate_gpu_proxy_struct_field(field, gl_path) })
                .collect()
        },
        syn::Data::Union(_) => unimplemented!()
    }
}

fn generate_gpu_proxy_struct_field(field: &syn::Field, gl_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let field_name = match &field.ident {
        Some(name) => name,
        None => panic!("Unnamed fields not supported")
    };

    let ty = get_variant_ty(field, gl_path);

    let gen = quote! {
        pub #field_name: #ty,
    };

    gen.into()
}

fn get_variant_ty(field: &syn::Field, gl_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &field.ty {
        Type::Array(arr) => {
            let tmp = &arr.elem;
            quote!{
                <#tmp as #gl_path::GPUVariant>::ArrayVariant
            }
        },
        _ => {
            let tmp = &field.ty;
            quote! {
                <#tmp as #gl_path::GPUVariant>::Variant
            }
        }
    }
}
