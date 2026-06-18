#![allow(unused)]
mod button;
mod draw;
mod screen;
mod select_box;

fn main() {
    let screen = screen::Screen {
        components: vec![
            Box::new(select_box::SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(button::Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
