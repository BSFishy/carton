//! TODO: document this

use proc_macro::{TokenStream};
use proc_macro2::{TokenStream as TokenStream2, Ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Error, spanned::Spanned, punctuated::Punctuated, DataStruct, DataEnum, Index};
use quote::{quote, ToTokens, TokenStreamExt};

#[proc_macro_derive(View)]
pub fn derive_view(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = input.clone().data;

    match data {
        Data::Struct(data) => {
            handle_struct_view(input, data)
        }
        Data::Enum(data) => {
            handle_enum_view(input, data)
        }
        _ => {
            TokenStream::from(Error::new(input.span(), "View can only be derived on structs or enums").to_compile_error())
        }
    }
}

fn handle_struct_view(input: DeriveInput, data: DataStruct) -> TokenStream {
    let fields = data.fields;

    let layout = call_on_fields(&fields, quote![layout], quote!());
    let draw = call_on_fields(&fields, quote![draw], quote!());
    let update = call_on_fields(&fields, quote![update], quote!());

    impl_view(input, layout, draw, update)
}

fn handle_enum_view(input: DeriveInput, data: DataEnum) -> TokenStream {
    let layout = setup_variants(&data, quote![layout], quote!());
    let draw = setup_variants(&data, quote![draw], quote!());
    let update = setup_variants(&data, quote![update], quote!());

    impl_view(input, layout, draw, update)
}

fn setup_variants(data: &DataEnum, function: TokenStream2, params: TokenStream2) -> TokenStream2 {
    let mut cases: Vec<TokenStream2> = vec![];

    for variant in data.variants.iter() {
        let name = &variant.ident;
        let fields = &variant.fields;
        let mut calls: TokenStream2 = TokenStream2::new();

        let case_selector = match fields {
            Fields::Named(fields_named) => {
                let mut names: Vec<TokenStream2> = vec![];
                for field in fields_named.named.iter() {
                    let name = field.ident.clone().expect("Could not get name of named field. This should not be possible.").to_token_stream();
                    names.push(name.clone());
                    calls.append_all(function_call(name, function.clone(), params.clone()));
                }

                quote! { #name { #( #names ),* } }
            }
            Fields::Unnamed(fields) => {
                let mut views: Vec<TokenStream2> = vec![];
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let name = Ident::new(&format!("__view{}", i), field.span()).to_token_stream();
                    views.push(name.clone());
                    calls.append_all(function_call(name, function.clone(), params.clone()));
                }

                quote! { #name(#( #views ),*) }
            }
            Fields::Unit => quote! { #name }
        };

        cases.push(quote! { Self::#case_selector => { #calls } });
    }

    return quote! {
        match self {
            #( #cases )*
        }
    }
}

fn call_on_fields(fields: &Fields, function: TokenStream2, params: TokenStream2) -> TokenStream2 {
    let fields = match fields {
        Fields::Named(fields) => fields.named.clone(),
        Fields::Unnamed(fields) => fields.unnamed.clone(),
        Fields::Unit => Punctuated::new(),
    };

    let mut calls: TokenStream2 = TokenStream2::new();
    for (i, field) in fields.iter().enumerate() {
        let name = match &field.ident {
            None => Index { index: i as u32, span: field.span() }.to_token_stream(),
            Some(name) => name.to_token_stream()
        };

        calls.append_all(function_call(quote! { &self.#name }, function.clone(), params.clone()));
    }

    calls
}

fn function_call(obj: TokenStream2, function: TokenStream2, params: TokenStream2) -> TokenStream2 {
    quote! {
        carton_mvvm::view::View::#function(#obj, #params);
    }
}

fn impl_view(input: DeriveInput, layout: TokenStream2, draw: TokenStream2, update: TokenStream2) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics carton_mvvm::view::View for #name #ty_generics #where_clause {
            fn layout(&self) {
                #layout
            }

            fn draw(&self) {
                #draw
            }

            fn update(&self) {
                #update
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
