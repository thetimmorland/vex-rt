//! Do not use this crate on it's own

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, parse_macro_input, ImplItem, ItemImpl, Signature};

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#`[vex_rt::entry]` attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let expected = TokenStream::from(quote! {
        impl Robot {
            fn initialize() -> Self;
            fn autonomous(&self);
            fn opcontrol(&self);
            fn disable(&self);
        }
    });

    let expected = parse_macro_input!(expected as ItemImpl);
    let mut expected_sigs: Vec<Signature> = Vec::new();
    for item in expected.items {
        match item {
            ImplItem::Method(method) => expected_sigs.push(method.sig),
            _ => {}
        };
    }

    let impl_ = parse_macro_input!(input as ItemImpl);

    let valid_impl = impl_.trait_.is_none() && impl_.items.len() == 4;
    impl_.items.iter().fold(true, |is_valid, it| match it {
        ImplItem::Method(method) => is_valid && expected_sigs.contains(&method.sig),
        _ => false,
    });

    if !valid_impl {
        return parse::Error::new(
            impl_.span(),
            "`#[vex_rt::entry]` impl must have the form:
impl Foo {
\tfn initialize -> Self;
\t\t...
\t}
\tfn autonomous(&self) {
\t\t...
\t}
\tfn opcontrol(&self) {
\t\t...
\t}
\tfn disable(&self) {
\t\t...
\t}
}",
        )
        .to_compile_error()
        .into();
    }

    let self_ty = impl_.self_ty.clone();

    let expanded = quote! {
        #impl_

        static ROBOT: vex_rt::once::Once<#self_ty> = vex_rt::once::Once::new();

        #[no_mangle]
        unsafe extern "C" fn initialize() {
            ROBOT.call_once(|| #self_ty::initialize());
        }

        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            ROBOT.get().as_ref().unwrap().opcontrol();
        }

        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            ROBOT.get().as_ref().unwrap().autonomous();
        }

        #[no_mangle]
        unsafe extern "C" fn disabled() {
            ROBOT.get().as_ref().unwrap().disable();
        }
    };

    TokenStream::from(expanded)
}
