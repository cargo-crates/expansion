type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

pub struct DeriveInputHelper {
    value: syn::DeriveInput
}

impl DeriveInputHelper {
    // pub type AttributeArgs = Vec<NestedMeta>;
    pub fn new(value: syn::DeriveInput) -> Self { Self { value } }
    pub fn value(&self) -> &syn::DeriveInput { &self.value }
    pub fn get_fields(&self) -> syn::Result<&StructFields> {
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: syn::Fields::Named(
                    syn::FieldsNamed {
                        ref named,
                        ..
                    }
                ),
                ..
            }
        ) = self.value().data {
            Ok(named)
        } else {
            Err(syn::Error::new_spanned(self.value(), "Must Define On Struct, Not Allow On Enum Or Union".to_string()))
        }
    }
}

