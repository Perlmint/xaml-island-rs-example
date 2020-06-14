//! Original source from [ubuntu-installer](https://github.com/roblabla/ubuntu-installer/blob/master/src/desktopwindowxamlsource.rs)
use std::ffi::c_void;
use std::ptr::null_mut;

use super::windows::ui::xaml::hosting::DesktopWindowXamlSource;

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct IDesktopWindowXamlSourceNative {
    ptr: ::winrt::ComPtr<IDesktopWindowXamlSourceNative>,
}

impl IDesktopWindowXamlSourceNative {
    pub fn attach_to_window(&self, hwnd: *mut c_void) -> winrt::Result<()> {
        use winrt::AbiTransferable;
        match self.ptr.get_abi() {
            None => panic!("The `this` pointer was null when calling method"),
            Some(this) => {
                (this.vtable().attach_to_window)(this, hwnd).ok()
            }
        }
    }

    pub fn get_window_handle(&self) -> winrt::Result<*mut c_void> {
        use winrt::AbiTransferable;
        match self.ptr.get_abi() {
            None => panic!("The `this` pointer was null when calling method"),
            Some(this) => {
                let mut hwnd = null_mut();
                (this.vtable().get_window_handle)(this, &mut hwnd as *mut _).and_then(|| hwnd)
            }
        }
    }
}

unsafe impl ::winrt::ComInterface for IDesktopWindowXamlSourceNative {
    type VTable = abi_IDesktopWindowXamlSourceNative;
    fn iid() -> ::winrt::Guid {
        ::winrt::Guid::from_values(0x3cbcf1bf, 0x2f76, 0x4e9c, [0x96, 0xab, 0xe8, 0x4b, 0x37, 0x97, 0x25, 0x54])
    }
}


#[repr(C)]
pub struct abi_IDesktopWindowXamlSourceNative {
    pub unknown_query_interface: extern "system" fn(
        ::winrt::NonNullRawComPtr<::winrt::IUnknown>,
        &::winrt::Guid,
        *mut ::winrt::RawPtr,
    )
        -> ::winrt::ErrorCode,
    pub unknown_add_ref:
        extern "system" fn(::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32,
    pub unknown_release:
        extern "system" fn(::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32,
    pub attach_to_window: extern "system" fn(
        ::winrt::NonNullRawComPtr<IDesktopWindowXamlSourceNative>,
        *mut c_void) -> ::winrt::ErrorCode,
    pub get_window_handle: extern "system" fn(
        ::winrt::NonNullRawComPtr<IDesktopWindowXamlSourceNative>,
        *mut *mut c_void) -> ::winrt::ErrorCode,
}

unsafe impl ::winrt::AbiTransferable for IDesktopWindowXamlSourceNative {
    type Abi = ::winrt::RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <::winrt::ComPtr<r#IDesktopWindowXamlSourceNative> as ::winrt::AbiTransferable>::get_abi(
            &self.ptr,
        )
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <::winrt::ComPtr<r#IDesktopWindowXamlSourceNative> as ::winrt::AbiTransferable>::set_abi(
            &mut self.ptr,
        )
    }
}
impl ::std::fmt::Debug for IDesktopWindowXamlSourceNative {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "{}({:?})",
            "IDesktopWindowXamlSourceNative",
            <Self as ::winrt::AbiTransferable>::get_abi(self)
        )
    }
}

impl From<&DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
    fn from(value: &DesktopWindowXamlSource) -> IDesktopWindowXamlSourceNative {
        <DesktopWindowXamlSource as ::winrt::ComInterface>::query(value)
    }
}

impl From<DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
    fn from(value: DesktopWindowXamlSource) -> IDesktopWindowXamlSourceNative {
        std::convert::From::from(&value)
    }
}