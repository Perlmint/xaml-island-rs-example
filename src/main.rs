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
        hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager},
        markup::XamlReader,
        RoutedEventHandler,
    },
    Object,
};

const MAIN_XAML: &'static str = include_str!("../res/main.xaml");

fn main() {
    // should call this at the beginning
    unsafe { RoInitialize(RO_INIT_SINGLETHREADED) };

    unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };

    // WindowsXamlManager should be initialized before event loop creation.
    // In some evironment, creating DesktopWindowXamlSource initialized WindowsXamlManager, Initializing WindowsXamlManager will occur error.
    // let _ = WindowsXamlManager::initialize_for_current_thread();
    let desktop_source =
        winrt::factory::<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory>()
            .unwrap()
            .create_instance(Object::default(), &mut Object::default())
            .unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("XAML Island on rust");
    window.set_window_icon(Some(Icon::from_resource(1, None).unwrap()));

    let hwnd = window.hwnd();
    let window_id = window.id();

    let xaml_island_hwnd = {
        let idestkop_source: IDesktopWindowXamlSourceNative = desktop_source.clone().into();
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

    let main_page = winrt::ComInterface::query::<Page>(&XamlReader::load(MAIN_XAML).unwrap());
    desktop_source.set_content(&main_page).unwrap();

    let text_box = winrt::ComInterface::query::<TextBox>(&main_page.find_name("text_box").unwrap());
    let stack = winrt::ComInterface::query::<StackPanel>(&main_page.find_name("stack").unwrap());
    let button = winrt::ComInterface::query::<Button>(&main_page.find_name("button").unwrap());
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
}
