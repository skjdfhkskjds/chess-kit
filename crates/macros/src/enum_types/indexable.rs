use super::utils::{parse_repr_primitive, require_fieldless_enum};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Error, Result};

pub fn expand_indexable_enum(input: &DeriveInput) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "IndexableEnum derive does not support generic enums",
        ));
    }

    let data_enum = require_fieldless_enum(input, "IndexableEnum")?;
    let repr_ty = parse_repr_primitive(input, "IndexableEnum")?;

    for variant in &data_enum.variants {
        if variant.discriminant.is_some() {
            return Err(Error::new(
                variant.ident.span(),
                "IndexableEnum derive requires implicit discriminants starting at 0",
            ));
        }
    }

    let variant_count = syn::Index::from(data_enum.variants.len());
    let name = &input.ident;

    let output = quote! {
        impl #name {
            #[inline(always)]
            pub const fn idx(self) -> usize {
                self as usize
            }

            #[inline(always)]
            pub const fn from_idx(idx: usize) -> Self {
                unsafe { ::core::mem::transmute::<#repr_ty, Self>(idx as #repr_ty) }
            }

            #[inline(always)]
            pub const fn from_idx_safe(idx: usize) -> ::core::option::Option<Self> {
                if idx < #variant_count {
                    Some(unsafe { ::core::mem::transmute::<#repr_ty, Self>(idx as #repr_ty) })
                } else {
                    None
                }
            }
        }


        impl<T, const N: usize> ::std::ops::Index<#name> for [T; N] {
            type Output = T;

            #[inline(always)]
            fn index(&self, index: #name) -> &Self::Output {
                &self[index as usize]
            }
        }

        impl<T, const N: usize> ::std::ops::IndexMut<#name> for [T; N] {
            #[inline(always)]
            fn index_mut(&mut self, index: #name) -> &mut Self::Output {
                &mut self[index as usize]
            }
        }
    };

    Ok(output)
}
