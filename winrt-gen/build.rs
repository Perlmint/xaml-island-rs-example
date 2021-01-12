fn main() {
    winrt::build!(
        windows::ui::xaml::controls::{Button, Page, StackPanel, TextBlock, TextBox, ListView}
        windows::ui::xaml::hosting::{DesktopWindowXamlSource, WindowsXamlManager}
        windows::ui::xaml::markup::XamlReader

        windows::ui::xaml::data::ICustomPropertyProvider
    );
}