use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier},
    text::{Text},
    widgets::{Widget},
};

#[derive(Debug, Default)]
pub struct Player{
}

impl Widget for &Player{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_sprite = Text::styled("/-^-\\", (Color::Yellow, Modifier::BOLD));
        player_sprite.render(Rect::new(10, 10, area.width, 1), buf);
    }
}
