use gpui::{
    App,
    Context,
    Window,
    WindowOptions,
    Render,
    div,
    prelude::*,
};

struct Rex;

impl Render for Rex {

    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x111111))
            .text_color(rgb(0xffffff))
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(16.))
                    .py(px(10.))
                    .bg(rgb(0x1b1b1b))
                    .child("Rex Terminal")
                    .child("+ New Tab"),
            )
            .child(
                // Main terminal area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p(px(16.))
                    .bg(rgb(0x111111))
                    .child("ansh@omarchy ~")
                    .child("$"),
            )
            .child(
                // Status bar
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .px(px(12.))
                    .py(px(6.))
                    .bg(rgb(0x1b1b1b))
                    .child("Shell: zsh")
                    .child("UTF-8"),
            )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| {
                cx.new(|_cx| Rex)
            },
        )
        .unwrap();

        cx.activate(true);
    });
}