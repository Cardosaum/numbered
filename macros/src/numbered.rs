//! `#[derive(Numbered)]` implementation.

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::BTreeMap;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, ExprUnary, Fields, Ident, Lit, Result, Token, UnOp,
};

const REPR_HELP: &str = "u8|u16|u32|u64|usize|i8|i16|i32|i64|isize";

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReprType {
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
}

impl ReprType {
    fn from_name(s: &str) -> Option<Self> {
        Some(match s {
            "u8" => Self::U8,
            "u16" => Self::U16,
            "u32" => Self::U32,
            "u64" => Self::U64,
            "usize" => Self::Usize,
            "i8" => Self::I8,
            "i16" => Self::I16,
            "i32" => Self::I32,
            "i64" => Self::I64,
            "isize" => Self::Isize,
            _ => return None,
        })
    }

    fn name(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Usize => "usize",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::Isize => "isize",
        }
    }

    fn min_max(self) -> (i128, i128) {
        match self {
            Self::U8 => (i128::from(u8::MIN), i128::from(u8::MAX)),
            Self::U16 => (i128::from(u16::MIN), i128::from(u16::MAX)),
            Self::U32 => (i128::from(u32::MIN), i128::from(u32::MAX)),
            Self::U64 => (i128::from(u64::MIN), i128::from(u64::MAX)),
            Self::Usize => (0, i128::from(u64::MAX)),
            Self::I8 => (i128::from(i8::MIN), i128::from(i8::MAX)),
            Self::I16 => (i128::from(i16::MIN), i128::from(i16::MAX)),
            Self::I32 => (i128::from(i32::MIN), i128::from(i32::MAX)),
            Self::I64 => (i128::from(i64::MIN), i128::from(i64::MAX)),
            Self::Isize => (i128::from(i64::MIN), i128::from(i64::MAX)),
        }
    }

    fn fits(self, n: i128) -> bool {
        let (lo, hi) = self.min_max();
        n >= lo && n <= hi
    }

    fn ty_tokens(self) -> TokenStream {
        let id = format_ident!("{}", self.name());
        quote! { #id }
    }

    fn lit(self, n: i128, span: Span) -> syn::LitInt {
        syn::LitInt::new(&format!("{}{}", n, self.name()), span)
    }
}

/// `#[numbered(u8)]` or `#[numbered(repr = u8, start = 1, crate = ::numbered)]`.
struct NumberedAttr {
    repr: ReprType,
    start: i128,
    crate_path: syn::Path,
    no_display: bool,
    no_variants: bool,
}

fn set_once<T>(slot: &mut Option<T>, value: T, span: Span, msg: &str) -> Result<()> {
    if slot.is_some() {
        return Err(syn::Error::new(span, msg));
    }
    *slot = Some(value);
    Ok(())
}

fn eat_comma(input: ParseStream<'_>) -> Result<()> {
    if !input.is_empty() {
        input.parse::<Token![,]>()?;
    }
    Ok(())
}

fn parse_signed_int(input: ParseStream<'_>) -> Result<(i128, Span)> {
    if input.peek(Token![-]) {
        let minus: Token![-] = input.parse()?;
        let lit: syn::LitInt = input.parse()?;
        let n: i128 = lit.base10_parse()?;
        Ok((-n, minus.span))
    } else {
        let lit: syn::LitInt = input.parse()?;
        let n: i128 = lit.base10_parse()?;
        Ok((n, lit.span()))
    }
}

fn parse_repr_ident(ident: &Ident) -> Result<ReprType> {
    ReprType::from_name(&ident.to_string()).ok_or_else(|| {
        syn::Error::new(
            ident.span(),
            format!("unknown numbered repr type `{ident}`; expected {REPR_HELP}"),
        )
    })
}

fn overflow_err(n: i128, repr: ReprType, span: Span) -> syn::Error {
    syn::Error::new(span, format!("number {n} does not fit in {}", repr.name()))
}

impl Parse for NumberedAttr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut repr = None;
        let mut start = None;
        let mut crate_path = None;
        let mut no_display = None;
        let mut no_variants = None;

        while !input.is_empty() {
            if input.peek(Token![crate]) && input.peek2(Token![=]) {
                let crate_tok: Token![crate] = input.parse()?;
                input.parse::<Token![=]>()?;
                set_once(
                    &mut crate_path,
                    input.parse()?,
                    crate_tok.span,
                    "duplicate numbered crate",
                )?;
            } else if input.peek(syn::Ident) && input.peek2(Token![=]) {
                let key: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                if key == "repr" {
                    let ty: Ident = input.parse()?;
                    set_once(
                        &mut repr,
                        parse_repr_ident(&ty)?,
                        key.span(),
                        "duplicate numbered repr",
                    )?;
                } else if key == "start" {
                    let (n, span) = parse_signed_int(input)?;
                    set_once(&mut start, n, key.span(), "duplicate numbered start")?;
                    let _ = span;
                } else {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown numbered key `{key}`"),
                    ));
                }
            } else if input.peek(syn::Ident) {
                let ident: Ident = input.parse()?;
                if ident == "no_display" {
                    set_once(
                        &mut no_display,
                        true,
                        ident.span(),
                        "duplicate numbered no_display",
                    )?;
                } else if ident == "no_variants" {
                    set_once(
                        &mut no_variants,
                        true,
                        ident.span(),
                        "duplicate numbered no_variants",
                    )?;
                } else {
                    set_once(
                        &mut repr,
                        parse_repr_ident(&ident)?,
                        ident.span(),
                        "duplicate numbered repr",
                    )?;
                }
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    format!("expected numbered repr type or key; expected {REPR_HELP}"),
                ));
            }
            eat_comma(input)?;
        }

        let Some(repr) = repr else {
            return Err(syn::Error::new(
                input.span(),
                "missing numbered repr type (e.g. #[numbered(u8)] or #[numbered(repr = u8)])",
            ));
        };

        Ok(Self {
            repr,
            start: start.unwrap_or(0),
            crate_path: crate_path.unwrap_or_else(|| syn::parse_quote!(::numbered)),
            no_display: no_display.unwrap_or(false),
            no_variants: no_variants.unwrap_or(false),
        })
    }
}

struct VariantAttr {
    n: Option<(i128, Span)>,
}

impl Parse for VariantAttr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut n = None;
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key != "n" {
                return Err(syn::Error::new(
                    key.span(),
                    format!("unknown numbered variant key `{key}`"),
                ));
            }
            let (value, span) = parse_signed_int(input)?;
            set_once(&mut n, (value, span), key.span(), "duplicate numbered n")?;
            eat_comma(input)?;
        }
        Ok(Self { n })
    }
}

fn find_numbered_attr<'a>(
    attrs: &'a [Attribute],
    duplicate_msg: &str,
) -> Result<Option<&'a Attribute>> {
    let mut found = None;
    for attr in attrs {
        if !attr.path().is_ident("numbered") {
            continue;
        }
        if found.is_some() {
            return Err(syn::Error::new(attr.span(), duplicate_msg));
        }
        found = Some(attr);
    }
    Ok(found)
}

fn parse_variant_n(variant: &syn::Variant) -> Result<Option<(i128, Span)>> {
    let Some(attr) = find_numbered_attr(&variant.attrs, "duplicate #[numbered(...)] on variant")?
    else {
        return Ok(None);
    };
    let parsed: VariantAttr = attr.parse_args()?;
    let Some(n) = parsed.n else {
        return Err(syn::Error::new(
            attr.span(),
            "variant #[numbered(...)] requires n = <integer>",
        ));
    };
    Ok(Some(n))
}

fn parse_expr_int(expr: &Expr) -> Result<(i128, Span)> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit), ..
        }) => Ok((lit.base10_parse()?, lit.span())),
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr,
            ..
        }) => {
            let (n, span) = parse_expr_int(expr)?;
            Ok((-n, span))
        }
        Expr::Group(g) => parse_expr_int(&g.expr),
        Expr::Paren(p) => parse_expr_int(&p.expr),
        _ => Err(syn::Error::new(
            expr.span(),
            "numbered discriminant must be an integer literal",
        )),
    }
}

fn parse_discriminant(variant: &syn::Variant) -> Result<Option<(i128, Span)>> {
    let Some((_, expr)) = &variant.discriminant else {
        return Ok(None);
    };
    parse_expr_int(expr).map(Some)
}

struct NumberedVariant<'a> {
    ident: &'a Ident,
    number: i128,
    span: Span,
}

fn unit_variants(input: &DeriveInput) -> Result<Vec<&syn::Variant>> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "Numbered can only be derived for enums",
        ));
    };

    let mut variants = Vec::with_capacity(data.variants.len());
    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new(
                variant.span(),
                "Numbered only supports unit variants (no fields)",
            ));
        }
        variants.push(variant);
    }
    if variants.is_empty() {
        return Err(syn::Error::new(
            input.ident.span(),
            "Numbered enum must have at least one variant",
        ));
    }
    Ok(variants)
}

fn assign_numbers<'a>(
    variants: &[&'a syn::Variant],
    repr: ReprType,
    start: i128,
) -> Result<Vec<NumberedVariant<'a>>> {
    if !repr.fits(start) {
        return Err(overflow_err(start, repr, Span::call_site()));
    }

    let mut out = Vec::with_capacity(variants.len());
    let mut prev: Option<i128> = None;
    let mut owner = BTreeMap::<i128, &Ident>::new();

    for variant in variants {
        let n_attr = parse_variant_n(variant)?;
        let disc = parse_discriminant(variant)?;

        let (number, span) = match (n_attr, disc) {
            (Some((a, sa)), Some((b, _sb))) if a != b => {
                return Err(syn::Error::new(
                    sa,
                    format!("#[numbered(n = {a})] disagrees with discriminant {b}"),
                ));
            }
            (Some((n, span)), _) | (None, Some((n, span))) => (n, span),
            (None, None) => {
                let n = match prev {
                    Some(p) => p.checked_add(1).ok_or_else(|| {
                        syn::Error::new(
                            variant.ident.span(),
                            format!("number overflow for {}", repr.name()),
                        )
                    })?,
                    None => start,
                };
                (n, variant.ident.span())
            }
        };

        if !repr.fits(number) {
            return Err(overflow_err(number, repr, span));
        }
        if let Some(prev_ident) = owner.insert(number, &variant.ident) {
            return Err(syn::Error::new(
                variant.ident.span(),
                format!(
                    "number {number} is shared by `{prev_ident}` and `{}`",
                    variant.ident
                ),
            ));
        }

        prev = Some(number);
        out.push(NumberedVariant {
            ident: &variant.ident,
            number,
            span: variant.ident.span(),
        });
    }
    Ok(out)
}

pub fn derive(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let name = &input.ident;

    let attr = match find_numbered_attr(&input.attrs, "duplicate #[numbered(...)] attribute")? {
        Some(a) => a.parse_args::<NumberedAttr>()?,
        None => {
            return Err(syn::Error::new(
                name.span(),
                "missing #[numbered(<repr>)] container attribute (e.g. #[numbered(u8)])",
            ));
        }
    };

    let raw_variants = unit_variants(&input)?;
    let variants = assign_numbers(&raw_variants, attr.repr, attr.start)?;
    let crate_path = &attr.crate_path;
    let repr_ty = attr.repr.ty_tokens();
    let as_method = format_ident!("as_{}", attr.repr.name());
    let from_method = format_ident!("from_{}", attr.repr.name());

    let idents: Vec<&Ident> = variants.iter().map(|v| v.ident).collect();
    let number_lits: Vec<syn::LitInt> = variants
        .iter()
        .map(|v| attr.repr.lit(v.number, v.span))
        .collect();

    let number_arms: Vec<TokenStream> = variants
        .iter()
        .zip(number_lits.iter())
        .map(|(v, lit)| {
            let ident = v.ident;
            quote! { Self::#ident => #lit }
        })
        .collect();

    let from_arms: Vec<TokenStream> = variants
        .iter()
        .zip(number_lits.iter())
        .map(|(v, lit)| {
            let ident = v.ident;
            quote! { #lit => ::core::result::Result::Ok(Self::#ident) }
        })
        .collect();

    let eq_arms: Vec<TokenStream> = variants
        .iter()
        .zip(number_lits.iter())
        .map(|(v, lit)| {
            let ident = v.ident;
            quote! { Self::#ident => *other == #lit }
        })
        .collect();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants_const = (input.generics.params.is_empty() && !attr.no_variants).then(|| {
        quote! {
            /// All variants in declaration order.
            pub const VARIANTS: &'static [Self] = &[#(Self::#idents,)*];
        }
    });

    let numbers_const = input.generics.params.is_empty().then(|| {
        quote! {
            /// [`Self::number`] for each variant in declaration order.
            pub const NUMBERS: &'static [#repr_ty] = &[#(#number_lits,)*];
        }
    });

    let display_impl = (!attr.no_display).then(|| {
        quote! {
            impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let n = match self { #(#number_arms,)* };
                    ::core::fmt::Display::fmt(&n, f)
                }
            }
        }
    });

    let serde_impls = cfg!(feature = "serde").then(|| {
        quote! {
            impl #impl_generics #crate_path::__serde::Serialize for #name #ty_generics #where_clause {
                fn serialize<S: #crate_path::__serde::Serializer>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error> {
                    let n = match self { #(#number_arms,)* };
                    #crate_path::__serde::Serialize::serialize(&n, serializer)
                }
            }

            impl<'de> #crate_path::__serde::Deserialize<'de> for #name #ty_generics #where_clause {
                fn deserialize<D: #crate_path::__serde::Deserializer<'de>>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error> {
                    let n = <#repr_ty as #crate_path::__serde::Deserialize>::deserialize(deserializer)?;
                    Self::from_number(n).map_err(#crate_path::__serde::de::Error::custom)
                }
            }
        }
    });

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #variants_const
            #numbers_const

            /// Stable integer number assigned to this variant.
            ///
            /// Overridden by `#[numbered(n = ...)]` or a native discriminant.
            #[inline]
            #[must_use]
            pub const fn number(self) -> #repr_ty {
                match self { #(#number_arms,)* }
            }

            /// Alias of [`Self::number`].
            #[inline]
            #[must_use]
            pub const fn #as_method(self) -> #repr_ty {
                self.number()
            }

            /// Parse a number into `Self`.
            ///
            /// Equivalent to [`TryFrom::try_from`](core::convert::TryFrom).
            #[inline]
            pub const fn from_number(
                n: #repr_ty,
            ) -> ::core::result::Result<Self, #crate_path::FromNumberError<#repr_ty>> {
                match n {
                    #(#from_arms,)*
                    n => ::core::result::Result::Err(#crate_path::FromNumberError::new(n)),
                }
            }

            /// Alias of [`Self::from_number`].
            #[inline]
            pub const fn #from_method(
                n: #repr_ty,
            ) -> ::core::result::Result<Self, #crate_path::FromNumberError<#repr_ty>> {
                Self::from_number(n)
            }
        }

        #display_impl

        impl #impl_generics ::core::convert::From<#name #ty_generics> for #repr_ty #where_clause {
            #[inline]
            fn from(value: #name #ty_generics) -> #repr_ty {
                value.number()
            }
        }

        impl #impl_generics ::core::convert::TryFrom<#repr_ty> for #name #ty_generics #where_clause {
            type Error = #crate_path::FromNumberError<#repr_ty>;
            #[inline]
            fn try_from(
                n: #repr_ty,
            ) -> ::core::result::Result<Self, #crate_path::FromNumberError<#repr_ty>> {
                Self::from_number(n)
            }
        }

        impl #impl_generics ::core::cmp::PartialEq<#repr_ty> for #name #ty_generics #where_clause {
            #[inline]
            fn eq(&self, other: &#repr_ty) -> bool {
                match self { #(#eq_arms,)* }
            }
        }

        impl #impl_generics ::core::cmp::PartialEq<#name #ty_generics> for #repr_ty #where_clause {
            #[inline]
            fn eq(&self, other: &#name #ty_generics) -> bool {
                other == self
            }
        }

        #serde_impls
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repr_fits() {
        assert!(ReprType::U8.fits(0));
        assert!(ReprType::U8.fits(255));
        assert!(!ReprType::U8.fits(256));
        assert!(!ReprType::U8.fits(-1));
        assert!(ReprType::I8.fits(-128));
        assert!(ReprType::I8.fits(127));
        assert!(!ReprType::I8.fits(128));
        assert!(!ReprType::I8.fits(-129));
        assert!(ReprType::U16.fits(256));
        assert!(ReprType::Isize.fits(i64::MIN as i128));
        assert!(ReprType::Usize.fits(0));
        assert!(!ReprType::Usize.fits(-1));
    }

    #[test]
    fn repr_from_name() {
        assert_eq!(ReprType::from_name("u8").unwrap().name(), "u8");
        assert_eq!(ReprType::from_name("isize").unwrap().name(), "isize");
        assert!(ReprType::from_name("f32").is_none());
        assert!(ReprType::from_name("u128").is_none());
    }
}
