extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse_macro_input,
	DeriveInput,
};

#[proc_macro_derive(StackTraceDebug)]
pub fn stack_trace_debug_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;

	let expanded = quote! {
		impl std::fmt::Debug for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				writeln!(f, "{}\n\t", self)?;

				let mut current = self.source();
				while let Some(cause) = current {
					writeln!(f, "Caused by:\n\t{}", cause)?;
					current = cause.source();
				}
				Ok(())
			}
		}
	};

	TokenStream::from(expanded)
}
