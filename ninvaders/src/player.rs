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
    pub fn move_right(&mut self, area_width: u16) {
        let rect_x = ((area_width / 2) as i16) + self.m_x_offset;
        if rect_x + PLAYER_WIDTH as i16 + 1 > area_width as i16 {
            self.m_x_offset = ((area_width / 2) as i16) - PLAYER_WIDTH as i16;
        } else {
            self.m_x_offset += 1;
        }
    }

    pub fn move_left(&mut self, area_width: u16) {
        let rect_x = ((area_width / 2) as i16) + self.m_x_offset;
        if rect_x < 0 {
            self.m_x_offset = -((area_width / 2) as i16);
        } else {
            self.m_x_offset -= 1;
        }
    }
}

fn calculate_player_x_position(current_x_offset: i16, area_width: u16) -> u16 {
    let rect_x = ((area_width / 2) as i16) + current_x_offset;
    if rect_x < 1 {
        1
    } else if rect_x + PLAYER_WIDTH as i16 + 1 > area_width as i16 {
        area_width - PLAYER_WIDTH - 1
    } else {
        rect_x as u16
    }
}

impl Widget for &Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_sprite = Text::styled(PLAYER_SPRITE, (Color::Yellow, Modifier::BOLD));
        let rect_y = (area.height as i16) + self.m_y_offset - 2;

        player_sprite.render(
            Rect::new(
                calculate_player_x_position(self.m_x_offset, area.width),
                rect_y as u16,
                PLAYER_WIDTH,
                PLAYER_HEIGHT,
            ),
            buf,
        );
    }
}
