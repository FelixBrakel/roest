#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;

use syn;
//use quote;
use quote::quote;
use proc_macro::TokenStream;
//use core::panicking::panic_fmt;

#[proc_macro_derive(VertexAttribPointers, attributes())]
pub fn vertex_attrib_pointers_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
//    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    generate_impl(&ast)
}

fn generate_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

//    panic!("{:#?}", ast);

    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.data);

    let gen = quote! {
        impl #ident #generics #where_clause {
            pub fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>();
                let offset = 0;

                #(#fields_vertex_attrib_pointer)*
            }
        }
    };

//    let gen = quote! {
//        impl Vertex {
//            fn vertex_attrib_pointers(gl: &gl::Gl) {
//                let stride = std::mem::size_of::<Self>();
//
//                let location = 0;
//                let offset = 0;
//
//                unsafe {
//                    data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset)
//                }
//
//                let location = 1;
//                let offset = offset + std::mem::size_of::<data::f32_f32_f32>();
//                unsafe {
//                    data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset)
//                }
//            }
//        }
//    };

    panic!("CODE: {:#?}", gen);

    gen.into()
}

fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match &data {
        syn::Data::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => panic!("VertexAttribPointers can not be implemented for Unit structs"),
            syn::Fields::Unnamed(_) => panic!("VertexAttribPointers can not be implemented  for Tuple structs"),
            syn::Fields::Named(fields) => fields.named.iter().map(generate_vertex_attrib_pointer_call).collect()
        },
        syn::Data::Union(_) => unimplemented!()
    }
}

fn generate_vertex_attrib_pointer_call(field: &syn::Field) -> proc_macro2::TokenStream {
//    panic!("{:#?}", field);
    let field_name = match &field.ident {
        Some(i) => format!("{}", i),
        None => String::from(""),
    };

    let location_attr = field.attrs
        .iter()
        .filter(|a| a.path.is_ident("location"))
        .next()
        .unwrap_or_else(|| panic!("Field {:?} is missing #[location = ?] attribute", field_name));
//    panic!("ATTR: {:#?}", location_attr);

    let val_literal = match ocation_attr.parse_meta().unwrap() {
        syn::Meta::NameValue(ref lit @ syn::Lit::Int(_)) => lit,
        _ => panic!("Field {} location attribute must be an iteger literal", field_name),
    };

    quote!(imple A {}).into()
}