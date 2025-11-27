use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, Result, Type, TypePath};

pub fn expand_bitops(input: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            input.generics.span(),
            "BitOps derive only supports non-generic tuple structs",
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
                    "BitOps derive requires a tuple struct with exactly one field",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.span(),
                "BitOps derive can only be used on tuple structs",
            ));
        }
    };

    let name = struct_ident;
    let inner = inner_ty;

    let inner_is_u32 = is_primitive_type(inner_ty, "u32");
    let inner_is_u8 = is_primitive_type(inner_ty, "u8");

    let shl_u32_impls = if !inner_is_u32 {
        quote! {
            impl ::core::ops::Shl<u32> for #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u32) -> Self::Output {
                    #name(self.0 << rhs)
                }
            }

            impl<'a> ::core::ops::Shl<u32> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u32) -> Self::Output {
                    #name(self.0 << rhs)
                }
            }

            impl ::core::ops::ShlAssign<u32> for #name {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: u32) {
                    self.0 <<= rhs;
                }
            }
        }
    } else {
        quote! {}
    };

    let shl_u8_impls = if !inner_is_u8 {
        quote! {
            impl ::core::ops::Shl<u8> for #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u8) -> Self::Output {
                    #name(self.0 << rhs)
                }
            }

            impl<'a> ::core::ops::Shl<u8> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u8) -> Self::Output {
                    #name(self.0 << rhs)
                }
            }

            impl ::core::ops::ShlAssign<u8> for #name {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: u8) {
                    self.0 <<= rhs;
                }
            }
        }
    } else {
        quote! {}
    };

    let shr_u32_impls = if !inner_is_u32 {
        quote! {
            impl ::core::ops::Shr<u32> for #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u32) -> Self::Output {
                    #name(self.0 >> rhs)
                }
            }

            impl<'a> ::core::ops::Shr<u32> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u32) -> Self::Output {
                    #name(self.0 >> rhs)
                }
            }

            impl ::core::ops::ShrAssign<u32> for #name {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: u32) {
                    self.0 >>= rhs;
                }
            }
        }
    } else {
        quote! {}
    };

    let shr_u8_impls = if !inner_is_u8 {
        quote! {
            impl ::core::ops::Shr<u8> for #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u8) -> Self::Output {
                    #name(self.0 >> rhs)
                }
            }

            impl<'a> ::core::ops::Shr<u8> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u8) -> Self::Output {
                    #name(self.0 >> rhs)
                }
            }

            impl ::core::ops::ShrAssign<u8> for #name {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: u8) {
                    self.0 >>= rhs;
                }
            }
        }
    } else {
        quote! {}
    };

    let output = quote! {
        // ================================================
        //                      BITOR
        // ================================================
        impl ::core::ops::BitOr for #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #name) -> Self::Output {
                #name(self.0 | rhs.0)
            }
        }

        impl<'a> ::core::ops::BitOr<#name> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #name) -> Self::Output {
                #name(self.0 | rhs.0)
            }
        }

        impl<'a> ::core::ops::BitOr<&'a #name> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: &'a #name) -> Self::Output {
                #name(self.0 | rhs.0)
            }
        }

        impl ::core::ops::BitOr<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #inner) -> Self::Output {
                #name(self.0 | rhs)
            }
        }

        impl<'a> ::core::ops::BitOr<#inner> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #inner) -> Self::Output {
                #name(self.0 | rhs)
            }
        }

        impl ::core::ops::BitOrAssign for #name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl<'a> ::core::ops::BitOrAssign<&'a #name> for #name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: &'a #name) {
                self.0 |= rhs.0;
            }
        }

        impl ::core::ops::BitOrAssign<#inner> for #name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: #inner) {
                self.0 |= rhs;
            }
        }

        // ================================================
        //                      BITAND
        // ================================================

        impl ::core::ops::BitAnd for #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self::Output {
                #name(self.0 & rhs.0)
            }
        }

        impl<'a> ::core::ops::BitAnd<#name> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: #name) -> Self::Output {
                #name(self.0 & rhs.0)
            }
        }

        impl<'a> ::core::ops::BitAnd<&'a #name> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: &'a #name) -> Self::Output {
                #name(self.0 & rhs.0)
            }
        }

        impl ::core::ops::BitAnd<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: #inner) -> Self::Output {
                #name(self.0 & rhs)
            }
        }

        impl<'a> ::core::ops::BitAnd<#inner> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: #inner) -> Self::Output {
                #name(self.0 & rhs)
            }
        }

        impl ::core::ops::BitAndAssign for #name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl<'a> ::core::ops::BitAndAssign<&'a #name> for #name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: &'a #name) {
                self.0 &= rhs.0;
            }
        }

        impl ::core::ops::BitAndAssign<#inner> for #name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: #inner) {
                self.0 &= rhs;
            }
        }

        // ================================================
        //                      BITNOT
        // ================================================

        impl ::core::ops::Not for #name {
            type Output = #name;

            #[inline(always)]
            fn not(self) -> Self::Output {
                #name(!self.0)
            }
        }

        // ================================================
        //                      BITXOR
        // ================================================

        impl ::core::ops::BitXor for #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                #name(self.0 ^ rhs.0)
            }
        }

        impl<'a> ::core::ops::BitXor<#name> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: #name) -> Self::Output {
                #name(self.0 ^ rhs.0)
            }
        }

        impl<'a> ::core::ops::BitXor<&'a #name> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: &'a #name) -> Self::Output {
                #name(self.0 ^ rhs.0)
            }
        }

        impl ::core::ops::BitXor<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: #inner) -> Self::Output {
                #name(self.0 ^ rhs)
            }
        }

        impl<'a> ::core::ops::BitXor<#inner> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: #inner) -> Self::Output {
                #name(self.0 ^ rhs)
            }
        }

        impl ::core::ops::BitXorAssign for #name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl<'a> ::core::ops::BitXorAssign<&'a #name> for #name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: &'a #name) {
                self.0 ^= rhs.0;
            }
        }

        impl ::core::ops::BitXorAssign<#inner> for #name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: #inner) {
                self.0 ^= rhs;
            }
        }

        // ================================================
        //                   BITSHIFT LEFT
        // ================================================

        impl ::core::ops::Shl for #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: #name) -> Self::Output {
                #name(self.0 << rhs.0)
            }
        }

        impl<'a> ::core::ops::Shl for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: &'a #name) -> Self::Output {
                #name(self.0 << rhs.0)
            }
        }

        impl ::core::ops::ShlAssign for #name {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: #name) {
                self.0 <<= rhs.0;
            }
        }

        impl ::core::ops::Shl<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: #inner) -> Self::Output {
                #name(self.0 << rhs)
            }
        }

        impl<'a> ::core::ops::Shl<#inner> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: #inner) -> Self::Output {
                #name(self.0 << rhs)
            }
        }

        impl ::core::ops::ShlAssign<#inner> for #name {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: #inner) {
                self.0 <<= rhs;
            }
        }

        #shl_u32_impls
        #shl_u8_impls

        // ================================================
        //                   BITSHIFT RIGHT
        // ================================================

        impl ::core::ops::Shr for #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: #name) -> Self::Output {
                #name(self.0 >> rhs.0)
            }
        }

        impl<'a> ::core::ops::Shr for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: &'a #name) -> Self::Output {
                #name(self.0 >> rhs.0)
            }
        }

        impl ::core::ops::ShrAssign for #name {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: #name) {
                self.0 >>= rhs.0;
            }
        }

        impl ::core::ops::Shr<#inner> for #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: #inner) -> Self::Output {
                #name(self.0 >> rhs)
            }
        }

        impl<'a> ::core::ops::Shr<#inner> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: #inner) -> Self::Output {
                #name(self.0 >> rhs)
            }
        }

        impl ::core::ops::ShrAssign<#inner> for #name {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: #inner) {
                self.0 >>= rhs;
            }
        }

        #shr_u32_impls
        #shr_u8_impls
    };

    Ok(output)
}

fn is_primitive_type(ty: &Type, name: &str) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(ident) = path.get_ident() {
            return ident == name;
        }
    }
    false
}
