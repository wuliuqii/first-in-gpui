use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x333147))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .font("JetBrains Mono")
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        });
    });
}
