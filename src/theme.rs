type Color = (u8, u8, u8);

#[allow(dead_code)]
pub struct Theme {
    pub sand: Color,                // Sand color at the bottom
    pub sand_highlight: Color,      // Highlighted sand color
    pub water: Color,               // Water / ocean color
    pub pipes: Color,               // Color of the pipes
    pub bubbles: Color,             // Color of the water bubbles
    pub   text: Color,                // Text on the screen
    pub sub: Color,                 // Submarine color

    pub fauna_color_1: Color,
    pub fauna_color_2: Color,
    pub fauna_color_3: Color,
    pub fauna_color_4: Color,
}

pub const COLOR_THEME: Theme = Theme {
    sand: (244, 214, 164),
    sand_highlight: (178, 147, 114),
    water: (74, 179, 219),
    pipes: (111, 191, 115),
    bubbles: (136, 207, 241),

    text: (116, 100, 076),
    sub: (128, 128, 255),

    fauna_color_1: (242, 140, 140),
    fauna_color_2: (42, 123, 79),
    fauna_color_3: (255, 111, 97),
    fauna_color_4: (139, 111, 169),
};

pub const GRAYSCALE_THEME: Theme = Theme {
    sand: (213, 213, 213),          // Grayscale of (244, 214, 164)
    sand_highlight: (155, 155, 155), // Grayscale of (178, 147, 114)
    water: (142, 142, 142),         // Grayscale of (74, 179, 219)
    pipes: (161, 161, 161),         // Grayscale of (111, 191, 115)
    bubbles: (196, 196, 196),       // Grayscale of (136, 207, 241)

    text: (104, 104, 104),          // Grayscale of (116, 100, 76)
    sub: (128, 128, 128),

    fauna_color_1: (161, 161, 161), // Grayscale of (242, 140, 140)
    fauna_color_2: (93, 93, 93),    // Grayscale of (42, 123, 79)
    fauna_color_3: (144, 144, 144), // Grayscale of (255, 111, 97)
    fauna_color_4: (116, 116, 116), // Grayscale of (139, 111, 169)
};

pub const THEME: Theme = Theme {
    sand: (245, 203, 123),          // Warm yellowish sand
    sand_highlight: (214, 163, 92), // Darker sandy color for highlights
    water: (93, 188, 210),          // Light cyan blue for water
    pipes: (129, 199, 132),         // Fresh mint green for pipes
    bubbles: (171, 222, 239),       // Soft pale blue for bubbles

    text: (89, 80, 66),             // Warm brown for readable text
    sub: (128, 128, 255),

    fauna_color_1: (243, 156, 18),  // Bright orange for coral/creatures
    fauna_color_2: (39, 174, 96),   // Bold green for plants
    fauna_color_3: (231, 76, 60),   // Vibrant red for fauna
    fauna_color_4: (155, 89, 182),  // Soft purple for shells or fauna
};

pub const THEME_2: Theme = Theme {
    sand: (232, 198, 135),          // Soft beige with a hint of gold
    sand_highlight: (192, 157, 104), // Darker tan for sand highlights
    water: (89, 168, 245),          // Bright sky blue for water
    pipes: (84, 153, 124),          // Teal green for pipes
    bubbles: (156, 209, 247),       // Light blue for bubbles

    text: (70, 63, 55),             // Deep taupe for readable text
    sub: (128, 128, 255),

    fauna_color_1: (255, 87, 51),   // Bright, bold red-orange for fauna
    fauna_color_2: (46, 204, 113),  // Fresh spring green for plants
    fauna_color_3: (240, 147, 43),  // Warm amber orange for coral/fauna
    fauna_color_4: (128, 90, 213),  // Bold violet for fauna or shells
};

pub struct ThemeSwitcher {
    current_theme_idx: usize,
    themes: Vec<Theme>,
}

impl ThemeSwitcher {
    pub(crate) fn new() -> Self {
        Self {
            current_theme_idx: 0,
            themes: vec![COLOR_THEME, GRAYSCALE_THEME, THEME, THEME_2],
        }
    }

    pub(crate) fn next(&mut self) -> &Theme {
        self.current_theme_idx = (self.current_theme_idx + 1) % self.themes.len();
        &self.themes[self.current_theme_idx]
    }

    pub(crate) fn current(&self) -> &Theme {
        &self.themes[self.current_theme_idx]
    }
}