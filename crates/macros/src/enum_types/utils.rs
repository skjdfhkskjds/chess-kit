use syn::{
    Data, DataEnum, DeriveInput, Error, Fields, Meta, Result, Token, Type, TypePath,
    punctuated::Punctuated, spanned::Spanned,
};

pub fn require_fieldless_enum<'a>(
    input: &'a DeriveInput,
    macro_name: &str,
) -> Result<&'a DataEnum> {
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Err(Error::new(
                input.span(),
                format!("{macro_name} derive only works with enums"),
            ));
        }
    };

    for variant in &data_enum.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(Error::new(
                variant.fields.span(),
                format!("{macro_name} derive requires all variants to be fieldless"),
            ));
        }
    }

    Ok(data_enum)
}

pub fn parse_repr_primitive(input: &DeriveInput, macro_name: &str) -> Result<Type> {
    for attr in &input.attrs {
        if attr.path().is_ident("repr") {
            if let Meta::List(meta_list) = attr.meta.clone() {
                let nested =
                    meta_list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

                for meta in nested {
                    if let Meta::Path(path) = meta {
                        if let Some(ident) = path.get_ident() {
                            if is_allowed_repr_ident(ident) {
                                return Ok(Type::Path(TypePath { qself: None, path }));
                            }
                        }
                    }
                }
            }
        }
    }

    Err(Error::new(
        input.span(),
        format!("{macro_name} derive requires #[repr(u8|u16|u32|u64|u128|usize)]"),
    ))
}

pub fn is_primitive_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            return ident == name;
        }
    }
    false
}

fn is_allowed_repr_ident(ident: &syn::Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
    )
}
