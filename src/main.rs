use gpui::{
    App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};


struct Rex {
    tabs:Vec<Entity<Tab>>,
}


struct tab {
    title:String,
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
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(16.))
                    .py(px(10.))
                    .bg(rgb(0x1b1b1b))
                    .child("Rex Terminal")
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(12.))
                            .items_center()
                            .child(format!("Tabs: {}", self.tabs))
                            .child(
                                div()
                                    .px(px(10.))
                                    .py(px(5.))
                                    .bg(rgb(0x333333))
                                    .child("+ New Tab")
                                    .on_click(cx.listener(
                                        |this, _event, _window, cx| {
                                            this.tabs += 1;
                                            cx.notify();
                                        },
                                    )),
                            ),
                    ),
            )
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
        .bg(rgb(0xFF0000))
        .child(div().text_color(rgb(0xa6e3a1)).child("ansh@omarchy ~"))
        .child(div().text_color(rgb(0xa6e3a1)).child("$"))
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
            |_window, cx| cx.new(|_cx| Rex { tabs: 1 }),
        )
        .unwrap();

        cx.activate(true);
    });
}
