mod calculator;

use calculator::Calculator;
use slint::SharedString;

slint::include_modules!();

fn main() {
    let app = CalculatorApp::new().expect("new app failed");
    let calculator = Calculator::new().unwrap();
    let week = app.as_weak();

    app.global::<Logic>().on_button_pressed(
        move | value | {
        let app = week.unwrap();
        let expression = app.get_expression();
        app.set_expression(  expression +  value.as_str() );
        }
    );
    // Handle the `calculate` callback
    app.global::<Logic>().on_calculate(
        move | raw_expr | {
        let expression = calculator.calculate(&raw_expr);
        SharedString::from(format!("{}", expression))
        }
    );

    app.run().expect("run failed"); // Start the Slint application
}
