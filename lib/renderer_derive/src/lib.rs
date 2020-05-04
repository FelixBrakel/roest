#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;

mod gl_setters;
mod gl_getters;
mod gpu_variant;

use syn;
use quote::quote;
use proc_macro::{TokenStream};

use gl_setters::generate_gl_set_impl;
use gl_getters::generate_gl_get_impl;
use gpu_variant::generate_gpu_variant_impl;

#[proc_macro_derive(gl_setters, attributes(location))]
pub fn gl_set_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    generate_gl_set_impl(&ast)
}

#[proc_macro_derive(gl_getters, attributes(location))]
pub fn gl_get_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    generate_gl_get_impl(&ast)
}

#[proc_macro_derive(GPUVariant, attributes(name))]
pub fn gpu_variant_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    generate_gpu_variant_impl(&ast)
}


#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    generate_v_attr_ptr_impl(&ast)
}

fn generate_v_attr_ptr_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let name = std::env::var("CARGO_PKG_NAME").unwrap();
    let import = if name != "gl_renderer" {
        quote! {
            use gl_renderer as _gl_renderer;
        }
    } else {
        quote! {
            use crate as _gl_renderer;
        }
    };

    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.data);

    let gen = quote! {
        #import
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl _gl_renderer::VertexAttribPointers for #ident #generics #where_clause {
            fn vertex_attrib_pointers() {
                let stride = ::std::mem::size_of::<Self>();
                let offset = 0;

                #(#fields_vertex_attrib_pointer)*
            }
        }
    };

    gen.into()
}

fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match &data {
        syn::Data::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => panic!("VertexAttribPointers can not be implemented for Unit structs"),
            syn::Fields::Unnamed(_) => panic!("VertexAttribPointers can not be implemented  for Tuple structs"),
            syn::Fields::Named(fields) => fields.named
                .iter()
                .map(generate_vertex_attrib_pointer_call)
                .collect()
        },
        syn::Data::Union(_) => unimplemented!()
    }
}

fn generate_vertex_attrib_pointer_call(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = match &field.ident {
        Some(i) => format!("{}", i),
        None => String::from(""),
    };

    let location_attr = field.attrs
        .iter()
        .filter(|a| a.path.is_ident("location"))
        .next()
        .unwrap_or_else(|| panic!("Field {:?} is missing #[location = ?] attribute", field_name));

    let val_literal = match location_attr.parse_meta().unwrap() {
        syn::Meta::NameValue(val) => val.lit,
        _ => panic!("Field {} location attribute must be an integer literal", field_name),
    };

    let field_ty = &field.ty;
    let gen = quote! {
        let location = #val_literal;
        unsafe {
            #field_ty::vertex_attrib_pointer(stride, location, offset);
        }
        let offset = offset + ::std::mem::size_of::<#field_ty>();
    };

    gen.into()
}