#![feature(proc_macro_span)]
use proc_macro::{TokenStream, Span};
use syn::{LitInt, parse_macro_input};
use quote::quote;

#[proc_macro]
pub fn year(input: TokenStream) -> TokenStream {
  let n: LitInt = parse_macro_input!(input);
  let path = format!("{}/mod.rs", n);
  quote! {
    #[path = #path]
    mod year;
    year::main();
  }
  .into()
}

#[proc_macro]
pub fn day(input: TokenStream) -> TokenStream {
  let n: LitInt = parse_macro_input!(input);
  let path = format!("{}.rs", n);
  let file = Span::call_site().source_file().path();
  let year = &file.to_str().unwrap()[4..8];
  let input = format!("../../inputs/{}/{}.txt", year, n);
  quote! {{
    use std::time::Instant;
    #[path = #path]
    mod day;
    let input = include_str!(#input);
    println!("- day {}:" ,#n);
    let start = Instant::now();
    println!("  part 1: {} ({:?})", day::main(input, false), start.elapsed());
    let start = Instant::now();
    println!("  part 2: {} ({:?})", day::main(input, true), start.elapsed());
  }}
  .into()
}
