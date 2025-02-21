use iced::widget::container;
use iced::widget::container::Appearance;
use iced::{Border, Color, Shadow, Theme};

pub struct KeyboardKeyStyle {
    pub background: Color,
    pub border_color: Color,
    pub border_width: f32,
}

impl container::StyleSheet for KeyboardKeyStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Some(self.background.into()),
            border: Border {
                radius: 2.0.into(),
                width: self.border_width,
                color: self.border_color,
            },
            text_color: None,
            shadow: Shadow::default(),
        }
    }
}
