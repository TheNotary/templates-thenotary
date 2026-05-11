#![allow(non_snake_case)]
#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
pub(crate) mod bindings;

#[cfg(windows)]
mod views;

#[cfg(windows)]
use bindings::*;

#[cfg(windows)]
use windows::{
    core::*,
    ApplicationModel::{
        Activation::{
            CachedFileUpdaterActivatedEventArgs, FileActivatedEventArgs,
            FileOpenPickerActivatedEventArgs, FileSavePickerActivatedEventArgs,
            IActivatedEventArgs, LaunchActivatedEventArgs, SearchActivatedEventArgs,
            ShareTargetActivatedEventArgs,
        },
        Package,
    },
    Win32::{
        System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
};

#[cfg(windows)]
#[implement(IApplicationOverrides)]
struct App;

#[cfg(windows)]
impl IApplicationOverrides_Impl for App_Impl {
    fn OnLaunched(&self, _: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        views::main_view::build_ui()
    }

    fn OnActivated(&self, _: Ref<IActivatedEventArgs>) -> Result<()> { Ok(()) }
    fn OnFileActivated(&self, _: Ref<FileActivatedEventArgs>) -> Result<()> { Ok(()) }
    fn OnSearchActivated(&self, _: Ref<SearchActivatedEventArgs>) -> Result<()> { Ok(()) }
    fn OnShareTargetActivated(&self, _: Ref<ShareTargetActivatedEventArgs>) -> Result<()> { Ok(()) }
    fn OnFileOpenPickerActivated(&self, _: Ref<FileOpenPickerActivatedEventArgs>) -> Result<()> { Ok(()) }
    fn OnFileSavePickerActivated(&self, _: Ref<FileSavePickerActivatedEventArgs>) -> Result<()> { Ok(()) }
    fn OnCachedFileUpdaterActivated(
        &self,
        _: Ref<CachedFileUpdaterActivatedEventArgs>,
    ) -> Result<()> { Ok(()) }
    fn OnWindowCreated(&self, _: Ref<WindowCreatedEventArgs>) -> Result<()> { Ok(()) }
}

#[cfg(windows)]
fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        if let Err(result) = Package::Current() {
            MessageBoxW(
                None,
                w!("This sample must be registered (via register.cmd) and launched from the Start menu."),
                w!("Error"),
                MB_ICONSTOP | MB_OK,
            );
            return Err(result);
        }
    }

    Application::Start(&ApplicationInitializationCallback::new(|_| {
        Application::compose(App)?;
        Ok(())
    }))
}
