use csscolorparser::Color;
use std::convert::TryFrom;

#[test]
fn basic() {
    let c = Color::new(1.0, 0.0, 0.0, 1.0);
    assert_eq!(c.to_array(), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(c.to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(c.to_rgba16(), [65535, 0, 0, 65535]);

    let c = Color::from_rgb(1., 0., 0.);
    assert_eq!(c.rgba(), (1., 0., 0., 1.));
    assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
    assert_eq!(c.to_hex_string(), "#ff0000");
    assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
    assert_eq!(c.to_string(), "RGBA(1,0,0,1)");
    assert_eq!(c.to_hsva(), (0., 1., 1., 1.));
    assert_eq!(c.to_hsla(), (0., 1., 0.5, 1.));
    assert_eq!(c.to_hwba(), (0., 0., 0., 1.));
    assert_eq!(c.to_linear_rgba(), (1., 0., 0., 1.));
    assert_eq!(c.to_linear_rgba_u8(), (255, 0, 0, 255));

    let c = Color::from_rgba(1., 0., 0., 0.5);
    assert_eq!(c.rgba(), (1., 0., 0., 0.5));
    assert_eq!(c.rgba_u8(), (255, 0, 0, 128));
    assert_eq!(c.to_hex_string(), "#ff000080");
    assert_eq!(c.to_rgb_string(), "rgba(255,0,0,0.5)");
    assert_eq!(c.to_string(), "RGBA(1,0,0,0.5)");

    let c = Color::from_rgb(0., 1., 0.);
    assert_eq!(c.to_hsva(), (120., 1., 1., 1.));
    assert_eq!(c.to_hsla(), (120., 1., 0.5, 1.));
    assert_eq!(c.to_hwba(), (120., 0., 0., 1.));

    let c = Color::from_rgb(0., 0., 1.);
    assert_eq!(c.to_hsva(), (240., 1., 1., 1.));
    assert_eq!(c.to_hsla(), (240., 1., 0.5, 1.));
    assert_eq!(c.to_hwba(), (240., 0., 0., 1.));

    let c = Color::from_rgb(0., 0., 0.6);
    assert_eq!(c.to_hsva(), (240., 1., 0.6, 1.));
    assert_eq!(c.to_hsla(), (240., 1., 0.3, 1.));
    assert_eq!(c.to_hwba(), (240., 0., 0.4, 1.));

    let c = Color::from_rgb(0.5, 0.5, 0.5);
    assert_eq!(c.to_hsva(), (0., 0., 0.5, 1.));
    assert_eq!(c.to_hsla(), (0., 0., 0.5, 1.));
    assert_eq!(c.to_hwba(), (0., 0.5, 0.5, 1.));

    #[cfg(feature = "lab")]
    {
        let c = Color::from_lab(0.0, 0.0, 0.0, 1.0);
        assert_eq!(c.rgba_u8(), (0, 0, 0, 255));

        let c = Color::from_lab(100.0, 0.0, 0.0, 1.0);
        assert_eq!(c.rgba_u8(), (255, 255, 255, 255));

        let c = Color::from_lch(0.0, 0.0, 0.0, 1.0);
        assert_eq!(c.rgba_u8(), (0, 0, 0, 255));

        let c = Color::from_lch(100.0, 0.0, 0.0, 1.0);
        assert_eq!(c.rgba_u8(), (255, 255, 255, 255));
    }

    assert_eq!(Color::default().rgba_u8(), (0, 0, 0, 255));

    assert_eq!(Color::try_from("#f00").unwrap().rgba_u8(), (255, 0, 0, 255));

    assert_eq!(Color::from((1., 0., 0., 0.5)).rgba_u8(), (255, 0, 0, 128));
    assert_eq!(Color::from((1., 0., 0.)).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(Color::from((255, 0, 0, 128)).rgba_u8(), (255, 0, 0, 128));
    assert_eq!(Color::from((255, 0, 0)).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(Color::from([1., 0., 0., 0.5]).rgba_u8(), (255, 0, 0, 128));
    assert_eq!(Color::from([1., 0., 0.]).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(Color::from([255, 0, 0, 128]).rgba_u8(), (255, 0, 0, 128));
    assert_eq!(Color::from([255, 0, 0]).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(
        Color::from([0.0_f32, 1.0, 0.5, 1.0]).to_rgba8(),
        [0, 255, 128, 255]
    );
    assert_eq!(
        Color::from([0.0_f32, 1.0, 0.5]).to_rgba8(),
        [0, 255, 128, 255]
    );

    // clamp
    let c = Color::new(1.23, 0.5, -0.01, 1.01);
    assert_eq!(c.to_array(), [1.23, 0.5, -0.01, 1.01]);
    assert_eq!(c.to_rgba8(), [255, 128, 0, 255]);
    assert_eq!(c.to_rgba16(), [65535, 32768, 0, 65535]);

    let c = Color::new(1.23, 0.5, -0.01, 1.01).clamp();
    assert_eq!(c.to_array(), [1.0, 0.5, 0.0, 1.0]);
    assert_eq!(c.to_rgba8(), [255, 128, 0, 255]);
    assert_eq!(c.to_rgba16(), [65535, 32768, 0, 65535]);
}

#[test]
fn red() {
    let data = &[
        Color::from_rgb(1., 0., 0.),
        Color::from_rgba(1., 0., 0., 1.),
        Color::from_rgb_u8(255, 0, 0),
        Color::from_rgba_u8(255, 0, 0, 255),
        Color::from_linear_rgb(1., 0., 0.),
        Color::from_linear_rgb_u8(255, 0, 0),
        Color::from_linear_rgba_u8(255, 0, 0, 255),
        Color::from_hsv(0., 1., 1.),
        Color::from_hsl(360., 1., 0.5),
        Color::from_hwb(0., 0., 0.),
        Color::from_oklab(0.6279151939969809, 0.2249032308661071, 0.12580287012451802),
        Color::from_html("#f00").unwrap(),
        Color::from_html("hsv(360,100%,100%)").unwrap(),
    ];
    for c in data {
        assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
    }
}

#[test]
fn interpolate() {
    let a = Color::from_rgb(0., 1., 0.);
    let b = Color::from_rgb(0., 0., 1.);

    assert_eq!(a.interpolate_rgb(&b, 0.0).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(a.interpolate_rgb(&b, 0.5).rgba_u8(), (0, 128, 128, 255));
    assert_eq!(a.interpolate_rgb(&b, 1.0).rgba_u8(), (0, 0, 255, 255));

    assert_eq!(b.interpolate_rgb(&a, 0.0).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(b.interpolate_rgb(&a, 0.5).rgba_u8(), (0, 128, 128, 255));
    assert_eq!(b.interpolate_rgb(&a, 1.0).rgba_u8(), (0, 255, 0, 255));

    assert_eq!(
        a.interpolate_linear_rgb(&b, 0.0).rgba_u8(),
        (0, 255, 0, 255)
    );
    assert_eq!(
        a.interpolate_linear_rgb(&b, 0.5).rgba_u8(),
        (0, 188, 188, 255)
    );
    assert_eq!(
        a.interpolate_linear_rgb(&b, 1.0).rgba_u8(),
        (0, 0, 255, 255)
    );

    assert_eq!(a.interpolate_hsv(&b, 0.0).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(a.interpolate_hsv(&b, 0.5).rgba_u8(), (0, 255, 255, 255));
    assert_eq!(a.interpolate_hsv(&b, 1.0).rgba_u8(), (0, 0, 255, 255));

    assert_eq!(a.interpolate_oklab(&b, 0.0).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(a.interpolate_oklab(&b, 0.5).rgba_u8(), (0, 170, 191, 255));
    assert_eq!(a.interpolate_oklab(&b, 1.0).rgba_u8(), (0, 0, 255, 255));

    #[cfg(feature = "lab")]
    {
        assert_eq!(a.interpolate_lab(&b, 0.0).rgba_u8(), (0, 255, 0, 255));
        assert_eq!(a.interpolate_lab(&b, 1.0).rgba_u8(), (0, 0, 255, 255));

        assert_eq!(a.interpolate_lch(&b, 0.0).rgba_u8(), (0, 255, 0, 255));
        assert_eq!(a.interpolate_lch(&b, 1.0).rgba_u8(), (0, 0, 255, 255));
    }
}
