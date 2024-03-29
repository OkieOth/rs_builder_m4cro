extern crate proc_macro;

use proc_macro::TokenStream;

extern crate syn;
#[macro_use]
extern crate quote;
use syn::{parse_macro_input, DeriveInput};

fn filter_option(field: &&syn::Field, is_optional: bool) -> bool {
    let field_type = &field.ty;
    if let syn::Type::Path(type_path) = field_type {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident.to_string() == "Option" {
                return is_optional;
            } else {
                return ! is_optional;
            }
        }
    };
    return ! is_optional;
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    
    // Parse the string representation
    let  DeriveInput { ident, data,.. } = parse_macro_input!(item);

    let builder_name = format_ident!("{}Builder", ident);

    let builder_struct = match &data {
        syn::Data::Struct(data_struct) => {
            let fields_mandatory = data_struct.fields.iter()
            .filter(|field| { 
                filter_option(field, false)
             }) 
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! { #field_name: Option<#field_type>, }
            });
            let fields_optional = data_struct.fields.iter()
            .filter(|field| { 
                filter_option(field, true)
             })
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                quote! { #field_name: #field_type, }
            });

            quote! {
                #[derive(Default, Debug)]
                pub struct #builder_name {
                    #(#fields_mandatory)*
                    #(#fields_optional)*
                }
            }
        }
        _ => panic!("Builder macro only supports structs"),
    };

    // Generate code for the builder methods
    let builder_methods = match &data {
        syn::Data::Struct(data_struct) => {
            let methods_mandatory = data_struct.fields.iter()
            .filter(|field| { 
                filter_option(field, false)
             })            
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();

                let field_type = &field.ty;
                quote! {
                    pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
            });

            let methods_optional = data_struct.fields.iter()
            .filter(|field| { 
                filter_option(field, true)
             })
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let mut field_type = &field.ty;

                if let syn::Type::Path(type_path) = field_type {
                    if let Some(segment) = type_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(arg) = args.args.iter().next() {
                                if let syn::GenericArgument::Type(inner_type) = arg {
                                    // At this point, 'inner_type' represents the inner type T
                                    field_type = inner_type;
                                }
                            }
                        }
                    }
                };

                quote! {
                    pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                        self.#field_name = Some(#field_name);
                        self
                    }
                }
            });


            let build_mandatory = data_struct.fields.iter()
            .filter(|field| { 
                filter_option(field, false)
             })
            .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    #field_name: self.#field_name.clone().ok_or_else(|| format!("{} is not set", stringify!(#field_name)))?,
                }
            });

            let build_optional = data_struct.fields.iter()
                .filter(|field| { 
                    filter_option(field, true)
                })
                .map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                quote! {
                    #field_name: self.#field_name.clone(),
                }
            });

            quote! {
                impl #builder_name {
                    pub fn new() -> #builder_name {
                        #builder_name::default()
                    }

                    #(#methods_mandatory)*
                    #(#methods_optional)*

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
