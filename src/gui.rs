use slint::{ComponentHandle, ToSharedString};
use crate::eval;

slint::include_modules!();

pub fn gui() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    let ui_handle = main_window.as_weak();

    main_window.on_button_pressed(move |input| {
        let ui = ui_handle.unwrap();

        let mut display = String::from(ui.get_display());
        let button = input.as_str();

        match button {
            "=" => {
                match eval::return_string_result(&display) {
                    Ok(result) => {
                        ui.set_display(result.to_shared_string())
                    },
                    Err(error) => {
                        eprintln!("{}", error);
                        ui.set_display("ERROR".to_shared_string())
                    }
                }
            }
            "C" => { ui.set_display("".to_shared_string()) }
            _ => {
               display.push_str(button);
                ui.set_display(display.to_shared_string())
            }
        }
    });


    main_window.run()
}