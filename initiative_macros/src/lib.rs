use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(RandomTable)]
pub fn random_table_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_random_table(&ast)
}

fn impl_random_table(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let syn::Data::Enum(data_enum) = &ast.data {
        let cases = data_enum.variants.iter().enumerate().map(|(i, variant)| {
            quote! { #i => #name::#variant, }
        });
        let len = cases.len();
        let gen = quote! {
            impl RandomTable for #name {
                fn get_random(rng: &mut impl rand::Rng) -> Self {
                    //println!("{}", stringify!(#ast));
                    match rng.gen_range(0..#len) {
                        #(#cases)*
                        _ => unreachable!(),
                    }
                }
            }
        };
        gen.into()
    } else {
        panic!();
    }
}

#[proc_macro_attribute]
pub fn weight(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
