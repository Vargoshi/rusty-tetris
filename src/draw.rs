use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureQuery}, ttf::Font, video::Window};

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub fn draw_text(
    canvas: &mut Canvas<Window>,
    font: &Font,
    pos: Point,
    text: &str,
    size: f32,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            pos.x,
            pos.y,
            (width as f32 * size) as u32,
            (height as f32 * size) as u32,
        )),
    )?;

    Ok(())
}
