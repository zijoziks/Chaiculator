use slint::ComponentHandle;

slint::include_modules!();

pub fn gui() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    let ui_handle = main_window.as_weak();


    main_window.run()
}