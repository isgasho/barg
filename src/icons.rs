use rvg::*;

const BACK: &'static [u8] = include_bytes!("../rvg/back.svg.rvg");
const EXIT: &'static [u8] = include_bytes!("../rvg/exit.svg.rvg");
const FULLSCREEN: &'static [u8] = include_bytes!("../rvg/fullscreen.svg.rvg");
const GRID: &'static [u8] = include_bytes!("../rvg/grid.svg.rvg");
const HIDE: &'static [u8] = include_bytes!("../rvg/hide.svg.rvg");
const MENU: &'static [u8] = include_bytes!("../rvg/menu.svg.rvg");
const MORE: &'static [u8] = include_bytes!("../rvg/more.svg.rvg");
const NEW: &'static [u8] = include_bytes!("../rvg/new.svg.rvg");
const NEXT: &'static [u8] = include_bytes!("../rvg/next.svg.rvg");
const SEARCH: &'static [u8] = include_bytes!("../rvg/search.svg.rvg");
const VIEW: &'static [u8] = include_bytes!("../rvg/view.svg.rvg");
const ZOOM_IN: &'static [u8] = include_bytes!("../rvg/zoom_in.svg.rvg");
const ZOOM_OUT: &'static [u8] = include_bytes!("../rvg/zoom_out.svg.rvg");

pub fn text(pixels: &mut [crate::footile::Rgba8], width: u16, graphic_h: u16) {
    let font = fonterator::normal_font();

    let graphic_h = graphic_h / 2;

    let offs = {
        width / 2
    };

    let text = "";

//    let mut image = crate::Image::new(crate::Size(width, graphic_h));

    let back = (text.len() as f32 / 2.0) * (graphic_h as f32 / 2.0);

    // Render
    let mut p = crate::footile::Plotter::new(width as u32, graphic_h as u32 * 2);
    let r = crate::footile::RasterB::new(p.width(), p.height());
    let mut path = font.render(
        text, /*text*/
        (offs as f32 - back, graphic_h as f32 / 2.0),         /*position*/
        (graphic_h as f32, graphic_h as f32),     /*size*/
    );
    r.over(
        p.fill(&mut path, crate::footile::FillRule::NonZero),
        crate::footile::Rgba8::rgb(200, 200, 200), /*color*/
        unsafe { std::slice::from_raw_parts_mut(pixels.as_mut_ptr(), width as usize * graphic_h as usize * 2) },
    );
}

fn half(pixels: &mut [crate::footile::Rgba8], mut x: u16, width: u16, graphic_h: u16, slice: &[u8]) {
    let margin = graphic_h / 8;
    let graphic_width = (graphic_h / 2) - (margin);
    let ad = (graphic_h / 2) - (margin);

    let offs = if x > 6 {
        x = x - 6;
        width - (8 * ad)
    } else {
        0
    };
    render_from_rvg(slice, pixels, width, offs + x * ad, margin, graphic_width)
}

fn full(pixels: &mut [crate::footile::Rgba8], mut x: u16, width: u16, graphic_h: u16, slice: &[u8]) {
    let margin = graphic_h / 8;
    let graphic_width = (graphic_h) - (margin * 2);
    let ad = (graphic_h / 2) - (margin);

    let offs = if x > 6 {
        x = x - 6;
        width - (8 * ad)
    } else {
        0
    };
    render_from_rvg(slice, pixels, width, offs + x * ad, margin, graphic_width)
}

pub fn back(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, BACK);
}

pub fn next(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, NEXT);
}

pub fn menu(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, MENU);
}

pub fn exit(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    half(pixels, x, width, graphic_h, EXIT);
}

pub fn new(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, NEW);
}

pub fn more(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, MORE);
}

pub fn search(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, SEARCH);
}

pub fn grid(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, GRID);
}

pub fn hide(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, HIDE);
}

pub fn fullscreen(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, FULLSCREEN);
}

pub fn zoom_out(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, ZOOM_OUT);
}

pub fn zoom_in(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, ZOOM_IN);
}

pub fn view(pixels: &mut [crate::footile::Rgba8], x: u16, width: u16, graphic_h: u16) {
    full(pixels, x, width, graphic_h, VIEW);
}
