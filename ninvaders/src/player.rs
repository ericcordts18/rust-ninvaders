use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier},
    text::Text,
    widgets::Widget,
};

const PLAYER_HEIGHT: u16 = 1;
const PLAYER_WIDTH: u16 = 5;
const PLAYER_SPRITE: &str = "/-^-\\";

#[derive(Debug, Default)]
pub struct Player {
    m_x_offset: i16,
    m_y_offset: i16,
}

impl Player {
    pub fn move_right(&mut self) {
        self.m_x_offset += 1;
    }

    pub fn move_left(&mut self) {
        self.m_x_offset -= 1;
    }
}

impl Widget for &Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_sprite = Text::styled(PLAYER_SPRITE, (Color::Yellow, Modifier::BOLD));
        let rect_x = ((area.width / 2) as i16) + self.m_x_offset - 1;
        let rect_y = (area.height as i16) + self.m_y_offset - 2;

        player_sprite.render(
            Rect::new(rect_x as u16, rect_y as u16, PLAYER_WIDTH, PLAYER_HEIGHT),
            buf,
        );
    }
}
