mod calculator;

use calculator::Calculator;
use slint::SharedString;

slint::include_modules!();

fn main() {
    let app = CalculatorApp::new().expect("new app failed");
    let calculator = Calculator::new();

    // Handle the `calculate` callback
    app.on_calculate(move |expr: SharedString| {
        let result = calculator.calculate(&expr);
        SharedString::from(format!("{}", result))
    });

    app.run().expect("run failed"); // Start the Slint application
}
