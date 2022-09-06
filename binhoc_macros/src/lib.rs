use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::{AttributeArgs, ItemFn};
use crate::adhoc_reqwest_fn::impl_adhoc_reqwest_fn;

mod adhoc_reqwest_fn;


#[proc_macro_attribute]
pub fn binhoc(attr: TokenStream, item:TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);

    // Build the trait implementation
    impl_adhoc_reqwest_fn(&attr,&item)
}
