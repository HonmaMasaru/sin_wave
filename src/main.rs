use core::f32::consts::PI;

const UNIT: i32 = 1;
const RATE: f32 = 200.0;

const IMG: &str = "onepiece17_doflamingo.png";
const WIDTH: i32 = 400;
const HEIGHT: i32 = 400;

fn main() {
    let mut style: String = "".to_string();
    let mut group: String = "".to_string();

    println!(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    println!(r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 {} {}">"#, HEIGHT, WIDTH);

    for y in 0..(HEIGHT / UNIT) {
        let x = (to_radian(y * UNIT).sin() * RATE * -1.0) as i32;
        style += &format!(r#"#g{}{{clip-path:url(#c{})}}"#, y, y);
        group += &format!(r##"<g><defs><rect id="r{}" x="0" y="{}" width="{}" height="{}"/></defs>"##, y, y * UNIT, WIDTH, UNIT);
        group += &format!(r##"<clipPath id="c{}"><use xlink:href="#r{}"/></clipPath>"##, y, y);
        group += &format!(r##"<g id="g{}" transform="translate({},0)"><image xlink:href="{}" width="{}" height="{}"></image></g></g>"##, y, x, IMG, WIDTH, HEIGHT);
    }
    print!(r#"<style type="text/css">{}</style>{}</svg>"#, style, group);
}

fn to_radian(deg: i32) -> f32 {
    (deg as f32) * PI / 180.0
}
