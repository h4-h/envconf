use proc_macro::TokenStream;
use syn::{parse_macro_input, DataStruct, DeriveInput};
use quote::quote;

/// Derive macro for configuration struct.
///
/// Supports only structs with named fields.
///
/// ## Provided attributes:
///
/// 1. [`var`] - specifies the environment variable name, if not provided - uppercase field name will be used.
/// 2. [`default`] - specifies default value if variable not presented in the environment.
///
/// ## Example of attribute usage:
///
/// ```rust
/// // if not specified - it will try to find `DB_URL`.
/// #[var("DATABASE_URL")]
/// // if not specified and can't be found in the environment - it will panic.
/// #[default("nothing")]
/// db_url: String,
/// ```
///
/// ## Generated methods:
///
/// 1. [`new`] - factory method (there we fetch variables)
/// 2. [`FIELD_NAME`] - public getter for `FIELD_NAME`
///
/// Do not make fields public, it can cause errors when someone accedentally modify them.
///
/// ## Full example
///
/// ```rust,ignore
#[doc = include_str!("../examples/full.rs")]
/// ```
#[proc_macro_derive(EnvConfig, attributes(var, default))]
pub fn env_config_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    
    let fields = match input.data {
        syn::Data::Struct(DataStruct { fields, .. }) => fields,
        _ => panic!("\x1B[0;31mEnvConfig only supports structs.\x1B[0m"),
    };

    let field_getters = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("WHATAHELLL");

        quote! {
            pub fn #field_name(&self) -> &str { self.#field_name.as_str() }
        }
    });
    
    let field_initializers = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("WHATAHELLL");
        let var_name = get_attr_value(field, "var")
            .unwrap_or(field_name.to_string().to_uppercase());
        let default_value = get_attr_value(field, "default");

        let default_value_getter = match default_value {
            Some(val) => quote! {
                #val.to_string()
            },
            None => quote! {
                panic!("\x1B[0;31m{} not found in the environment.\x1B[0m", #var_name)
            },
        };

        quote! {
            #field_name: {
                match std::env::var(#var_name) {
                    Ok(val) => val,
                    Err(_) => #default_value_getter,
                }
            }
        }
    });
    
    let expanded = quote! {
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(#field_initializers),*
                }
            }

            #(#field_getters)*
        }
    };

    expanded.into()
}

/// Simple extractor of the field attribute value
///
/// Returns [`None`] if field not found or have no value.
///
/// Example:
/// ```rust
/// let var_value = get_attr_value(field, "var");
/// ```
fn get_attr_value(field: &syn::Field, ident: &str) -> Option<String> {
    let idx = field.attrs.iter().position(|f| f.path().is_ident(ident))?;
    let args = field.attrs[idx].parse_args::<syn::Expr>().ok()?;

    let lit = match args {
        syn::Expr::Lit(syn::ExprLit { lit , .. }) => Some(lit),
        _ => None,
    };
    
    match lit {
        Some(syn::Lit::Str(val)) => Some(val.value()),
        _ => None,
    }
}
