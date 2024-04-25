use ori::prelude::*;

use crate::Data;

pub struct OnePicker;

impl OnePicker {
    pub fn get_color(hue: f32) -> Color {
        Color::hsv(hue, 1.0, 1.0)
    }

    pub fn create_image() -> Image {
        let mut pixels = vec![0u8; 4 * 256];

        for i in 0..256 {
            let hue = i as f32 / 255.0 * 360.0;

            let color = Self::get_color(hue);

            let index = 4 * i;

            let [r, g, b, a] = color.to_rgba8();
            pixels[index] = r;
            pixels[index + 1] = g;
            pixels[index + 2] = b;
            pixels[index + 3] = a;
        }

        Image::new(pixels, 1, 256)
    }
}

pub struct OnePickerState {
    image: Image,
    clicked: bool,
}

impl View<Data> for OnePicker {
    type State = OnePickerState;

    fn build(&mut self, _cx: &mut BuildCx, _data: &mut Data) -> Self::State {
        OnePickerState {
            image: Self::create_image(),
            clicked: false,
        }
    }

    fn rebuild(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut RebuildCx,
        _data: &mut Data,
        _old: &Self,
    ) {
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventCx, data: &mut Data, event: &Event) {
        if let Event::PointerPressed(e) = event {
            if !cx.is_hot() {
                return;
            }

            let local = cx.local(e.position);
            let hue = local.y / cx.size().height * 360.0;

            let (_, s, l) = data.color.to_hsv();
            data.color = Color::hsv(hue, s, l);
            data.hue = hue;

            state.clicked = true;

            cx.request_rebuild();
            cx.request_draw();
        }

        if let Event::PointerMoved(e) = event {
            if !state.clicked {
                return;
            }

            let local = cx.local(e.position);
            let mut hue = local.y / cx.size().height * 360.0;
            hue = hue.clamp(0.0, 360.0);

            let (_, s, l) = data.color.to_hsv();
            data.color = Color::hsv(hue, s, l);
            data.hue = hue;

            cx.request_rebuild();
            cx.request_draw();
        }

        if matches!(event, Event::PointerReleased(_)) {
            state.clicked = false;
        }
    }

    fn layout(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut LayoutCx,
        _data: &mut Data,
        space: Space,
    ) -> Size {
        space.fit(Size::new(30.0, space.max.height))
    }

    fn draw(
        &mut self,
        state: &mut Self::State,
        cx: &mut DrawCx,
        data: &mut Data,
        canvas: &mut Canvas,
    ) {
        canvas.set_hoverable(cx.id());
        canvas.draw_quad(cx.rect(), state.image.clone(), 6.0, 0.0, Color::TRANSPARENT);

        let y = data.hue / 360.0 * cx.size().height;

        let center = Point::new(cx.rect().min.x + cx.size().width / 2.0, y).round();
        let size = Size::new(cx.size().width + 4.0, 4.0);

        canvas.draw_quad(
            Rect::center_size(center, size),
            Color::TRANSPARENT,
            0.0,
            1.0,
            Color::WHITE,
        );

        canvas.draw_quad(
            Rect::center_size(center, size + 2.0),
            Color::TRANSPARENT,
            0.0,
            1.0,
            Color::BLACK,
        );
    }
}

pub struct TwoPicker {
    pub hue: f32,
}

impl TwoPicker {
    pub fn get_color(&self, uv: Point) -> Color {
        Color::hsv(self.hue, uv.x, 1.0 - uv.y)
    }
}

pub struct TwoPickerState {
    image: Image,
    clicked: bool,
}

impl View<Data> for TwoPicker {
    type State = TwoPickerState;

    fn build(&mut self, _cx: &mut BuildCx, _data: &mut Data) -> Self::State {
        TwoPickerState {
            image: include_image!("saturation_value_gradient.png"),
            clicked: false,
        }
    }

    fn rebuild(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut RebuildCx,
        _data: &mut Data,
        _old: &Self,
    ) {
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventCx, data: &mut Data, event: &Event) {
        if let Event::PointerPressed(e) = event {
            if !cx.is_hot() {
                return;
            }

            let local = cx.local(e.position);
            let uv = local / cx.size();

            data.color = self.get_color(uv);
            state.clicked = true;

            cx.request_rebuild();
            cx.request_draw();
        }

        if let Event::PointerMoved(e) = event {
            if !state.clicked {
                return;
            }

            let local = cx.local(e.position);
            let mut uv = local / cx.size();
            uv = uv.clamp(Point::ZERO, Point::ONE);

            data.color = self.get_color(uv);

            cx.request_rebuild();
            cx.request_draw();
        }

        if matches!(event, Event::PointerReleased(_)) {
            state.clicked = false;
        }
    }

    fn layout(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut LayoutCx,
        _data: &mut Data,
        space: Space,
    ) -> Size {
        space.max
    }

    fn draw(
        &mut self,
        state: &mut Self::State,
        cx: &mut DrawCx,
        data: &mut Data,
        canvas: &mut Canvas,
    ) {
        canvas.set_hoverable(cx.id());

        canvas.draw_quad(
            cx.rect(),
            Color::hsv(data.hue, 1.0, 1.0),
            6.0,
            0.0,
            Color::TRANSPARENT,
        );
        canvas.draw_quad(cx.rect(), state.image.clone(), 6.0, 0.0, Color::TRANSPARENT);

        let (_, s, l) = data.color.to_hsv();
        let uv = Point::new(s, 1.0 - l) * cx.size();

        canvas.draw_quad(
            Rect::center_size(uv.round(), Size::all(6.0)),
            Color::TRANSPARENT,
            0.0,
            1.0,
            Color::WHITE,
        );

        canvas.draw_quad(
            Rect::center_size(uv.round(), Size::all(8.0)),
            Color::TRANSPARENT,
            0.0,
            1.0,
            Color::BLACK,
        );
    }
}
