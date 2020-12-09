#![no_std]

//! Do not use this crate on it's own

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn robot(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);

    let self_ty = input.self_ty.clone();

    let expanded = quote! {
        #input

        static mut ROBOT: Option<#self_ty> = None;

        extern "C" fn initialize() {
            let peripherals = vex_rt::Peripherals::take();
            ROBOT = Some(#self_ty::initialize(peripherals));
        }

        extern "C" fn autonomous() {
            ROBOT.unwrap().autonomous();
        }
    };

    TokenStream::from(expanded)
}
