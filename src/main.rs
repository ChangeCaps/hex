mod views;

use ori::prelude::*;
use views::{OnePicker, TwoPicker};

#[derive(Clone, Copy, Debug)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn swap(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    fn palette(self) -> Palette {
        match self {
            Self::Light => Palette::light(),
            Self::Dark => Palette::dark(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Output {
    Css,
    Ori,
}

pub struct Data {
    hue: f32,
    color: Color,
    theme: Theme,
    output: Output,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            hue: Default::default(),
            color: Color::BLACK,
            theme: Theme::Dark,
            output: Output::Css,
        }
    }
}

fn theme_button(data: &mut Data) -> impl View<Data> {
    let icon = match data.theme {
        Theme::Light => fa::icon("sun"),
        Theme::Dark => fa::icon("moon"),
    };

    let view = button(icon)
        .color(palette().surface)
        .border_radius([12.0, 0.0, 0.0, 0.0])
        .border_bottom(1.0);

    let view = tooltip(view, "Change theme");

    on_click(view, |_, data: &mut Data| data.theme = data.theme.swap())
}

fn close_button() -> impl View<Data> {
    let view = button(fa::icon("xmark").color(palette().surface))
        .color(palette().error)
        .border_radius([0.0, 12.0, 0.0, 0.0])
        .border_bottom(1.0);

    let view = tooltip(view, "Quit");

    on_click(view, |cx, _| cx.cmd(AppCommand::Quit))
}

fn top_bar(data: &mut Data) -> impl View<Data> {
    let theme = theme_button(data);
    let title = text!("hex");
    let close = close_button();

    let view = hstack![theme, title, close].justify(Justify::SpaceBetween);
    let view = (container(view).border_bottom(1.0)).border_radius([12.0, 12.0, 0.0, 0.0]);
    on_press(trigger(view), |_, _| {}).descendants(false)
}

fn one_picker() -> impl View<Data> {
    height(200.0, OnePicker)
}

fn two_picker(data: &mut Data) -> impl View<Data> {
    let view = TwoPicker { hue: data.hue };
    size(200.0, view)
}

fn picker(data: &mut Data) -> impl View<Data> {
    let color = container(height(50.0, ()))
        .background(data.color)
        .border_radius(6.0);

    let view = hstack![two_picker(data), one_picker()]
        .justify(Justify::SpaceBetween)
        .gap(20.0);

    vstack![view, color].gap(20.0)
}

fn copy_button<T>() -> impl View<T> {
    let mut init = false;

    animate(move |copied: &mut bool, cx, _, _| {
        if !cx.is_hot() {
            *copied = false;
        }

        if !(cx.active_changed() || cx.hot_changed()) && init {
            return None;
        }

        init = true;

        let copy = button(fa::icon("copy").size(12.0))
            .color(palette().surface)
            .padding(6.0);

        if cx.is_active() {
            *copied = true;
        }

        if *copied {
            Some(tooltip(copy, "Copied!"))
        } else {
            Some(tooltip(copy, "Copy"))
        }
    })
}

fn copyable_text<T>(shown: &str, copied: &str) -> impl View<T> {
    let copy = on_click(copy_button(), {
        let copied = copied.to_owned();
        move |cx, _| {
            cx.clipboard().set(copied.clone());
        }
    });

    hstack![text(shown).font_size(12.0), copy]
        .justify(Justify::SpaceBetween)
        .gap(12.0)
}

// https://stackoverflow.com/a/61101531
fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

fn hsl_css(data: &mut Data) -> impl View<Data> {
    let (h, s, l) = data.color.to_hsv();
    let shown = format!(
        "hsl({: <5.0}, {: <4}, {: <4})",
        h,
        format!("{:.0}%", s * 100.0),
        format!("{:.0}%", l * 100.0),
    );
    let copied = format!("hsl({}, {}, {})", h, s * 100.0, l * 100.0);
    copyable_text(&shown, &copied)
}

fn hsv_css(data: &mut Data) -> impl View<Data> {
    let (h, s, v) = data.color.to_hsv();
    let shown = format!(
        "hsv({: <5.0}, {: <4}, {: <4})",
        h,
        format!("{:.0}%", s * 100.0),
        format!("{:.0}%", v * 100.0),
    );
    let copied = format!("hsv({}, {}, {})", h, s * 100.0, v * 100.0);
    copyable_text(&shown, &copied)
}

fn rgb_css(data: &mut Data) -> impl View<Data> {
    let [r, g, b, _] = data.color.to_rgba8();
    let shown = format!("rgb({: <5}, {: <4}, {: <4})", r, g, b);
    let copied = format!("rgb({}, {}, {})", r, g, b);
    copyable_text(&shown, &copied)
}

fn hex_css(data: &mut Data) -> impl View<Data> {
    let hex = data.color.to_hex();
    copyable_text(&hex, &hex)
}

fn ori_hsl(data: &mut Data) -> impl View<Data> {
    let (h, s, l) = data.color.to_hsl();
    let shown = format!(
        "hsl({: <5?}, {: <4?}, {: <4?})",
        round(h, 1),
        round(s, 2),
        round(l, 2)
    );

    let copied = format!(
        "hsl({:?}, {:?}, {:?})",
        round(h, 1),
        round(s, 2),
        round(l, 2)
    );

    copyable_text(&shown, &copied)
}

fn ori_hsv(data: &mut Data) -> impl View<Data> {
    let (h, s, v) = data.color.to_hsv();
    let shown = format!(
        "hsv({: <5?}, {: <4?}, {: <4?})",
        round(h, 1),
        round(s, 2),
        round(v, 2)
    );

    let copied = format!(
        "hsv({:?}, {:?}, {:?})",
        round(h, 1),
        round(s, 2),
        round(v, 2)
    );

    copyable_text(&shown, &copied)
}

fn ori_rgb(data: &mut Data) -> impl View<Data> {
    let shown = format!(
        "rgb({: <5?}, {: <4?}, {: <4?})",
        round(data.color.r, 2),
        round(data.color.g, 2),
        round(data.color.b, 2),
    );

    let copied = format!(
        "rgb({:?}, {:?}, {:?})",
        round(data.color.r, 2),
        round(data.color.g, 2),
        round(data.color.b, 2)
    );

    copyable_text(&shown, &copied)
}

fn ori_hex(data: &mut Data) -> impl View<Data> {
    let hex = format!("hex(\"{}\")", data.color.to_hex());
    copyable_text(&hex, &hex)
}

fn output_button(data: &mut Data, output: Output) -> impl View<Data> {
    let mut icon = match output {
        Output::Css => text!("css").font_size(14.0),
        Output::Ori => text!("ori").font_size(14.0),
    };

    if data.output == output {
        icon = icon.color(palette().surface);
    }

    let mut view = button(icon)
        .color(palette().surface)
        .border_radius(12.0)
        .padding([8.0, 2.0]);

    if data.output == output {
        view = view.color(palette().secondary);
    }

    let view = tooltip(view, "Change output format");

    on_click(view, move |_, data: &mut Data| data.output = output)
}

fn output_selector(data: &mut Data) -> impl View<Data> {
    hstack![
        output_button(data, Output::Css),
        output_button(data, Output::Ori),
    ]
    .gap(8.0)
}

fn output(data: &mut Data) -> impl View<Data> {
    let view = match data.output {
        Output::Css => any(
            vstack![hsl_css(data), hsv_css(data), rgb_css(data), hex_css(data),]
                .align(Align::Start)
                .gap(2.0),
        ),
        Output::Ori => any(
            vstack![ori_hsl(data), ori_hsv(data), ori_rgb(data), ori_hex(data),]
                .align(Align::Start)
                .gap(2.0),
        ),
    };

    vstack![
        center(output_selector(data)),
        container(pad(12.0, view))
            .background(palette().surface_low)
            .border_radius(4.0)
    ]
    .gap(4.0)
}

fn content(data: &mut Data) -> impl View<Data> {
    pad(24.0, vstack![picker(data), output(data)].gap(20.0))
}

fn ui(data: &mut Data) -> impl View<Data> {
    styled(data.theme.palette(), || {
        let view = vstack![top_bar(data), content(data)].align(Align::Stretch);

        container(top(view))
            .background(palette().background)
            .border_radius(12.0)
    })
}

fn style() -> Styles {
    Styles::new()
        .build(|style| TextStyle {
            font_family: FontFamily::Monospace,
            ..Style::style(style)
        })
        .build(|style| CheckboxStyle {
            size: 24.0,
            border_width: BorderWidth::all(1.0),
            border_radius: BorderRadius::all(4.0),
            ..Style::style(style)
        })
}

fn main() {
    let window = Window::new()
        .title("hex")
        .decorated(false)
        .fit_content()
        .color(Some(Color::TRANSPARENT))
        .icon(include_image!("icon.png"))
        .resizable(false);

    let app = App::build().window(window, ui).style(style());

    ori::launch(app, Data::default()).unwrap();
}
