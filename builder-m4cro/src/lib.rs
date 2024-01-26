extern crate proc_macro;
use std::any::Any;

use proc_macro::{TokenStream, Ident};
extern crate syn;
#[macro_use]
extern crate quote;
use syn::{parse_macro_input, DeriveInput, Attribute, Data};

#[proc_macro_derive(Builder)]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    
    // Parse the string representation
    let  DeriveInput { ident, attrs, generics, data,.. } = parse_macro_input!(item);

    let builder_name = format_ident!("{}Builder", ident);

    let builder_struct = match &data {
        syn::Data::Struct(data_struct) => {
            let fields = data_struct.fields.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! { #field_name: Option<#field_type>, }
            });

            quote! {
                #[derive(Default, Debug)]
                pub struct #builder_name {
                    #(#fields)*
                }
            }
        }
        _ => panic!("Builder macro only supports structs"),
    };

    // Generate code for the builder methods
    let builder_methods = match &data {
        syn::Data::Struct(data_struct) => {
            let methods = data_struct.fields.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! {
                    pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
            });

            let build_mandatory = data_struct.fields.iter()
            .filter(|field| { 
                let field_type = &field.ty;
                if let syn::Type::Path(type_path) = field_type {
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident.to_string() == "Option" {
                            return false;
                        }
                    }
                };
                return true;
             })
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    #field_name: self.#field_name.clone().ok_or_else(|| format!("{} is not set", stringify!(#field_name)))?,
                }
            });

            let build_optional = data_struct.fields.iter()
                .filter(|field| { 
                    let field_type = &field.ty;
                    if let syn::Type::Path(type_path) = field_type {
                        if let Some(segment) = type_path.path.segments.last() {
                            if segment.ident.to_string() == "Option" {
                                return true;
                            }
                        }
                    };
                    return false;
                 })
                .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    #field_name: self.#field_name.clone().unwrap_or_else(|| None),
                }
            });

            quote! {
                impl #builder_name {
                    pub fn new() -> #builder_name {
                        #builder_name::default()
                    }

                    #(#methods)*

                    pub fn build(&self) -> Result<#ident, String> {
                        Ok(#ident {
                            #(#build_mandatory)*
                            #(#build_optional)*
                        })
                    }
                }
            }
        }
        _ => panic!("Builder macro only supports structs"),
    };

    // Build the impl
    let output = quote! {
        impl #ident {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }

        #builder_struct

        #builder_methods
    };

    // Return the generated impl
    output.into()
}



#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
    
    }
}
