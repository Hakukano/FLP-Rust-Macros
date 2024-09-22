mod duplicated;
mod partial;

use proc_macro::TokenStream;

/// Implements From and Into for the derived struct to a target struct, they need to have identical fields.
/// This is useful when you need to apply derive attributes to a third party struct's fields or such.
///
/// ## Derive attributes: `duplicated`
///
/// - `target`: target struct type path
#[proc_macro_derive(Duplicated, attributes(duplicated))]
pub fn duplicated(tokens: TokenStream) -> TokenStream {
    duplicated::handle(tokens)
}

/// Create partials from the derived struct
///
/// ## Derive attributes: `partial`
///
/// - `structs`: A tuple of the partial structs' names
/// - `derives`: A tuple of derives that need to be added to the partials
///
/// ## Field attributes: `partial`
///
/// - `included`: A tuple of the names of partials that have this field.
#[proc_macro_derive(Partial, attributes(partial))]
pub fn partial(tokens: TokenStream) -> TokenStream {
    partial::handle(tokens)
}
