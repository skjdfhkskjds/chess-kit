use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, Ident, Result, Type, spanned::Spanned};

use crate::enum_types::utils::{is_primitive_type, parse_repr_primitive, require_fieldless_enum};

enum TargetKind {
    Tuple,
    Enum,
}

struct Target<'a> {
    name: &'a Ident,
    raw_ty: Type,
    kind: TargetKind,
}

impl<'a> Target<'a> {
    fn parse(input: &'a DeriveInput) -> Result<Self> {
        if !input.generics.params.is_empty() {
            return Err(Error::new(
                input.generics.span(),
                "BitOps derive does not support generic types",
            ));
        }

        match &input.data {
            Data::Struct(data) => {
                let Fields::Unnamed(fields) = &data.fields else {
                    return Err(Error::new(
                        data.fields.span(),
                        "BitOps derive requires a tuple struct with exactly one field",
                    ));
                };
                if fields.unnamed.len() != 1 {
                    return Err(Error::new(
                        fields.span(),
                        "BitOps derive requires a tuple struct with exactly one field",
                    ));
                }
                let raw_ty = fields.unnamed.first().unwrap().ty.clone();
                if !is_integer_type(&raw_ty) {
                    return Err(Error::new(
                        raw_ty.span(),
                        "BitOps derive requires an integer tuple field",
                    ));
                }
                Ok(Self {
                    name: &input.ident,
                    raw_ty,
                    kind: TargetKind::Tuple,
                })
            }
            Data::Enum(_) => {
                require_fieldless_enum(input, "BitOps")?;
                Ok(Self {
                    name: &input.ident,
                    raw_ty: parse_repr_primitive(input, "BitOps")?,
                    kind: TargetKind::Enum,
                })
            }
            Data::Union(_) => Err(Error::new(
                input.span(),
                "BitOps derive only supports tuple structs and fieldless enums",
            )),
        }
    }

    fn owned_raw(&self, ident: TokenStream) -> TokenStream {
        let raw_ty = &self.raw_ty;
        match self.kind {
            TargetKind::Tuple => quote! { #ident.0 },
            TargetKind::Enum => quote! { (#ident as #raw_ty) },
        }
    }

    fn ref_raw(&self, ident: TokenStream) -> TokenStream {
        let raw_ty = &self.raw_ty;
        match self.kind {
            TargetKind::Tuple => quote! { #ident.0 },
            TargetKind::Enum => quote! { (*#ident as #raw_ty) },
        }
    }

    fn wrap(&self, value: TokenStream) -> TokenStream {
        let name = self.name;
        match self.kind {
            TargetKind::Tuple => quote! { #name(#value) },
            TargetKind::Enum => quote! { #name::__chess_kit_bitops_from_raw(#value) },
        }
    }

    fn assign(&self, value: TokenStream) -> TokenStream {
        let wrapped = self.wrap(value.clone());
        match self.kind {
            TargetKind::Tuple => quote! { self.0 = #value; },
            TargetKind::Enum => quote! { *self = #wrapped; },
        }
    }

    fn helper(&self) -> TokenStream {
        let name = self.name;
        let raw_ty = &self.raw_ty;
        match self.kind {
            TargetKind::Tuple => quote! {},
            TargetKind::Enum => quote! {
                impl #name {
                    #[inline]
                    const fn __chess_kit_bitops_from_raw(value: #raw_ty) -> Self {
                        unsafe { ::core::mem::transmute::<#raw_ty, Self>(value) }
                    }
                }
            },
        }
    }

    fn conversions(&self) -> TokenStream {
        if matches!(self.kind, TargetKind::Enum) {
            return quote! {};
        }

        let name = self.name;
        let raw_ty = &self.raw_ty;
        let extras = ["u64", "u32", "u8"].into_iter().filter_map(|primitive| {
            if is_primitive_type(raw_ty, primitive) {
                return None;
            }
            let ty = format_ident!("{primitive}");
            Some(quote! {
                impl ::core::convert::From<#ty> for #name {
                    #[inline]
                    fn from(value: #ty) -> Self { #name(value as #raw_ty) }
                }

                impl ::core::convert::From<#name> for #ty {
                    #[inline]
                    fn from(value: #name) -> Self { value.0 as #ty }
                }
            })
        });

        quote! {
            impl ::core::convert::From<#raw_ty> for #name {
                #[inline]
                fn from(value: #raw_ty) -> Self { #name(value) }
            }

            impl ::core::convert::From<#name> for #raw_ty {
                #[inline]
                fn from(value: #name) -> Self { value.0 }
            }

            #(#extras)*

            impl #name {
                #[inline]
                pub const fn const_unwrap(self) -> #raw_ty { self.0 }
            }
        }
    }
}

pub fn expand_bitops(input: &DeriveInput) -> Result<TokenStream> {
    let target = Target::parse(input)?;
    let helper = target.helper();
    let conversions = target.conversions();
    let bitor = generate_binary(
        &target,
        "BitOr",
        "bitor",
        "BitOrAssign",
        "bitor_assign",
        quote!(|),
    );
    let bitand = generate_binary(
        &target,
        "BitAnd",
        "bitand",
        "BitAndAssign",
        "bitand_assign",
        quote!(&),
    );
    let bitxor = generate_binary(
        &target,
        "BitXor",
        "bitxor",
        "BitXorAssign",
        "bitxor_assign",
        quote!(^),
    );
    let not = generate_not(&target);
    let shl = generate_shift(&target, "Shl", "shl", "ShlAssign", "shl_assign", quote!(<<));
    let shr = generate_shift(&target, "Shr", "shr", "ShrAssign", "shr_assign", quote!(>>));

    Ok(quote! {
        #helper
        #bitor
        #bitand
        #not
        #bitxor
        #shl
        #shr
        #conversions
    })
}

fn generate_binary(
    target: &Target<'_>,
    trait_name: &str,
    method_name: &str,
    assign_trait_name: &str,
    assign_method_name: &str,
    op: TokenStream,
) -> TokenStream {
    let name = target.name;
    let raw_ty = &target.raw_ty;
    let trait_ident = format_ident!("{trait_name}");
    let method_ident = format_ident!("{method_name}");
    let assign_trait_ident = format_ident!("{assign_trait_name}");
    let assign_method_ident = format_ident!("{assign_method_name}");
    let self_owned = target.owned_raw(quote!(self));
    let self_ref = target.ref_raw(quote!(self));
    let rhs_owned = target.owned_raw(quote!(rhs));
    let rhs_ref = target.ref_raw(quote!(rhs));
    let owned_owned = target.wrap(quote! { (#self_owned) #op (#rhs_owned) });
    let ref_owned = target.wrap(quote! { (#self_ref) #op (#rhs_owned) });
    let owned_ref = target.wrap(quote! { (#self_owned) #op (#rhs_ref) });
    let owned_raw = target.wrap(quote! { (#self_owned) #op rhs });
    let ref_raw = target.wrap(quote! { (#self_ref) #op rhs });
    let assign_owned = target.assign(quote! { (#self_ref) #op (#rhs_owned) });
    let assign_ref = target.assign(quote! { (#self_ref) #op (#rhs_ref) });
    let assign_raw = target.assign(quote! { (#self_ref) #op rhs });

    quote! {
        impl ::core::ops::#trait_ident for #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #name) -> Self::Output { #owned_owned }
        }
        impl<'a> ::core::ops::#trait_ident<#name> for &'a #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #name) -> Self::Output { #ref_owned }
        }
        impl<'a> ::core::ops::#trait_ident<&'a #name> for #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: &'a #name) -> Self::Output { #owned_ref }
        }
        impl ::core::ops::#trait_ident<#raw_ty> for #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #raw_ty) -> Self::Output { #owned_raw }
        }
        impl<'a> ::core::ops::#trait_ident<#raw_ty> for &'a #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #raw_ty) -> Self::Output { #ref_raw }
        }
        impl ::core::ops::#assign_trait_ident for #name {
            #[inline]
            fn #assign_method_ident(&mut self, rhs: Self) { #assign_owned }
        }
        impl<'a> ::core::ops::#assign_trait_ident<&'a #name> for #name {
            #[inline]
            fn #assign_method_ident(&mut self, rhs: &'a #name) { #assign_ref }
        }
        impl ::core::ops::#assign_trait_ident<#raw_ty> for #name {
            #[inline]
            fn #assign_method_ident(&mut self, rhs: #raw_ty) { #assign_raw }
        }
    }
}

fn generate_not(target: &Target<'_>) -> TokenStream {
    let name = target.name;
    let self_raw = target.owned_raw(quote!(self));
    let output = target.wrap(quote! { !(#self_raw) });
    quote! {
        impl ::core::ops::Not for #name {
            type Output = #name;
            #[inline]
            fn not(self) -> Self::Output { #output }
        }
    }
}

fn generate_shift(
    target: &Target<'_>,
    trait_name: &str,
    method_name: &str,
    assign_trait_name: &str,
    assign_method_name: &str,
    op: TokenStream,
) -> TokenStream {
    let name = target.name;
    let raw_ty = &target.raw_ty;
    let trait_ident = format_ident!("{trait_name}");
    let method_ident = format_ident!("{method_name}");
    let assign_trait_ident = format_ident!("{assign_trait_name}");
    let assign_method_ident = format_ident!("{assign_method_name}");
    let self_owned = target.owned_raw(quote!(self));
    let self_ref = target.ref_raw(quote!(self));
    let rhs_owned = target.owned_raw(quote!(rhs));
    let rhs_ref = target.ref_raw(quote!(rhs));
    let owned_type = target.wrap(quote! { (#self_owned) #op (#rhs_owned) });
    let ref_type = target.wrap(quote! { (#self_ref) #op (#rhs_ref) });
    let assign_type = target.assign(quote! { (#self_ref) #op (#rhs_owned) });
    let raw_impls = generate_shift_rhs(
        target,
        &trait_ident,
        &method_ident,
        &assign_trait_ident,
        &assign_method_ident,
        raw_ty,
        op.clone(),
    );
    let extra_impls = ["u64", "u32", "u8"].into_iter().filter_map(|primitive| {
        if is_primitive_type(raw_ty, primitive) {
            None
        } else {
            let ty_ident = format_ident!("{primitive}");
            let ty: Type = syn::parse_quote!(#ty_ident);
            Some(generate_shift_rhs(
                target,
                &trait_ident,
                &method_ident,
                &assign_trait_ident,
                &assign_method_ident,
                &ty,
                op.clone(),
            ))
        }
    });

    quote! {
        impl ::core::ops::#trait_ident for #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #name) -> Self::Output { #owned_type }
        }
        impl<'a> ::core::ops::#trait_ident for &'a #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: &'a #name) -> Self::Output { #ref_type }
        }
        impl ::core::ops::#assign_trait_ident for #name {
            #[inline]
            fn #assign_method_ident(&mut self, rhs: #name) { #assign_type }
        }
        #raw_impls
        #(#extra_impls)*
    }
}

fn generate_shift_rhs(
    target: &Target<'_>,
    trait_ident: &Ident,
    method_ident: &Ident,
    assign_trait_ident: &Ident,
    assign_method_ident: &Ident,
    rhs_ty: &Type,
    op: TokenStream,
) -> TokenStream {
    let name = target.name;
    let self_owned = target.owned_raw(quote!(self));
    let self_ref = target.ref_raw(quote!(self));
    let owned = target.wrap(quote! { (#self_owned) #op rhs });
    let referenced = target.wrap(quote! { (#self_ref) #op rhs });
    let assign = target.assign(quote! { (#self_ref) #op rhs });
    quote! {
        impl ::core::ops::#trait_ident<#rhs_ty> for #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #rhs_ty) -> Self::Output { #owned }
        }
        impl<'a> ::core::ops::#trait_ident<#rhs_ty> for &'a #name {
            type Output = #name;
            #[inline]
            fn #method_ident(self, rhs: #rhs_ty) -> Self::Output { #referenced }
        }
        impl ::core::ops::#assign_trait_ident<#rhs_ty> for #name {
            #[inline]
            fn #assign_method_ident(&mut self, rhs: #rhs_ty) { #assign }
        }
    }
}

fn is_integer_type(ty: &Type) -> bool {
    [
        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
    ]
    .into_iter()
    .any(|primitive| is_primitive_type(ty, primitive))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn dispatches_supported_shapes() {
        assert!(
            expand_bitops(&parse_quote!(
                struct Flags(u8);
            ))
            .is_ok()
        );
        assert!(
            expand_bitops(&parse_quote!(
                #[repr(u8)]
                enum Mask {
                    Empty,
                    One,
                }
            ))
            .is_ok()
        );
    }

    #[test]
    fn rejects_unsupported_shapes() {
        assert!(
            expand_bitops(&parse_quote!(
                struct Named {
                    value: u8,
                }
            ))
            .is_err()
        );
        assert!(
            expand_bitops(&parse_quote!(
                struct Pair(u8, u8);
            ))
            .is_err()
        );
        assert!(
            expand_bitops(&parse_quote!(
                struct Generic<T>(T);
            ))
            .is_err()
        );
        assert!(
            expand_bitops(&parse_quote!(
                enum MissingRepr {
                    A,
                    B,
                }
            ))
            .is_err()
        );
        assert!(
            expand_bitops(&parse_quote!(
                #[repr(u8)]
                enum Data {
                    A(u8),
                }
            ))
            .is_err()
        );
    }
}
