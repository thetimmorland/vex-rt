//! Do not use this crate on it's own

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, parse_macro_input, ItemImpl, Path};

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#`[entry]` attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let impl_ = parse_macro_input!(input as ItemImpl);

    let valid_impl = match impl_.trait_.clone() {
        None => false,
        Some((_, p1, _)) => {
            let p2 = TokenStream::from(quote!(vex_rt::Robot));
            let p2 = parse_macro_input!(p2 as Path);
            p1 == p2
        }
    };

    if !valid_impl {
        return parse::Error::new(
            impl_.span(),
            "`#[entry]` impl block must implement `vex_rt::Robot`",
        )
        .to_compile_error()
        .into();
    }

    let self_ty = impl_.self_ty.clone();

    let expanded = quote! {
        #impl_

        static mut ROBOT: Option<#self_ty> = None;

        #[no_mangle]
        unsafe extern "C" fn initialize() {
            ROBOT = Some(#self_ty::initialize());
        }

        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            ROBOT.as_mut().unwrap().opcontrol();
        }

        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            ROBOT.as_mut().unwrap().autonomous();
        }

        #[no_mangle]
        unsafe extern "C" fn disabled() {
            ROBOT.as_mut().unwrap().disable();
        }
    };

    TokenStream::from(expanded)
}
