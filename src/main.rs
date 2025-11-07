use slint::SharedString;
slint::include_modules!();

fn main(){
    let app = CalculatorWindow::new().unwrap();

    let weak = app.as_weak();
    app.on_on_button_pressed(move |button: SharedString| {
        let app = weak.unwrap();
        let current = app.get_display_text();

        let new_text = handle_button_press(&current, &button);
        app.set_display_text(SharedString::from(new_text));
    });

    app.run().unwrap();
}

fn handle_button_press(current: &str, button: &str) -> String {
    let mut text = current.to_string();

    match button {
        
        "C" => return "0".to_string(),

        // Xóa 1 ký tự (backspace)
        "⌫" | "←" => {
            if text.len() > 1 {
                text.pop();
            } else {
                text = "0".to_string();
            }
        }

        // Đổi dấu
        "±" => {
            if text.starts_with('-') {
                text.remove(0);
            } else if text != "0" {
                text = format!("-{}", text);
            }
        }

        // Tính kết quả
        "=" => {
            return calculate_expression(&text);
        }

        // Còn lại → thêm ký tự
        _ => {
            if text == "0" && button.chars().next().unwrap().is_numeric() {
                text = button.to_string();
            } else {
                text.push_str(button);
            }
        }
    }

    text
}


fn calculate_expression(expr: &str) -> String {
    let expr = expr
        .replace("×", "*")
        .replace("÷", "/")
        .replace("−", "-");

    match meval::eval_str(expr) {
        Ok(result) => format!("{}", result),
        Err(_) => "Error".to_string(),
    }
}