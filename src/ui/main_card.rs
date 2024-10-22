use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::repository::deck::CardItem;

pub struct MainCard {
    pub is_revealed: bool,
    pub content: CardItem
}

impl Widget for MainCard {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let name_text = Paragraph::new(self.content.name)
            .left_aligned()
            .bold();
        let glance_text = match self.content.glance {
            None => Paragraph::new(" "),
            Some(glance) => Paragraph::new(format!("glance: {}", glance))
        };
        let mut content_text = Paragraph::new(self.content.content)
            .left_aligned()
            .wrap(Wrap { trim: true })
            .white();
        if self.is_revealed {
            content_text = content_text.on_white();
        }
        let tags_text = match self.content.tags {
            None => Paragraph::new(" "),
            Some(tags) => {
                let tags_str = tags.join(", ");
                Paragraph::new(format!("tags: {}", tags_str))
            }
        };
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(4),
                    Constraint::Length(1),
                ]
            )
            .split(area);
        name_text.render(chunks[0], buf);
        glance_text.render(chunks[1], buf);
        content_text.render(chunks[2], buf);
        tags_text.render(chunks[3], buf);
    }
}