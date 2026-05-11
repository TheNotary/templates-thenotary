#![allow(non_snake_case)]

use crate::bindings::*;

use windows::core::*;
use windows::UI::Popups::{IUICommand, MessageDialog};
use windows_future::{AsyncOperationCompletedHandler, AsyncStatus, IAsyncOperation};

pub fn submit(input: &TextBox) -> Result<()> {
    let text = input.Text()?;
    if text.is_empty() {
        return Ok(());
    }

    let message: HSTRING = format!("You said '{}'", text.to_string_lossy()).into();
    let dialog = MessageDialog::Create(&message)?;
    let op: IAsyncOperation<IUICommand> = dialog.ShowAsync()?;

    // When the dialog is dismissed, clear the input and refocus it.
    // The completion handler runs on a dispatcher thread; the XAML calls
    // here marshal themselves because we own the cloned proxies.
    let input_for_done = input.clone();
    op.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |_op, status| -> Result<()> {
            if status == AsyncStatus::Completed {
                let _ = input_for_done.SetText(&HSTRING::new());
                let _ = input_for_done.Focus(FocusState::Programmatic);
            }
            Ok(())
        },
    ))?;
    let _ = op; // keep the operation alive via the handler registration
    Ok(())
}
