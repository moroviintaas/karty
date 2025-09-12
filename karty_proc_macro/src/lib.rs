use quote::quote;
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(RandomSymbol)]
pub fn implement_distribution_of_dimension(item: TokenStream) -> TokenStream{
    let ast = parse_macro_input!(item as DeriveInput);
    //let vis = ast.vis;
    let ident = ast.ident;
    let expanded = quote! {
        impl Distribution<#ident> for rand::distr::StandardUniform{
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> #ident{
                #ident::from_usize_index(rng.gen_range(0..#ident::SYMBOL_SPACE)).unwrap()
            }
        }
    };
    TokenStream::from(expanded)
}
