use super::utils::{is_primitive_type, parse_repr_primitive, require_fieldless_enum};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Error, Result};

pub fn expand_enum_bitops(input: &DeriveInput) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "EnumBitOps derive does not support generic enums",
        ));
    }

    require_fieldless_enum(input, "EnumBitOps")?;
    let repr_ty = parse_repr_primitive(input, "EnumBitOps")?;

    let name = &input.ident;
    let repr_is_u32 = is_primitive_type(&repr_ty, "u32");
    let repr_is_u8 = is_primitive_type(&repr_ty, "u8");

    let helper_impl = quote! {
        impl #name {
            #[inline(always)]
            fn __chess_kit_enum_bitops_from_raw(value: #repr_ty) -> Self {
                unsafe { ::core::mem::transmute::<#repr_ty, Self>(value) }
            }
        }
    };

    let shl_u32_impls = if !repr_is_u32 {
        quote! {
            impl ::core::ops::Shl<u32> for #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u32) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) << rhs)
                }
            }

            impl<'a> ::core::ops::Shl<u32> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u32) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << rhs)
                }
            }

            impl ::core::ops::ShlAssign<u32> for #name {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: u32) {
                    *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << rhs);
                }
            }
        }
    } else {
        quote! {}
    };

    let shl_u8_impls = if !repr_is_u8 {
        quote! {
            impl ::core::ops::Shl<u8> for #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u8) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) << rhs)
                }
            }

            impl<'a> ::core::ops::Shl<u8> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shl(self, rhs: u8) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << rhs)
                }
            }

            impl ::core::ops::ShlAssign<u8> for #name {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: u8) {
                    *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << rhs);
                }
            }
        }
    } else {
        quote! {}
    };

    let shr_u32_impls = if !repr_is_u32 {
        quote! {
            impl ::core::ops::Shr<u32> for #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u32) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) >> rhs)
                }
            }

            impl<'a> ::core::ops::Shr<u32> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u32) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> rhs)
                }
            }

            impl ::core::ops::ShrAssign<u32> for #name {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: u32) {
                    *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> rhs);
                }
            }
        }
    } else {
        quote! {}
    };

    let shr_u8_impls = if !repr_is_u8 {
        quote! {
            impl ::core::ops::Shr<u8> for #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u8) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) >> rhs)
                }
            }

            impl<'a> ::core::ops::Shr<u8> for &'a #name {
                type Output = #name;

                #[inline(always)]
                fn shr(self, rhs: u8) -> Self::Output {
                    #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> rhs)
                }
            }

            impl ::core::ops::ShrAssign<u8> for #name {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: u8) {
                    *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> rhs);
                }
            }
        }
    } else {
        quote! {}
    };

    let output = quote! {
        #helper_impl

        // ================================================
        //                      BITOR
        // ================================================
        impl ::core::ops::BitOr for #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) | (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::BitOr<#name> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) | (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::BitOr<&'a #name> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: &'a #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) | (*rhs as #repr_ty))
            }
        }

        impl ::core::ops::BitOr<#repr_ty> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) | rhs)
            }
        }

        impl<'a> ::core::ops::BitOr<#repr_ty> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitor(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) | rhs)
            }
        }

        impl ::core::ops::BitOrAssign for #name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: Self) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) | (rhs as #repr_ty));
            }
        }

        impl<'a> ::core::ops::BitOrAssign<&'a #name> for #name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: &'a #name) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) | (*rhs as #repr_ty));
            }
        }

        impl ::core::ops::BitOrAssign<#repr_ty> for #name {
            #[inline(always)]
            fn bitor_assign(&mut self, rhs: #repr_ty) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) | rhs);
            }
        }

        // ================================================
        //                      BITAND
        // ================================================
        impl ::core::ops::BitAnd for #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: Self) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) & (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::BitAnd<#name> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) & (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::BitAnd<&'a #name> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: &'a #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) & (*rhs as #repr_ty))
            }
        }

        impl ::core::ops::BitAnd<#repr_ty> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) & rhs)
            }
        }

        impl<'a> ::core::ops::BitAnd<#repr_ty> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitand(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) & rhs)
            }
        }

        impl ::core::ops::BitAndAssign for #name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: Self) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) & (rhs as #repr_ty));
            }
        }

        impl<'a> ::core::ops::BitAndAssign<&'a #name> for #name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: &'a #name) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) & (*rhs as #repr_ty));
            }
        }

        impl ::core::ops::BitAndAssign<#repr_ty> for #name {
            #[inline(always)]
            fn bitand_assign(&mut self, rhs: #repr_ty) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) & rhs);
            }
        }

        // ================================================
        //                      BITNOT
        // ================================================
        impl ::core::ops::Not for #name {
            type Output = #name;

            #[inline(always)]
            fn not(self) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw(!(self as #repr_ty))
            }
        }

        // ================================================
        //                      BITXOR
        // ================================================
        impl ::core::ops::BitXor for #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) ^ (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::BitXor<#name> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) ^ (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::BitXor<&'a #name> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: &'a #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) ^ (*rhs as #repr_ty))
            }
        }

        impl ::core::ops::BitXor<#repr_ty> for #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) ^ rhs)
            }
        }

        impl<'a> ::core::ops::BitXor<#repr_ty> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn bitxor(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) ^ rhs)
            }
        }

        impl ::core::ops::BitXorAssign for #name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) ^ (rhs as #repr_ty));
            }
        }

        impl<'a> ::core::ops::BitXorAssign<&'a #name> for #name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: &'a #name) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) ^ (*rhs as #repr_ty));
            }
        }

        impl ::core::ops::BitXorAssign<#repr_ty> for #name {
            #[inline(always)]
            fn bitxor_assign(&mut self, rhs: #repr_ty) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) ^ rhs);
            }
        }

        // ================================================
        //                   BITSHIFT LEFT
        // ================================================
        impl ::core::ops::Shl for #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) << (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::Shl for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: &'a #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << (*rhs as #repr_ty))
            }
        }

        impl ::core::ops::ShlAssign for #name {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: #name) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << (rhs as #repr_ty));
            }
        }

        impl ::core::ops::Shl<#repr_ty> for #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) << rhs)
            }
        }

        impl<'a> ::core::ops::Shl<#repr_ty> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shl(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << rhs)
            }
        }

        impl ::core::ops::ShlAssign<#repr_ty> for #name {
            #[inline(always)]
            fn shl_assign(&mut self, rhs: #repr_ty) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) << rhs);
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
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) >> (rhs as #repr_ty))
            }
        }

        impl<'a> ::core::ops::Shr for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: &'a #name) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> (*rhs as #repr_ty))
            }
        }

        impl ::core::ops::ShrAssign for #name {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: #name) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> (rhs as #repr_ty));
            }
        }

        impl ::core::ops::Shr<#repr_ty> for #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((self as #repr_ty) >> rhs)
            }
        }

        impl<'a> ::core::ops::Shr<#repr_ty> for &'a #name {
            type Output = #name;

            #[inline(always)]
            fn shr(self, rhs: #repr_ty) -> Self::Output {
                #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> rhs)
            }
        }

        impl ::core::ops::ShrAssign<#repr_ty> for #name {
            #[inline(always)]
            fn shr_assign(&mut self, rhs: #repr_ty) {
                *self = #name::__chess_kit_enum_bitops_from_raw((*self as #repr_ty) >> rhs);
            }
        }

        #shr_u32_impls
        #shr_u8_impls
    };

    Ok(output)
}

