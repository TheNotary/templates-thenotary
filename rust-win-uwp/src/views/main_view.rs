#![allow(non_snake_case)]

use crate::bindings::*;

use windows::{
    core::*,
    ApplicationModel::Core::CoreApplication,
    System::VirtualKey,
};

use super::main_actions;

pub fn build_ui() -> Result<()> {
    let window = Window::Current()?;

    // ---- Root grid: row 0 fills, row 1 is auto-sized for the input strip. ----
    let root = Grid::new()?;
    // A transparent brush makes the grid hit-testable so title-bar drag works everywhere.
    root.SetBackground(&SolidColorBrush::new()?)?;

    let row0 = RowDefinition::new()?;
    row0.SetHeight(GridLength {
        Value: 1.0,
        GridUnitType: GridUnitType::Star,
    })?;
    let row1 = RowDefinition::new()?;
    row1.SetHeight(GridLength {
        Value: 0.0,
        GridUnitType: GridUnitType::Auto,
    })?;
    root.RowDefinitions()?.Append(&row0)?;
    root.RowDefinitions()?.Append(&row1)?;

    // ---- Hello text, centered in row 0. ----
    let hello = TextBlock::new()?;
    hello.SetText(h!("Hello"))?;
    hello.SetFontSize(48.0)?;
    hello.SetHorizontalAlignment(HorizontalAlignment::Center)?;
    hello.SetVerticalAlignment(VerticalAlignment::Center)?;
    Grid::SetRow(&hello, 0)?;
    root.Children()?.Append(&hello)?;

    // ---- Bottom input strip: TextBox (*) + Send button (Auto). ----
    let bottom = Grid::new()?;
    bottom.SetMargin(Thickness { Left: 8.0, Top: 8.0, Right: 8.0, Bottom: 8.0 })?;

    let col0 = ColumnDefinition::new()?;
    col0.SetWidth(GridLength { Value: 1.0, GridUnitType: GridUnitType::Star })?;
    let col1 = ColumnDefinition::new()?;
    col1.SetWidth(GridLength { Value: 0.0, GridUnitType: GridUnitType::Auto })?;
    bottom.ColumnDefinitions()?.Append(&col0)?;
    bottom.ColumnDefinitions()?.Append(&col1)?;

    let input = TextBox::new()?;
    input.SetPlaceholderText(h!("Type here and press Enter"))?;
    Grid::SetColumn(&input, 0)?;
    bottom.Children()?.Append(&input)?;

    let send = Button::new()?;
    let send_label = TextBlock::new()?;
    send_label.SetText(h!("Send"))?;
    send.SetContent(&send_label)?;
    send.SetMargin(Thickness { Left: 8.0, Top: 0.0, Right: 0.0, Bottom: 0.0 })?;
    Grid::SetColumn(&send, 1)?;
    bottom.Children()?.Append(&send)?;

    Grid::SetRow(&bottom, 1)?;
    root.Children()?.Append(&bottom)?;

    // ---- Wire submit: Enter key in TextBox OR click on Send button. ----
    let input_for_btn = input.clone();
    send.Click(&RoutedEventHandler::new(move |_, _| -> Result<()> {
        main_actions::submit(&input_for_btn)
    }))?;

    let input_for_key = input.clone();
    input.KeyDown(&KeyEventHandler::new(move |_, args| -> Result<()> {
        let args = args.ok()?;
        if args.Key()? == VirtualKey::Enter {
            args.SetHandled(true)?;
            main_actions::submit(&input_for_key)?;
        }
        Ok(())
    }))?;

    // ---- Borderless drag: extend the view into the title bar and mark the
    //      whole root grid as the draggable region. (System caption buttons
    //      remain visible in UWP CoreWindow — see readme.md.)
    if let Ok(view) = CoreApplication::GetCurrentView() {
        if let Ok(title_bar) = view.TitleBar() {
            let _ = title_bar.SetExtendViewIntoTitleBar(true);
        }
    }
    window.SetTitleBar(&root)?;

    window.SetContent(&root)?;
    window.Activate()?;

    // Set initial focus to the input box for immediate typing.
    let _ = input.Focus(FocusState::Programmatic);
    Ok(())
}
