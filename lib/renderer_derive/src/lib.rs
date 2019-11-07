#![recursion_limit="128"]

extern crate proc_macro;

use syn;
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_derive(VertexAttribPointers, attributes())]
pub fn vertex_attrib_pointers_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
//    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    generate_impl(&ast)
}

fn generate_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

//    panic!("code = {:#}", quote! {
//        impl #ident #generics #where_clause {
//            pub fn vertex_attrib_pointers(gl: &gl::Gl) {
//                let stride = std::mem::size_of::<Self>();
//                let offset = 0;
//
//                let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.);
//                #(#fields_vertex_attrib_pointer)*
//            }
//        }
//    });

    let gen = quote! {
        impl Vertex {
            fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>();

                let location = 0;
                let offset = 0;

                unsafe {
                    data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset)
                }

                let location = 1;
                let offset = offset + std::mem::size_of::<data::f32_f32_f32>();
                unsafe {
                    data::f32_f32_f32::vertex_attrib_pointer(gl, stride, location, offset)
                }
            }
        }
    };

    gen.into()
}

//fn generate_vertex_attrib_pointer_calls(body: &syn::Body) -> Vec<quote::Tokens> {
//    match body {
//        &syn::Body::Enum(_)
//            => panic!("")
//        &syn::
//    }
//}