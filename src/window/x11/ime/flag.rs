use std::ops::BitOr;

pub enum Capabilities {
    PreeditText = 1 << 0,
    // AuxiliaryText = 1 << 1,
    // LookupTable = 1 << 2,
    Focus = 1 << 3,
    // Property = 1 << 4,
    // SurroundingText = 1 << 5,
}

impl BitOr for Capabilities {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}


pub enum Modifiers {
    Empty = 0,
    // Shift = 1 << 0,

    /// Caps Lock
    // Lock = 1 << 1,
    // Control = 1 << 2,
    // Mod1 = 1 << 3,

    /// Num Lock
    // Mod2 = 1 << 4,
    // Mod3 = 1 << 5,
    // Mod4 = 1 << 6,
    // Mod5 = 1 << 7,
    // Button1 = 1 << 8,
    // Button2 = 1 << 9,
    // Button3 = 1 << 10,
    // Button4 = 1 << 11,
    // Button5 = 1 << 12,

    // Handled = 1 << 24,
    // // Forward = 1 << 25,
    // Ignored = 1 << 25,
    //
    // Super = 1 << 26,
    // Hyper = 1 << 27,
    // Meta = 1 << 28,

    Release = 1 << 30,
}

impl BitOr for Modifiers {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}