use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, emit_error, proc_macro_error};
use quote::format_ident;
use syn::{parse_macro_input, AttrStyle, Data, DataEnum, DeriveInput, Expr, Ident, Meta, MetaList, MetaNameValue, Generics};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn with_common_variant_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = parse_macro_input!(item);

    let outer_attrs = attrs
        .iter()
        .filter(|attr| matches!(attr.style, AttrStyle::Outer));

    let common_data_type_ident = Ident::new("common_data_type", Span::call_site());

    let (common_data_type_attributes, other_outer_attributes) =
        outer_attrs.partition::<Vec<_>, _>(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue { path, .. }) => {
                path.get_ident() == Some(&common_data_type_ident)
            }
            _ => false,
        });

    const VALID_COMMON_DATA_TYPE_ATTR_STR: &str = "Has to be #[common_data_type = T]";

    let valid_common_data_types = common_data_type_attributes
        .into_iter()
        .filter_map(|attr| {
            match &attr.meta {
                Meta::NameValue(MetaNameValue {
                    value: Expr::Path(ty),
                    ..
                }) => return Some(ty),
                Meta::List(l) => emit_error!(l, VALID_COMMON_DATA_TYPE_ATTR_STR),
                Meta::Path(p) => emit_error!(p, VALID_COMMON_DATA_TYPE_ATTR_STR),
                _ => {}
            }

            None
        })
        .collect::<Vec<_>>();

    let common_data_type = match valid_common_data_types.len() {
        0 => abort! {
            ident,
            "Found no #[common_data_type = T] outer attributes";
            note = "Add a #[common_data_type = T] outer attribute"
        },
        1 => valid_common_data_types.into_iter().next().unwrap(),
        _ => abort! {
            ident,
            "More than 1 valid #[common_data_type = T] attribute";
            note = "Remove one of the attributes"
        },
    };

    let DataEnum { variants, .. } = match data {
        Data::Enum(data_enum) => data_enum,
        _ => abort! {ident, "This attribute is only applicable for enums"},
    };
    let variants_enum_name = format_ident!("{}Variants", ident);

    let Generics { lt_token, params: gen_params, where_clause , ..} = &generics;
    let gen_params = lt_token.map(|_| quote::quote!{<#gen_params>});

    let doc_ident = Ident::new("doc", Span::call_site());
    let non_doc_outer_attrs = other_outer_attributes.iter().filter(|attr| attr.meta.path().get_ident() != Some(&doc_ident));

    (quote::quote! {
        #(#other_outer_attributes)*
        #vis struct #ident #generics {
            common: #common_data_type,
            variant: #variants_enum_name #gen_params
        }

        #(#non_doc_outer_attrs)*
        #vis enum #variants_enum_name #generics {
            #variants
        }

        impl #gen_params #ident #gen_params #where_clause {
            fn new(common: #common_data_type, variant: #variants_enum_name #gen_params) -> Self {
                Self { common, variant }
            }

            fn common(&self) -> &#common_data_type {
                &self.common
            }

            fn common_mut(&mut self) -> &mut #common_data_type {
                &mut self.common
            }

            fn variant(&self) -> &#variants_enum_name #gen_params {
                &self.variant
            }

            fn variant_mut(&mut self) -> &mut #variants_enum_name #gen_params {
                &mut self.variant
            }

            fn set_common(&mut self, common: #common_data_type) {
                self.common = common;
            }

            fn set_variant(&mut self, variant: #variants_enum_name #gen_params) {
                self.variant = variant;
            }

            fn as_ref_mut(&mut self) -> &mut #variants_enum_name #gen_params {
                &mut self.variant
            }
        }

        impl #gen_params AsRef<#variants_enum_name #gen_params> for #ident #gen_params #where_clause {
            fn as_ref(&self) -> &#variants_enum_name #gen_params {
                &self.variant
            }
        }

        impl #gen_params std::ops::Deref for #ident #gen_params #where_clause {
            type Target = #variants_enum_name #gen_params;

            fn deref(&self) -> &Self::Target {
                &self.variant
            }
        }

        impl #gen_params std::ops::DerefMut for #ident #gen_params #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.variant
            }
        }
    })
    .into()
}

enum TestVariants {
    A, B, C
}

struct Test {
    common: u8,
    variant: TestVariants,
}

impl AsRef<TestVariants> for Test {
    fn as_ref(&self) -> &TestVariants {
        &self.variant
    }
}

impl std::ops::Deref for Test {
    type Target = TestVariants;

    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl std::ops::DerefMut for Test {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}