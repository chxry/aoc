use proc_macro::TokenStream;
use syn::{LitInt, parse_macro_input};
use quote::quote;

#[proc_macro]
pub fn day(input: TokenStream) -> TokenStream {
  let n: LitInt = parse_macro_input!(input);
  let path = format!("{}.rs", n);
  let input = format!("../inputs/{}.txt", n);
  quote! {{
    use std::time::Instant;
    #[path = #path]
    mod day;
    let input = include_str!(#input);
    println!("- day {}:",#n);
    let start = Instant::now();
    println!("  part 1: {} ({:?})", day::main(input, false), Instant::now().duration_since(start));
    let start = Instant::now();
    println!("  part 2: {} ({:?})", day::main(input, true), Instant::now().duration_since(start));
  }}
  .into()
}
