//! Provides a `JatsCodec` derive macro for structs and enums in Stencila Schema

use std::collections::HashMap;

use darling::{self, FromDeriveInput, FromField};

use common::{
    proc_macro2::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields, Ident}, itertools::Itertools,
};

#[derive(FromDeriveInput)]
#[darling(attributes(jats))]
struct TypeAttr {
    ident: Ident,
    data: darling::ast::Data<darling::util::Ignored, FieldAttr>,

    #[darling(default)]
    elem: Option<String>,

    #[darling(default)]
    attribs: HashMap<String, String>,

    #[darling(default)]
    special: bool,
}

#[derive(FromField)]
#[darling(attributes(jats))]
struct FieldAttr {
    ident: Option<Ident>,

    #[darling(default)]
    elem: Option<String>,

    #[darling(default)]
    attr: Option<String>,

    #[darling(default)]
    content: bool,

    #[darling(default)]
    flatten: bool,
}

/// Derive the `JatsCodec` trait for a `struct` or an `enum`
#[proc_macro_derive(JatsCodec, attributes(jats))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro::TokenStream;

    let input = parse_macro_input!(input as DeriveInput);

    let attr = match TypeAttr::from_derive_input(&input) {
        Ok(value) => value,
        Err(error) => {
            return TokenStream::from(error.write_errors());
        }
    };

    let tokens = match &input.data {
        Data::Struct(..) => derive_struct(attr),
        Data::Enum(data) => derive_enum(attr, data),
        Data::Union(..) => return TokenStream::new(),
    };

    TokenStream::from(tokens)
}

/// Derive the `JatsCodec` trait for a `struct`
fn derive_struct(type_attr: TypeAttr) -> TokenStream {
    let struct_name = type_attr.ident;

    if type_attr.special {
        return quote! {
            impl JatsCodec for #struct_name {
                fn to_jats(&self) -> (String, Losses) {
                    self.to_jats_special()
                }

                fn to_jats_parts(&self) -> (String, Vec<(String, String)>, String, Losses) {
                    let (content, losses) = self.to_jats_special();
                    (String::new(), Vec::new(), content, losses) 
                }
            }
        };
    }

    let Some(elem) = type_attr.elem else {
        return quote! {
            impl JatsCodec for #struct_name {
                fn to_jats_parts(&self) -> (String, Vec<(String, String)>, String, Losses) {
                    (String::new(), Vec::new(), String::new(), Losses::of_all(stringify!(#struct_name)))
                } 
            }
        }
    };

    let mut attrs = TokenStream::new();
    for (name, value) in type_attr.attribs.iter().sorted() {
        let name = name.replace("__","-").replace("_",":");
        attrs.extend(quote!{
            (#name.to_string(), #value.to_string()),
        })
    }

    let mut fields = TokenStream::new();
    type_attr.data.map_struct_fields(|field_attr| {
        let Some(field_name) = field_attr.ident else {
            return
        };

        if field_name == "r#type" {
            return;
        }

        let field_tokens = if field_attr.flatten {
            quote! {
                let mut parts = self.#field_name.to_jats_parts();
                attrs.append(&mut parts.1);
                content.push_str(&parts.2);
                losses.append(&mut parts.3);
            }
        } else if let Some(attr) = field_attr.attr {
            quote! {
                let (field_text, mut field_losses) = self.#field_name.to_text();
                attrs.push((#attr.to_string(), field_text));
                losses.append(&mut field_losses);
            }
        } else if let Some(elem) = field_attr.elem {
            quote! {
                let (field_jats, mut field_losses) = self.#field_name.to_jats();
                if !field_jats.is_empty() {
                    content.push_str(&elem(#elem, [], field_jats));
                }
                losses.append(&mut field_losses);
            }
        } else if field_name == "content" || field_attr.content {
            quote! {
                let (field_jats, mut field_losses) = self.#field_name.to_jats();
                content.push_str(&field_jats);
                losses.append(&mut field_losses);
            }
        } else {
            quote! {
                losses.push(Loss::of_property(
                    LossDirection::Encode, stringify!(#struct_name), stringify!(#field_name)
                ));
            }
        };
        fields.extend(field_tokens)
    });

    quote! {
        impl JatsCodec for #struct_name {
            fn to_jats_parts(&self) -> (String, Vec<(String, String)>, String, Losses) {
                use codec_jats_trait::encode::elem;
                
                let mut attrs = vec![#attrs];
                let mut content = String::new();
                let mut losses = Losses::none();

                #fields

                (#elem.to_string(), attrs, content, losses)
            }
        }
    }
}

/// Derive the `JatsCodec` trait for an `enum`
fn derive_enum(type_attr: TypeAttr, data: &DataEnum) -> TokenStream {
    let enum_name = type_attr.ident;

    let mut variants_to_jats = TokenStream::new();
    for variant in &data.variants {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Named(..) | Fields::Unnamed(..) => {
                variants_to_jats.extend(quote! {
                    Self::#variant_name(v) => v.to_jats_parts(),
                });
            }
            Fields::Unit => {
                variants_to_jats.extend(quote! {
                    Self::#variant_name => (
                        String::new(), Vec::new(), stringify!(#variant_name).to_string(), Losses::none()
                    ),
                });
            }
        };
    }

    quote! {
        impl JatsCodec for #enum_name {
            fn to_jats_parts(&self) -> (String, Vec<(String, String)>, String, Losses) {
                match self {
                    #variants_to_jats
                }
            }
        }
    }
}