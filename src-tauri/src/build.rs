extern crate embed_resource;

fn main() {
  tauri_build::build();
  embed_resource::compile("windows/printnanny-imager.exe.rc");
  windows::build! {
    Windows::Win32::Foundation::*,
    Windows::Win32::Storage::FileSystem::*,
    Window::Win32::System::IO::*,
    Window::Win32::System::Ioctl::*,
    Windows::Win32::System::Diagnostics::Debug::*
};
}