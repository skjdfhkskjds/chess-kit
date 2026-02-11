use quote::quote;
use syn::{Data, DeriveInput, Fields, Result, spanned::Spanned};

pub fn expand_arithmetic(input: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "Arithmetic derive only supports non-generic tuple structs",
        ));
    }

    let (inner_ty, _vis) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed.first().unwrap().ty;
                (ty, &input.vis)
            }
            _ => {
                return Err(syn::Error::new(
                    data.struct_token.span(),
                    "Arithmetic derive requires a tuple struct with exactly one field",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "Arithmetic derive can only be used on tuple structs",
            ));
        }
    };

    let name = struct_ident;
    let inner = inner_ty;

    let output = quote! {
        // ================================================
        //              ARITHMETIC OPERATIONS
        // ================================================

        impl ::core::ops::Add<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn add(self, rhs: #inner) -> Self::Output {
                #name(self.0 + rhs)
            }
        }

        impl ::core::ops::Sub<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn sub(self, rhs: #inner) -> Self::Output {
                #name(self.0 - rhs)
            }
        }

        impl ::core::ops::Mul<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn mul(self, rhs: #inner) -> Self::Output {
                #name(self.0 * rhs)
            }
        }
    };

    Ok(output)
}
