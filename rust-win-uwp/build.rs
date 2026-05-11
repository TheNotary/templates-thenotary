// Generates a minimal slice of `Windows.UI.Xaml.*` bindings into `src/bindings.rs`.
// We include *only* the leaf types the sample actually uses; bindgen drops any
// method or property that returns a Xaml type that fell outside the filter
// (it emits a `cargo:warning` for each, which is expected). This keeps the
// `windows::UI::Xaml::*` references that don't exist in the `windows` crate
// from leaking into the generated source.
fn main() {
    let args = [
        "--out", "src/bindings.rs",
        "--flat",
        "--implement",

        "--filter",
        // Lifecycle / hosting
        "Windows.UI.Xaml.Application",
        "Windows.UI.Xaml.IApplicationOverrides",
        "Windows.UI.Xaml.ApplicationInitializationCallback",
        "Windows.UI.Xaml.Window",
        "Windows.UI.Xaml.WindowCreatedEventArgs",
        // Base class hierarchy referenced by required_hierarchy! / cast impls
        "Windows.UI.Xaml.DependencyObject",
        "Windows.UI.Xaml.UIElement",
        "Windows.UI.Xaml.FrameworkElement",
        // Layout / sizing primitives (value types & enums)
        "Windows.UI.Xaml.HorizontalAlignment",
        "Windows.UI.Xaml.VerticalAlignment",
        "Windows.UI.Xaml.Thickness",
        "Windows.UI.Xaml.GridLength",
        "Windows.UI.Xaml.GridUnitType",
        "Windows.UI.Xaml.FocusState",
        "Windows.UI.Xaml.RoutedEventHandler",
        // Controls actually instantiated + their base classes
        "Windows.UI.Xaml.Controls.Control",
        "Windows.UI.Xaml.Controls.ContentControl",
        "Windows.UI.Xaml.Controls.Panel",
        "Windows.UI.Xaml.Controls.Primitives.ButtonBase",
        "Windows.UI.Xaml.Controls.Grid",
        "Windows.UI.Xaml.Controls.RowDefinition",
        "Windows.UI.Xaml.Controls.ColumnDefinition",
        "Windows.UI.Xaml.Controls.RowDefinitionCollection",
        "Windows.UI.Xaml.Controls.ColumnDefinitionCollection",
        "Windows.UI.Xaml.Controls.UIElementCollection",
        "Windows.UI.Xaml.Controls.TextBlock",
        "Windows.UI.Xaml.Controls.TextBox",
        "Windows.UI.Xaml.Controls.Button",
        // Brush (needed so Panel::SetBackground is not dropped)
        "Windows.UI.Xaml.Media.Brush",
        "Windows.UI.Xaml.Media.SolidColorBrush",
        // Key events on TextBox
        "Windows.UI.Xaml.Input.KeyEventHandler",
        "Windows.UI.Xaml.Input.KeyRoutedEventArgs",

        // Route non-XAML types referenced by the XAML surface back to `windows`.
        // IMPORTANT: do NOT reference `Windows.UI` (the bare parent) — that
        // would also match `Windows.UI.Xaml.*` and route Xaml types to a
        // non-existent `windows::UI::Xaml` module.
        "--reference", "windows,skip-root,Windows.Foundation",
        "--reference", "windows,skip-root,Windows.ApplicationModel",
        "--reference", "windows,skip-root,Windows.System",
        "--reference", "windows,skip-root,Windows.UI.Composition",
        "--reference", "windows,skip-root,Windows.UI.Core",
        "--reference", "windows,skip-root,Windows.UI.Input",
        "--reference", "windows,skip-root,Windows.UI.Text",
        "--reference", "windows,skip-root,Windows.Win32",
    ];

    let _ = windows_bindgen::bindgen(args);

    println!("cargo:rerun-if-changed=build.rs");
}
