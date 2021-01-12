#![windows_subsystem = "windows"]
use winapi::{
    shared::windef::HWND,
    um::{
        wincon::{AttachConsole, ATTACH_PARENT_PROCESS},
        winuser::{SetWindowPos, SWP_SHOWWINDOW},
    },
    winrt::roapi::{RoInitialize, RO_INIT_SINGLETHREADED},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::{IconExtWindows, WindowExtWindows},
    window::{Icon, WindowBuilder},
};
use winrt::{
    microsoft::toolkit::win32::ui::xamlhost::IDesktopWindowXamlSourceNative,
    windows::ui::xaml::{
        controls::{Button, Page, StackPanel, TextBlock, TextBox},
        hosting::{DesktopWindowXamlSource, WindowsXamlManager},
        markup::XamlReader,
        RoutedEventHandler,
    },
    Interface,
};

const MAIN_XAML: &'static str = include_str!("../res/main.xaml");

fn main() {
    // should call this at the beginning
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED) };

    // Not mandatory, but useful
    unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };

    // WindowsXamlManager should be initialized before event loop creation.
    let manager = WindowsXamlManager::initialize_for_current_thread().unwrap();
    let desktop_source = DesktopWindowXamlSource::new().unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("XAML Island on rust");
    window.set_window_icon(Some(Icon::from_resource(1, None).unwrap()));

    let hwnd = window.hwnd();
    let window_id = window.id();

    let xaml_island_hwnd = {
        let idestkop_source: IDesktopWindowXamlSourceNative =
            std::convert::TryFrom::try_from(&desktop_source).unwrap();
        idestkop_source.attach_to_window(hwnd).unwrap();
        idestkop_source.get_window_handle().unwrap() as HWND
    };
    {
        let size = window.inner_size();
        unsafe {
            SetWindowPos(
                xaml_island_hwnd,
                std::ptr::null_mut(),
                0,
                0,
                size.width as _,
                size.height as _,
                SWP_SHOWWINDOW,
            )
        };
    }

    let main_page: Page = XamlReader::load(MAIN_XAML).unwrap().cast().unwrap();
    desktop_source.set_content(&main_page).unwrap();

    let text_box: TextBox = main_page.find_name("text_box").unwrap().cast().unwrap();
    let stack: StackPanel = main_page.find_name("stack").unwrap().cast().unwrap();
    let button: Button = main_page.find_name("button").unwrap().cast().unwrap();
    button
        .click(RoutedEventHandler::new(move |_, _| {
            let children = stack.children()?;
            let input_text = text_box.text()?;
            if !input_text.is_empty() {
                text_box.set_text("")?;
                children.append({
                    let new_text = TextBlock::new()?;
                    new_text.set_text(input_text.clone())?;

                    new_text
                })?;
            }

            Ok(())
        }))
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: w_id,
            } if w_id == window_id => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id: w_id,
            } if w_id == window_id => {
                unsafe {
                    SetWindowPos(
                        xaml_island_hwnd,
                        std::ptr::null_mut(),
                        0,
                        0,
                        size.width as _,
                        size.height as _,
                        SWP_SHOWWINDOW,
                    )
                };
            }
            _ => {}
        }
    });

    manager.close();
}
