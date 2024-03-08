use gpui::*;

#[derive(Clone)]
struct Main {
    state: Model<Counter>,
}

struct Counter {
    count: i32,
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        let state_model_clone: Model<Counter> = self.state.clone();
        let increment_button = div()
            .flex()
            .p_2()
            .rounded_md()
            .bg(rgb(0x5a5a5a))
            .hover(|s| s.bg(rgb(0xF38BA8)))
            .child("Increment")
            .on_mouse_down(MouseButton::Left, move |_ev, cx| {
                cx.update_model(&state_model_clone, |model, _cx| {
                    model.count += 1;
                });
            });

        let state_model_clone: Model<Counter> = self.state.clone();
        let decrement_button = div()
            .flex()
            .p_2()
            .rounded_md()
            .bg(rgb(0x5a5a5a))
            .hover(|s| s.bg(rgb(0xF38BA8)))
            .child("Decrement")
            .on_mouse_down(MouseButton::Left, move |_ev, cx| {
                cx.update_model(&state_model_clone, |model, _cx| {
                    model.count -= 1;
                });
            });

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x333147))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .font("JetBrains Mono")
            .child(format!("Counter = {}", state.count))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .children(vec![increment_button, decrement_button]),
            )
    }
}

fn main() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| {
                let counter_model: Model<Counter> = cx.new_model(|_cx| Counter { count: 0 });

                Main {
                    state: counter_model,
                }
            })
        });
    });
}
