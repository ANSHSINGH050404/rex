use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, px, rgb,
    size, prelude::*,
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
            .child(header())
            .child(terminal_content())
            .child(status_bar())
    }
}

fn header() -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .px(px(16.))
        .py(px(10.))
        .bg(rgb(0x1b1b1b))
        .child("Rex Terminal")
        .child("+ New Tab")
}

fn terminal_content() -> impl IntoElement {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .p(px(16.))
        .bg(rgb(0x111111))
        .child("ansh@omarchy ~")
        .child("$")
}

fn status_bar() -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .justify_between()
        .px(px(12.))
        .py(px(6.))
        .bg(rgb(0x1b1b1b))
        .child("Shell: zsh")
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
            |_window, cx| cx.new(|_cx| Rex),
        )
        .unwrap();

        cx.activate(true);
    });
}