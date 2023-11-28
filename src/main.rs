slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";

    export component App inherits Window {
        VerticalBox {
            Text { text: "Hello, world!"; }
            Button { text: "Click me!"; }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
