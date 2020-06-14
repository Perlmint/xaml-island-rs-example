winrt::build!(
    dependencies
        os
    types
        windows::ui::xaml::controls::{Button, Page, StackPanel, TextBlock, TextBox}
        windows::ui::xaml::hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager}
        windows::ui::xaml::markup::XamlReader
);

fn main() {
    build();
}