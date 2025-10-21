use slint::{ComponentHandle, ToSharedString};

slint::include_modules!();

pub fn gui() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    let ui_handle = main_window.as_weak();

    main_window.on_button_pressed(move |input| {
        let ui = ui_handle.unwrap();

        let mut display = String::from(ui.get_display());
        let button = input.as_str();

        match button {
            "=" => { ui.set_display("RESULT".to_shared_string()) }
            "C" => { ui.set_display("".to_shared_string()) }
            _ => {
               display.push_str(button);
                ui.set_display(display.to_shared_string())
            }
        }
    });


    main_window.run()
}