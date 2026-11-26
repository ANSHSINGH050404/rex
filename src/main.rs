use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, px, rgb,
    size, prelude::*,
};

struct Rex {
    tab_count: usize,
}

impl Rex {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self { tab_count: 1 }
    }
}

impl Render for Rex {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x111111))
            .text_color(rgb(0xffffff))
            .child(header(cx, self.tab_count))
            .child(terminal_content())
            .child(status_bar(self.tab_count))
    }
}

fn header(cx: &mut Context<Rex>, tab_count: usize) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .px(px(16.))
        .py(px(10.))
        .bg(rgb(0x1b1b1b))
        .child(format!(
            "Rex Terminal ({} {})",
            tab_count,
            if tab_count == 1 { "tab" } else { "tabs" }
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(4.))
                .child(
                    div()
                        .id("dec-tab")
                        .px(px(8.))
                        .py(px(4.))
                        .rounded(px(4.))
                        .bg(rgb(0x2a2a2a))
                        .cursor_pointer()
                        .child("-")
                        .on_click(cx.listener(|this, _, _, cx| {
                            if this.tab_count > 1 {
                                this.tab_count -= 1;
                            }
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .id("new-tab")
                        .px(px(8.))
                        .py(px(4.))
                        .rounded(px(4.))
                        .bg(rgb(0x2a2a2a))
                        .cursor_pointer()
                        .child("+ New Tab")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.tab_count += 1;
                            cx.notify();
                        })),
                ),
        )
}

fn terminal_content() -> impl IntoElement {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .p(px(16.))
        .bg(rgb(0x111111))
        .child(div().text_color(rgb(0x808080)).child("ansh@omarchy ~"))
        .child("$")
}

fn status_bar(tab_count: usize) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .justify_between()
        .px(px(12.))
        .py(px(6.))
        .bg(rgb(0x1b1b1b))
        .child(format!("Tabs: {}", tab_count))
        .child("UTF-8")
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(Rex::new),
        )
        .unwrap();

        cx.activate(true);
    });
}