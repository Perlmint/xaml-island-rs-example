mod gen {
    winrt::include_bindings!();
}
pub use gen::*;
pub use winrt::*;

mod interface;
pub use interface::*;
