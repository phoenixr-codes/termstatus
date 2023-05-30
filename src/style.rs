pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Color256(u8),
}

impl Color {
    #[inline]
    fn num(self) -> u8 {
        match self {
            Color::Black       => 0,
            Color::Red         => 1,
            Color::Green       => 2,
            Color::Yellow      => 3,
            Color::Blue        => 4,
            Color::Magenta     => 5,
            Color::Cyan        => 6,
            Color::White       => 7,
            Color::Color256(i) => i,
        }
    }
}

pub enum Effect {
    Bold,
    Dim,
    Italic,
    Underlined.
    Blink,
    BlinkFast,
    Reverse,
    Hidden,
    Strikethrough,
}

impl Effect {
    #[inline]
    fn num() -> u8 {
        match self {
            Effect::Bold =>          1,
            Effect::Dim =>           2,
            Effect::Italic =>        3,
            Effect::Underlined =>    4,
            Effect::Blink =>         5,
            Effect::BlinkFast =>     6,
            Effect::Reverse =>       7,
            Effect::Hidden =>        8,
            Effect::Strikethrough => 9,
        }
    }
}

pub enum Style {
    Color(Color),
    Effect(Effect),
}

impl TryFrom<&str> for Style {
    fn try_from(value: &str) -> Result<Self, &'static str> {
        if let Ok(c) = Color::try_from(value) {
            return Style::Color(c);
        }
        if let Ok(e) = Effect::t
    }
}

