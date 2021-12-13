mod derive_input_helper;

pub use derive_input_helper::DeriveInputHelper;

use syn::{spanned::Spanned};

// eg: Option<T>; get_generic_inner_type(&f.ty, "Option") -> Some(T: syn::Type)
// eg: Vec<T>; get_generic_inner_type(&f.ty, "Vec") -> Some(T: syn::Type)
pub fn get_type_inner_type_ident<'a>(r#type: &'a syn::Type, type_ident_name: &str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(
        syn::TypePath {
            path: syn::Path {
                segments,
                ..
            },
            ..
        }
    ) = r#type {
        if let Some(seg) = segments.last() {
            if seg.ident.to_string() == type_ident_name {
                if let syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }
                ) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}


pub fn parse_attrs_to_metas(attrs: &Vec<syn::Attribute>) -> syn::Result<Vec<syn::Meta>> {
    attrs.iter().map(|attr| attr.parse_meta()).collect::<syn::Result<Vec<syn::Meta>>>()
}

pub fn get_macro_nested_attr_value_ident(nested_metas: Vec<&syn::NestedMeta>, attr_name: &str, namespaces: Option<Vec<&str>>, allow_attrs: Option<Vec<&str>>) -> syn::Result<Option<syn::Ident>> {
    match namespaces {
        Some(namespaces) => {
            if namespaces.len() != 0 {
                let metas = nested_metas.iter().filter_map(|item| {
                    if let syn::NestedMeta::Meta(meta) = item { Some(meta) } else { None }
                }).collect::<Vec<_>>();
                if let Some(ident) = get_macro_attr_value_ident(metas, attr_name, Some(namespaces), allow_attrs)? {
                    return Ok(Some(ident))
                }
            } else {
                for nested_meta in nested_metas.iter() {
                    if let syn::NestedMeta::Meta(meta) = nested_meta {
                        let allow_attrs = if let Some(allow_attrs) = &allow_attrs { Some(allow_attrs.iter().map(|&i| i).collect()) } else { None };
                        if let Some(ident) = get_macro_attr_value_from_meta(meta, attr_name, allow_attrs)? {
                            return Ok(Some(ident))
                        }
                    }
                }
            }
        },
        None => {
            return get_macro_nested_attr_value_ident(nested_metas, attr_name, Some(vec![]), allow_attrs);
        },
    }
    Ok(None)
}
pub fn get_macro_attr_value_ident(metas: Vec<&syn::Meta>, attr_name: &str, namespaces: Option<Vec<&str>>, namespace_allow_attrs: Option<Vec<&str>>) -> syn::Result<Option<syn::Ident>> {
    match &namespaces {
        Some(namespaces) => {
            // eg: #[arel="users"]
            if namespaces.len() == 0 {
                for meta in metas {
                    let namespace_allow_attrs = if let Some(namespace_allow_attrs) = &namespace_allow_attrs { Some(namespace_allow_attrs.iter().map(|&i| i).collect()) } else { None };
                    if let Some(ident) = get_macro_attr_value_from_meta(&meta, attr_name, namespace_allow_attrs)? {
                        return Ok(Some(ident))
                    }
                }
            } else { // eg: #[arel(name="uuid")], #[arel(column(name="uuid"))]
                for meta in metas {
                    if let  syn::Meta::List(meta_list) = meta {
                        let mut namespaces: Vec<&str> = namespaces.iter().map(|&i| i).collect();
                        for segment in meta_list.path.segments.iter() {
                            let first_namespace = namespaces.remove(0);
                            if segment.ident == first_namespace {
                                let nested_metas = meta_list.nested.iter().map(|i| i).collect();
                                let namespace_allow_attrs = if let Some(namespace_allow_attrs) = &namespace_allow_attrs { Some(namespace_allow_attrs.iter().map(|&i| i).collect()) } else { None };
                                if let Some(ident) = get_macro_nested_attr_value_ident(nested_metas, attr_name, Some(namespaces.iter().map(|&i| i).collect()), namespace_allow_attrs)? {
                                    return Ok(Some(ident))
                                }
                            }
                        }
                    }
                }
            }
        },
        _ => {
            return get_macro_attr_value_ident(metas, attr_name, Some(vec![]), namespace_allow_attrs);
        },
    }
    Ok(None)
}

fn get_macro_attr_value_from_meta(meta: &syn::Meta, attr_name: &str, allow_attrs: Option<Vec<&str>>) -> syn::Result<Option<syn::Ident>> {
    if let syn::Meta::NameValue(kv) = meta {
        if let syn::Lit::Str(ref ident_str) = kv.lit {
            // 校验
            if let Some(allow_attrs) = &allow_attrs {
                for attr_name in allow_attrs {
                    if !kv.path.is_ident(attr_name) {
                        return Err(syn::Error::new_spanned(&kv.path, format!("Support Macro Attr List: {:?}", &allow_attrs)));
                    }
                }
            }
            if kv.path.is_ident(attr_name) {
                return Ok(Some(syn::Ident::new(ident_str.value().as_str(), kv.span())));
            }
        }
    }
    Ok(None)
}

