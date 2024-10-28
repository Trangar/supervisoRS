use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

extern crate proc_macro;
extern crate quote;
extern crate syn;

#[proc_macro_derive(CustomDeserialize, attributes(remaining, serde))]
pub fn custom_deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (_, ty_generics, _) = generics.split_for_impl();

    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("CustomDeserialize can only be derived for structs"),
    };

    let field_names = data
        .fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            if has_remaining_attribute(field) {
                None
            } else {
                Some(field_name)
            }
        })
        .collect::<Vec<_>>();
    if data
        .fields
        .iter()
        .find(|field| has_remaining_attribute(field))
        .is_none()
    {
        return syn::Error::new_spanned(
            input.ident,
            "A field with #[remaining] attribute is required",
        )
        .to_compile_error()
        .into();
    };

    let required_fields = data.fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        if is_optional(field) || has_remaining_attribute(field) {
            None
        } else {
            Some(field_name)
        }
    });

    let field_match_clauses = field_names.iter().map(|field_name| {
        let mut stringified_name = field_name.to_string();
        if stringified_name == "ty" {
            stringified_name = "type".to_string();
        }
        let mut dashed_name = stringified_name.replace("_", "-");

        if stringified_name == "mod" {
            stringified_name = "mod_".to_string();
            dashed_name = stringified_name.clone();
        }

        let name = if dashed_name != stringified_name {
            quote! {
                #dashed_name | #stringified_name
            }
        } else {
            quote! {
                #stringified_name
            }
        };
        quote! {
            #name => {
                #field_name = Some(map.next_value()?);
                missing_fields.remove(stringify!(#field_name));
            }
        }
    });

    let field_assign_clauses = data.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        if has_remaining_attribute(field) {
            quote! {
                #field_name: remaining,
            }
        } else if is_optional(field) {
            quote! {
                #field_name: #field_name,
            }
        } else {
            quote! {
                #field_name: #field_name.unwrap(),
            }
        }
    });

    let expanded = quote! {
        impl<'a, 'de: 'a> serde::de::Deserialize<'de> for #name #ty_generics {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {

                struct StructVisitor<'a>(std::marker::PhantomData<&'a ()>);

                impl<'de: 'a, 'a> serde::de::Visitor<'de> for StructVisitor<'a> {
                    type Value = #name #ty_generics;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("struct #name")
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<#name #ty_generics, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        let mut remaining = FxHashMap::default();
                        let mut missing_fields = FxHashSet::<&'a str>::default();
                        #(let mut #field_names = None;)*
                        #(missing_fields.insert(stringify!(#required_fields));)*

                        while let Some(key) = map.next_key::<&str>()? {
                            match key {
                                #(#field_match_clauses)*
                                _ => {
                                    remaining.insert(key, map.next_value()?);
                                }
                            }
                        }

                        if !missing_fields.is_empty() {
                            return Err(serde::de::Error::custom(format!(
                                "Missing fields {} in struct {}, should {} be made optional? Keys in remaining: {:?}",
                                missing_fields.iter().map(|s| format!("'{s}'")).collect::<Vec<_>>().join(", "), stringify!(#name),
                                if missing_fields.len() == 1 { "this" } else { "these" },
                                remaining.keys().collect::<Vec<_>>(),
                            )));
                        }

                        Ok(#name {
                            #(#field_assign_clauses)*
                        })
                    }
                }

                deserializer.deserialize_map(StructVisitor::<'a>(std::marker::PhantomData))
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_optional(field: &syn::Field) -> bool {
    if let syn::Type::Path(type_path) = &field.ty {
        type_path.path.segments.last().unwrap().ident == "Option"
    } else {
        false
    }
}

fn has_remaining_attribute(field: &syn::Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("remaining") {
            return true;
        }
    }

    false
}
