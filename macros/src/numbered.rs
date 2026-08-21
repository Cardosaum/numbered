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
                let ty: Ident = input.parse()?;
                set_once(
                    &mut repr,
                    parse_repr_ident(&ty)?,
                    ty.span(),
                    "duplicate numbered repr",
                )?;
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
    fields: &'a Fields,
}

fn ignore_pat(ident: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unit => quote! { Self::#ident },
        Fields::Named(_) => quote! { Self::#ident { .. } },
        Fields::Unnamed(_) => quote! { Self::#ident(..) },
    }
}

fn has_payload(fields: &Fields) -> bool {
    match fields {
        Fields::Unit => false,
        Fields::Named(n) => !n.named.is_empty(),
        Fields::Unnamed(u) => !u.unnamed.is_empty(),
    }
}

fn enum_variants(input: &DeriveInput) -> Result<Vec<&syn::Variant>> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(
            input.ident.span(),
            "Numbered can only be derived for enums",
        ));
    };

    let mut variants = Vec::with_capacity(data.variants.len());
    for variant in &data.variants {
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
            fields: &variant.fields,
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

    let raw_variants = enum_variants(&input)?;
    let variants = assign_numbers(&raw_variants, attr.repr, attr.start)?;
    let crate_path = &attr.crate_path;
    let repr_ty = attr.repr.ty_tokens();
    let fieldless = variants.iter().all(|v| !has_payload(v.fields));

    let idents: Vec<&Ident> = variants.iter().map(|v| v.ident).collect();
    let number_lits: Vec<syn::LitInt> = variants
        .iter()
        .map(|v| attr.repr.lit(v.number, v.span))
        .collect();

    let number_arms: Vec<TokenStream> = variants
        .iter()
        .zip(number_lits.iter())
        .map(|(v, lit)| {
            let pat = ignore_pat(v.ident, v.fields);
            quote! { #pat => #lit }
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
            let pat = ignore_pat(v.ident, v.fields);
            quote! { #pat => *other == #lit }
        })
        .collect();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants_impl = input.generics.params.is_empty().then(|| {
        let variant_ty_and_table = if fieldless {
            quote! {
                type Variant = Self;
                const VARIANTS: &'static [Self] = &[#(Self::#idents,)*];
            }
        } else {
            let holes = variants.iter().map(|_| quote! { () });
            quote! {
                type Variant = ();
                const VARIANTS: &'static [()] = &[#(#holes,)*];
            }
        };
        quote! {
            impl #crate_path::Variants for #name {
                type Repr = #repr_ty;
                #variant_ty_and_table
                const NUMBERS: &'static [Self::Repr] = &[#(#number_lits,)*];
            }
        }
    });

    let de_impl_generics = {
        let params = &input.generics.params;
        if params.is_empty() {
            quote! { <'de> }
        } else {
            quote! { <'de, #params> }
        }
    };

    let serde_serialize = cfg!(feature = "serde").then(|| {
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
        }
    });

    let serde_deserialize = (fieldless && cfg!(feature = "serde")).then(|| {
        quote! {
            impl #de_impl_generics #crate_path::__serde::Deserialize<'de> for #name #ty_generics #where_clause {
                fn deserialize<D: #crate_path::__serde::Deserializer<'de>>(
                    deserializer: D,
                ) -> ::core::result::Result<Self, D::Error> {
                    let n = <#repr_ty as #crate_path::__serde::Deserialize>::deserialize(deserializer)?;
                    #crate_path::FromNumber::from_number(n)
                        .map_err(#crate_path::__serde::de::Error::custom)
                }
            }
        }
    });

    let parse_impls = fieldless.then(|| {
        quote! {
            impl #impl_generics #crate_path::FromNumber for #name #ty_generics #where_clause {
                type Repr = #repr_ty;

                #[inline]
                fn from_number(
                    n: #repr_ty,
                ) -> ::core::result::Result<Self, #crate_path::FromNumberError<#repr_ty>> {
                    match n {
                        #(#from_arms,)*
                        n => ::core::result::Result::Err(#crate_path::FromNumberError::new(n)),
                    }
                }
            }
        }
    });

    let try_from_impl = fieldless.then(|| {
        quote! {
            impl #impl_generics ::core::convert::TryFrom<#repr_ty> for #name #ty_generics #where_clause {
                type Error = #crate_path::FromNumberError<#repr_ty>;
                #[inline]
                fn try_from(
                    n: #repr_ty,
                ) -> ::core::result::Result<Self, #crate_path::FromNumberError<#repr_ty>> {
                    #crate_path::FromNumber::from_number(n)
                }
            }
        }
    });

    Ok(quote! {
        impl #impl_generics #crate_path::Number for #name #ty_generics #where_clause {
            type Repr = #repr_ty;

            #[inline]
            fn number(&self) -> #repr_ty {
                match self { #(#number_arms,)* }
            }
        }

        #parse_impls

        #variants_impl

        impl #impl_generics ::core::convert::From<#name #ty_generics> for #repr_ty #where_clause {
            #[inline]
            fn from(value: #name #ty_generics) -> #repr_ty {
                #crate_path::Number::number(&value)
            }
        }

        #try_from_impl

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

        #serde_serialize
        #serde_deserialize
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn err_msg(input: TokenStream) -> String {
        derive(input)
            .expect_err("expected derive to fail")
            .to_string()
    }

    fn assert_err(input: TokenStream, needle: &str) {
        let msg = err_msg(input);
        assert!(
            msg.contains(needle),
            "expected {needle:?} in error, got {msg:?}"
        );
    }

    fn ok(input: TokenStream) -> String {
        derive(input)
            .expect("expected derive to succeed")
            .to_string()
    }

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
        assert!(ReprType::U32.fits(u32::MAX as i128));
        assert!(ReprType::U64.fits(u64::MAX as i128));
        assert!(ReprType::I16.fits(i16::MIN as i128));
        assert!(ReprType::I32.fits(i32::MAX as i128));
        assert!(ReprType::I64.fits(i64::MIN as i128));
        assert!(!ReprType::I64.fits(i128::from(i64::MAX) + 1));
    }

    #[test]
    fn repr_from_name() {
        for name in [
            "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize",
        ] {
            let repr = ReprType::from_name(name).expect(name);
            assert_eq!(repr.name(), name);
            assert_eq!(repr.ty_tokens().to_string(), name);
        }
        assert!(ReprType::from_name("f32").is_none());
        assert!(ReprType::from_name("u128").is_none());
        assert!(ReprType::from_name("i128").is_none());
    }

    #[test]
    fn rejects_every_container_error() {
        assert_err(
            quote! {
                #[numbered(u8)]
                struct NotAnEnum { x: u8 }
            },
            "Numbered can only be derived for enums",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                union NotAnEnum { x: u8 }
            },
            "Numbered can only be derived for enums",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {}
            },
            "Numbered enum must have at least one variant",
        );
        let fielded = ok(quote! {
            #[numbered(u8, start = 1)]
            enum HostError {
                Unsupported { capability: &'static str },
                OpenFailed { cause: String },
                BadRequest { why: &'static str },
                Io { status: String },
            }
        });
        assert!(fielded.contains(":: Number"));
        assert!(fielded.contains("Variant = ()"));
        assert!(!fielded.contains("from_number"));
        assert!(!fielded.contains("FromNumber"));
        assert!(fielded.contains("NUMBERS"));
        assert_err(
            quote! { enum Mode { A } },
            "missing #[numbered(<repr>)] container attribute",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                #[numbered(u16)]
                enum Mode { A }
            },
            "duplicate #[numbered(...)] attribute",
        );
        assert_err(
            quote! {
                #[numbered()]
                enum Mode { A }
            },
            "missing numbered repr type",
        );
        assert_err(
            quote! {
                #[numbered(start = 1)]
                enum Mode { A }
            },
            "missing numbered repr type",
        );
        assert_err(
            quote! {
                #[numbered(1)]
                enum Mode { A }
            },
            "expected numbered repr type or key",
        );
        assert_err(
            quote! {
                #[numbered(f32)]
                enum Mode { A }
            },
            "unknown numbered repr type `f32`",
        );
        assert_err(
            quote! {
                #[numbered(repr = f32)]
                enum Mode { A }
            },
            "unknown numbered repr type `f32`",
        );
        assert_err(
            quote! {
                #[numbered(u8, rename = "x")]
                enum Mode { A }
            },
            "unknown numbered key `rename`",
        );
        assert_err(
            quote! {
                #[numbered(u8, start = 1, start = 2)]
                enum Mode { A }
            },
            "duplicate numbered start",
        );
        assert_err(
            quote! {
                #[numbered(u8, u16)]
                enum Mode { A }
            },
            "duplicate numbered repr",
        );
        assert_err(
            quote! {
                #[numbered(repr = u8, u16)]
                enum Mode { A }
            },
            "duplicate numbered repr",
        );
        assert_err(
            quote! {
                #[numbered(u8, crate = ::a, crate = ::b)]
                enum Mode { A }
            },
            "duplicate numbered crate",
        );
    }

    #[test]
    fn rejects_every_variant_error() {
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered(foo = 1)]
                    A,
                }
            },
            "unknown numbered variant key `foo`",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered()]
                    A,
                }
            },
            "variant #[numbered(...)] requires n = <integer>",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered(n = 1, n = 2)]
                    A,
                }
            },
            "duplicate numbered n",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered(n = 1)]
                    #[numbered(n = 1)]
                    A,
                }
            },
            "duplicate #[numbered(...)] on variant",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode { A = 1 + 1 }
            },
            "numbered discriminant must be an integer literal",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered(n = 5)]
                    A = 3,
                }
            },
            "#[numbered(n = 5)] disagrees with discriminant 3",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    Zero,
                    #[numbered(n = 0)]
                    AlsoZero,
                }
            },
            "number 0 is shared by `Zero` and `AlsoZero`",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    Zero = 1,
                    Also = 1,
                }
            },
            "number 1 is shared by `Zero` and `Also`",
        );
    }

    #[test]
    fn rejects_every_overflow() {
        assert_err(
            quote! {
                #[numbered(u8, start = 256)]
                enum Mode { A }
            },
            "number 256 does not fit in u8",
        );
        assert_err(
            quote! {
                #[numbered(u8, start = -1)]
                enum Mode { A }
            },
            "number -1 does not fit in u8",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered(n = 256)]
                    TooBig,
                }
            },
            "number 256 does not fit in u8",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode {
                    #[numbered(n = -1)]
                    Neg,
                }
            },
            "number -1 does not fit in u8",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode { A = 256 }
            },
            "number 256 does not fit in u8",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode { A = 255, B }
            },
            "number 256 does not fit in u8",
        );
        assert_err(
            quote! {
                #[numbered(i8)]
                enum Mode {
                    #[numbered(n = 128)]
                    TooBig,
                }
            },
            "number 128 does not fit in i8",
        );
        assert_err(
            quote! {
                #[numbered(i8, start = -129)]
                enum Mode { A }
            },
            "number -129 does not fit in i8",
        );
        assert_err(
            quote! {
                #[numbered(usize, start = -1)]
                enum Mode { A }
            },
            "number -1 does not fit in usize",
        );
        assert_err(
            quote! {
                #[numbered(u8)]
                enum Mode { A = 0x100 }
            },
            "number 256 does not fit in u8",
        );
    }

    #[test]
    fn accepts_assignment_forms() {
        let basic = ok(quote! {
            #[numbered(u8)]
            enum Mode { A, B }
        });
        assert!(basic.contains("FromNumberError"));
        assert!(basic.contains("FromNumber"));
        assert!(basic.contains(":: Number"));
        assert!(!basic.contains("Self :: Error"));
        assert!(!basic.contains("pub const fn number"));
        assert!(basic.contains("Variants"));

        ok(quote! {
            #[numbered(u8, start = 1)]
            enum Mode { A, B }
        });
        ok(quote! {
            #[numbered(repr = u16, start = 1)]
            enum Mode { A, B = 443, C }
        });
        ok(quote! {
            #[numbered(u8)]
            enum Mode {
                #[numbered(n = 10)]
                A,
                B,
            }
        });
        ok(quote! {
            #[numbered(u8)]
            enum Mode {
                #[numbered(n = 5)]
                A = 5,
            }
        });
        ok(quote! {
            #[numbered(u8)]
            enum Mode { Hex = 0x10, Paren = (7) }
        });
        ok(quote! {
            #[numbered(i8, start = -1)]
            enum Mode { A, B = 5, C }
        });
        ok(quote! {
            #[numbered(u8, start = 1,)]
            enum Mode {
                #[numbered(n = 10,)]
                A,
            }
        });
        let via_crate = ok(quote! {
            #[numbered(u8, crate = ::other::numbered)]
            enum Mode { A }
        });
        assert!(via_crate.contains("other"));

        let generic = ok(quote! {
            #[numbered(u8)]
            enum Flag<const N: usize> { A, B }
        });
        assert!(!generic.contains("Variants"));
        #[cfg(feature = "serde")]
        {
            assert!(generic.contains("const N"));
            assert!(generic.contains("Serialize"));
            assert!(generic.contains("Deserialize"));
        }

        let error_var = ok(quote! {
            #[numbered(u8, start = 1)]
            enum Level { Error, Warn }
        });
        assert!(error_var.contains("FromNumberError"));
        assert!(!error_var.contains("Result < Self , Self :: Error >"));
    }

    #[test]
    fn accepts_every_repr() {
        for repr in [
            quote!(u8),
            quote!(u16),
            quote!(u32),
            quote!(u64),
            quote!(usize),
            quote!(i8),
            quote!(i16),
            quote!(i32),
            quote!(i64),
            quote!(isize),
        ] {
            ok(quote! {
                #[numbered(#repr)]
                enum Mode { A, B }
            });
        }
    }
}
