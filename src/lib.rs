use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
use quote::format_ident;
use syn::{parse_macro_input, AttrStyle, Data, DataEnum, DeriveInput, Ident, Generics, Type};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn with_common_variant_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    let common_data_type: Type = parse_macro_input!(attr);

    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = parse_macro_input!(item);

    let DataEnum { variants, .. } = match data {
        Data::Enum(data_enum) => data_enum,
        _ => abort! {ident, "This attribute is only applicable for enums"},
    };
    let variants_enum_name = format_ident!("{}Variants", ident);

    let Generics { lt_token, params: gen_params, where_clause , ..} = &generics;
    let gen_params = lt_token.map(|_| quote::quote!{<#gen_params>});

    let ident_gen = quote::quote! { #ident #generics };
    let ident_gen_par_where = quote::quote! { #ident #gen_params #where_clause };

    let var_enum_name_gen = quote::quote! { #variants_enum_name #generics };
    let var_enum_name_gen_par = quote::quote! { #variants_enum_name #gen_params };

    let outer_attrs = attrs
        .iter()
        .filter(|attr| matches!(attr.style, AttrStyle::Outer)).collect::<Vec<_>>();

    let doc_ident = Ident::new("doc", Span::call_site());
    let non_doc_outer_attrs = outer_attrs.iter().filter(|attr| attr.meta.path().get_ident() != Some(&doc_ident));

    (quote::quote! {
        #(#outer_attrs)*
        #vis struct #ident_gen {
            common: #common_data_type,
            variant: #var_enum_name_gen_par
        }

        #(#non_doc_outer_attrs)*
        #vis enum #var_enum_name_gen {
            #variants
        }

        impl #gen_params #ident_gen_par_where {
            fn new(common: #common_data_type, variant: #var_enum_name_gen_par) -> Self {
                Self { common, variant }
            }

            fn common(&self) -> &#common_data_type {
                &self.common
            }

            fn common_mut(&mut self) -> &mut #common_data_type {
                &mut self.common
            }

            fn set_common(&mut self, common: #common_data_type) {
                self.common = common;
            }

            fn variant(&self) -> &#var_enum_name_gen_par {
                &self.variant
            }

            fn variant_mut(&mut self) -> &mut #var_enum_name_gen_par {
                &mut self.variant
            }

            fn set_variant(&mut self, variant: #var_enum_name_gen_par) {
                self.variant = variant;
            }

            fn as_ref_mut(&mut self) -> &mut #var_enum_name_gen_par {
                &mut self.variant
            }
        }

        impl #gen_params AsRef<#var_enum_name_gen_par> for #ident_gen_par_where {
            fn as_ref(&self) -> &#var_enum_name_gen_par {
                &self.variant
            }
        }

        impl #gen_params std::ops::Deref for #ident_gen_par_where {
            type Target = #var_enum_name_gen_par;

            fn deref(&self) -> &Self::Target {
                &self.variant
            }
        }

        impl #gen_params std::ops::DerefMut for #ident_gen_par_where {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.variant
            }
        }
    })
    .into()
}