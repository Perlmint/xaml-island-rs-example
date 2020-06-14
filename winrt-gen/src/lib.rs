mod gen {
    include!(concat!(env!("OUT_DIR"), "/winrt.rs"));
}
pub use gen::*;
pub use winrt::*;

mod idesktop_window_xaml_source_native;
pub mod microsoft {
pub mod toolkit {
pub mod win32 {
pub mod ui {
pub mod xamlhost {
    pub use crate::idesktop_window_xaml_source_native::*;
}
}
}
}
}