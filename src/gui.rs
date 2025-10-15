use slint::ComponentHandle;

slint::include_modules!();

pub fn gui() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    main_window.run()
}