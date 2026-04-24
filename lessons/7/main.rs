// if you have main.rs defined in your source directory,
// then a binary crate with the same name as the package
// will be automatically created and main.rs will be the crate root

// the crate root is the source file that rust compiler starts at when
// building the crate and it also makes up the root module of the crate!

// CRATE RULES
// 1. A package must at least have one crate
// 2. A package can have 0..1 library crates
// 3. A package can have any number of binary crates
pub mod module;

// use allows us to bring path into scope e.g. hosting module will now be into scope
pub use module::example::front_of_house::hosting::{self};

fn main() {
  hosting::add_to_waitlist();
  module::super_keyword::how_to_use_super_key_word();

  // RUST IDIOMS:

  // Calling items via path makes origin explicit (not local scope confusion):
  // module::function()

  // Use full paths for types to make source clear:
  // structs: crate::module::StructName
  // enums:   crate::module::EnumName

  // Use aliases when names collide or improve readability:
  // use crate::module::StructName as AliasStruct;
  // use crate::module::EnumName as AliasEnum;

  // or
  // module:StructName
  // module:EnumName

  // Rule of thumb:
  // - Prefer explicit paths for clarity
  // - Import (`use`) only when it improves readability
  // - Alias only when necessary (naming conflicts or clarity)
}
