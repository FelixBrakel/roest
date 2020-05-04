use proc_macro::TokenStream;
use syn::spanned::Spanned;
use quote::quote;

pub fn generate_gl_get_impl(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let name = std::env::var("CARGO_PKG_NAME").unwrap();
    let import = if name != "gl_renderer" {
        quote! {
            use gl_renderer as __gl_renderer;
        }
    } else {
        quote! {
            use crate as __gl_renderer;
        }
    };

    let fields_gl_get = generate_gl_getters(&ast.data);
    if fields_gl_get.len() == 0 {
        return quote! {}.into();
    }

    let gen = quote! {
        #import
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl #ident #generics #where_clause {
            #(#fields_gl_get)*
        }
    };

    gen.into()
}

fn generate_gl_getters(data: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match &data {
        syn::Data::Enum(_) => panic!("VertexAttribPointers can not be implemented for enums"),
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => panic!("VertexAttribPointers can not be implemented for Unit structs"),
            syn::Fields::Unnamed(_) => panic!("VertexAttribPointers can not be implemented  for Tuple structs"),
            syn::Fields::Named(fields) => {
                let mut v: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.named.len());
                for field in &fields.named {
                    let gen = generate_gl_get(&field);
                    match gen {
                        Some(g) => v.push(g),
                        None => (),
                    }
                }

                v
            }
        },
        syn::Data::Union(_) => unimplemented!()
    }
}

fn generate_gl_get(field: &syn::Field) -> Option<proc_macro2::TokenStream> {
    let field_name = match &field.ident {
        Some(i) => format!("{}", i),
        None => String::from(""),
    };

    let loc = field.attrs
        .iter()
        .filter(|a| a.path.is_ident("location"))
        .next();

    let location_attr = match loc {
        Some(attr) => attr,
        None => return None
    };

    let val_literal = match location_attr.parse_meta().unwrap() {
        syn::Meta::NameValue(val) => val.lit,
        _ => panic!("Field {} location attribute must be an integer literal", field_name),
    };

    let field_ty = &field.ty;

    let func_ident = syn::Ident::new(format!("gl_get_{}", field_name).as_ref(), field.ident.span());

    let gen = quote! {
        pub fn #func_ident(&self) -> <#field_ty as __gl_renderer::data::ZSTVariant>::Original {
            unsafe {
                #field_ty::gl_get_uniform(&self.gl, &self.program, #val_literal)
            }
        }
    };

    // panic!("gen: {:}", gen);

    Some(gen.into())
}
