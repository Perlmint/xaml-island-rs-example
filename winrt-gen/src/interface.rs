pub mod microsoft {
    pub mod toolkit {
        pub mod win32 {
            pub mod ui {
                pub mod xamlhost {
                    use winrt::{IUnknown, Interface, Guid, RawPtr, ErrorCode, Result, Abi};

                    // IDL can be found in um/windows.ui.xaml.hosting.destktopwindowxamlsource.idl
                    #[repr(transparent)]
                    pub struct IDesktopWindowXamlSourceNative(IUnknown);

                    impl IDesktopWindowXamlSourceNative {
                        pub fn attach_to_window(&self, hwnd: RawPtr) -> Result<()> {
                            unsafe {
                                (self.vtable().3)(self.abi(), hwnd).ok()
                            }
                        }

                        pub fn get_window_handle(&self) -> Result<RawPtr> {
                            unsafe {
                                let mut hwnd = std::ptr::null_mut();
                                (self.vtable().4)(self.abi(), &mut hwnd).and_then(|| hwnd)
                            }
                        }
                    }

                    #[repr(C)]
                    pub struct IDesktopWindowXamlSourceNative_vtable(
                        pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode,
                        pub unsafe extern "system" fn(this: RawPtr) -> u32,
                        pub unsafe extern "system" fn(this: RawPtr) -> u32,
                        pub unsafe extern "system" fn(this: RawPtr, hwnd: RawPtr) -> ErrorCode,
                        pub unsafe extern "system" fn(this: RawPtr, hwnd_out: *mut RawPtr) -> ErrorCode,
                    );

                    unsafe impl Interface for IDesktopWindowXamlSourceNative {
                        type Vtable = IDesktopWindowXamlSourceNative_vtable;
                    
                        const IID: Guid = Guid::from_values(
                            0x3CBC_F1BF,
                            0x2F76,
                            0x4E9C,
                            [0x96, 0xAB, 0xE8, 0x4B, 0x37, 0x97, 0x25, 0x54],
                        );
                    }

                    use crate::windows::ui::xaml::hosting::DesktopWindowXamlSource;

                    impl std::convert::TryFrom<DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
                        type Error = winrt::Error;
                        fn try_from(value: DesktopWindowXamlSource) -> Result<Self> {
                            value.cast()
                        }
                    }

                    impl std::convert::TryFrom<&DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
                        type Error = winrt::Error;
                        fn try_from(value: &DesktopWindowXamlSource) -> Result<Self> {
                            value.cast()
                        }
                    }
                }
            }
        }
    }
}