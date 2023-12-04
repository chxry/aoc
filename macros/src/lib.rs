use proc_macro::TokenStream;
use syn::{LitInt, parse_macro_input};
use quote::quote;

#[proc_macro]
pub fn day(input: TokenStream) -> TokenStream {
  let n: LitInt = parse_macro_input!(input);
  let path = format!("{}.rs", n);
  let input = format!("../inputs/{}.txt", n);
  quote! {{
    #[path = #path]
    mod day;
    let input = include_str!(#input);
    println!("- day {}:",#n);
    println!("  part 1: {}", day::main(input,false));
    println!("  part 2: {}", day::main(input, true));
  }}
  .into()
}
