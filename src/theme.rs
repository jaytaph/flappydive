type Color = (u8, u8, u8);

pub struct Theme {
    pub sand: Color,
    pub sand_highlight: Color,
    pub water: Color,
    pub pipes: Color,
    pub bubbles: Color,
    pub text: Color,

    pub fauna_color_1: Color,
    pub fauna_color_2: Color,
    pub fauna_color_3: Color,
    pub fauna_color_4: Color,
}

pub const MAX_BUBBLES: usize = 15;

pub const COLOR_THEME: Theme = Theme {
    sand: (244, 214, 164),
    sand_highlight: (178, 147, 114),
    water: (74, 179, 219),
    pipes: (111, 191, 115),
    bubbles: (136, 207, 241),

    text: (116, 100, 076),

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

    fauna_color_1: (255, 87, 51),   // Bright, bold red-orange for fauna
    fauna_color_2: (46, 204, 113),  // Fresh spring green for plants
    fauna_color_3: (240, 147, 43),  // Warm amber orange for coral/fauna
    fauna_color_4: (128, 90, 213),  // Bold violet for fauna or shells
};