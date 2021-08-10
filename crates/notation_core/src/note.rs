use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Octave, Pitch, Semitones};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Note {
    pub octave: Octave,
    pub pitch: Pitch,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.octave, self.pitch)
    }
}

impl Note {
    pub fn new(octave: Octave, pitch: Pitch) -> Self {
        Self {
            octave,
            pitch,
        }
    }
}

impl From<(Octave, Pitch)> for Note {
    fn from(v: (Octave, Pitch)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<Note> for Semitones {
    fn from(v: Note) -> Self {
        let octave_val = Semitones::from(v.octave).0;
        let pitch_val = Semitones::from(v.pitch).0;
        Self(octave_val + pitch_val)
    }
}

impl From<Semitones> for Note {
    fn from(v: Semitones) -> Self {
        let octave = Octave::from(v);
        let pitch = Pitch::from(v);
        Self::new(octave, pitch)
    }
}

impl Note {
    pub const C_0: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P0,
    };
    pub const D_0: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P0,
    };
    pub const E_0: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P0,
    };
    pub const F_0: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P0,
    };
    pub const G_0: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P0,
    };
    pub const A_0: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P0,
    };
    pub const B_0: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P0,
    };
    pub const D_SHARP_0: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P0,
    };
    pub const F_SHARP_0: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P0,
    };
    pub const G_SHARP_0: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P0,
    };
    pub const A_SHARP_0: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P0,
    };

    pub const D_FLAT_0: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P0,
    };
    pub const E_FLAT_0: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P0,
    };
    pub const G_FLAT_0: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P0,
    };
    pub const A_FLAT_0: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P0,
    };
    pub const B_FLAT_0: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P0,
    };

    pub const C_1: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P1,
    };
    pub const D_1: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P1,
    };
    pub const E_1: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P1,
    };
    pub const F_1: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P1,
    };
    pub const G_1: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P1,
    };
    pub const A_1: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P1,
    };
    pub const B_1: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P1,
    };

    pub const C_SHARP_1: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P1,
    };
    pub const D_SHARP_1: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P1,
    };
    pub const F_SHARP_1: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P1,
    };
    pub const G_SHARP_1: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P1,
    };
    pub const A_SHARP_1: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P1,
    };

    pub const D_FLAT_1: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P1,
    };
    pub const E_FLAT_1: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P1,
    };
    pub const G_FLAT_1: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P1,
    };
    pub const A_FLAT_1: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P1,
    };
    pub const B_FLAT_1: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P1,
    };

    pub const C_2: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P2,
    };
    pub const D_2: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P2,
    };
    pub const E_2: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P2,
    };
    pub const F_2: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P2,
    };
    pub const G_2: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P2,
    };
    pub const A_2: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P2,
    };
    pub const B_2: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P2,
    };

    pub const C_SHARP_2: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P2,
    };
    pub const D_SHARP_2: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P2,
    };
    pub const F_SHARP_2: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P2,
    };
    pub const G_SHARP_2: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P2,
    };
    pub const A_SHARP_2: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P2,
    };

    pub const D_FLAT_2: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P2,
    };
    pub const E_FLAT_2: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P2,
    };
    pub const G_FLAT_2: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P2,
    };
    pub const A_FLAT_2: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P2,
    };
    pub const B_FLAT_2: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P2,
    };

    pub const C_3: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P3,
    };
    pub const D_3: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P3,
    };
    pub const E_3: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P3,
    };
    pub const F_3: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P3,
    };
    pub const G_3: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P3,
    };
    pub const A_3: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P3,
    };
    pub const B_3: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P3,
    };

    pub const C_SHARP_3: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P3,
    };
    pub const D_SHARP_3: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P3,
    };
    pub const F_SHARP_3: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P3,
    };
    pub const G_SHARP_3: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P3,
    };
    pub const A_SHARP_3: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P3,
    };

    pub const D_FLAT_3: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P3,
    };
    pub const E_FLAT_3: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P3,
    };
    pub const G_FLAT_3: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P3,
    };
    pub const A_FLAT_3: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P3,
    };
    pub const B_FLAT_3: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P3,
    };

    pub const C_4: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P4,
    };
    pub const D_4: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P4,
    };
    pub const E_4: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P4,
    };
    pub const F_4: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P4,
    };
    pub const G_4: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P4,
    };
    pub const A_4: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P4,
    };
    pub const B_4: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P4,
    };

    pub const C_SHARP_4: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P4,
    };
    pub const D_SHARP_4: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P4,
    };
    pub const F_SHARP_4: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P4,
    };
    pub const G_SHARP_4: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P4,
    };
    pub const A_SHARP_4: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P4,
    };

    pub const D_FLAT_4: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P4,
    };
    pub const E_FLAT_4: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P4,
    };
    pub const G_FLAT_4: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P4,
    };
    pub const A_FLAT_4: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P4,
    };
    pub const B_FLAT_4: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P4,
    };

    pub const C_5: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P5,
    };
    pub const D_5: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P5,
    };
    pub const E_5: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P5,
    };
    pub const F_5: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P5,
    };
    pub const G_5: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P5,
    };
    pub const A_5: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P5,
    };
    pub const B_5: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P5,
    };

    pub const C_SHARP_5: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P5,
    };
    pub const D_SHARP_5: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P5,
    };
    pub const F_SHARP_5: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P5,
    };
    pub const G_SHARP_5: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P5,
    };
    pub const A_SHARP_5: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P5,
    };

    pub const D_FLAT_5: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P5,
    };
    pub const E_FLAT_5: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P5,
    };
    pub const G_FLAT_5: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P5,
    };
    pub const A_FLAT_5: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P5,
    };
    pub const B_FLAT_5: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P5,
    };

    pub const C_6: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P6,
    };
    pub const D_6: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P6,
    };
    pub const E_6: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P6,
    };
    pub const F_6: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P6,
    };
    pub const G_6: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P6,
    };
    pub const A_6: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P6,
    };
    pub const B_6: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P6,
    };

    pub const C_SHARP_6: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P6,
    };
    pub const D_SHARP_6: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P6,
    };
    pub const F_SHARP_6: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P6,
    };
    pub const G_SHARP_6: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P6,
    };
    pub const A_SHARP_6: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P6,
    };

    pub const D_FLAT_6: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P6,
    };
    pub const E_FLAT_6: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P6,
    };
    pub const G_FLAT_6: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P6,
    };
    pub const A_FLAT_6: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P6,
    };
    pub const B_FLAT_6: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P6,
    };

    pub const C_7: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P7,
    };
    pub const D_7: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P7,
    };
    pub const E_7: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P7,
    };
    pub const F_7: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P7,
    };
    pub const G_7: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P7,
    };
    pub const A_7: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P7,
    };
    pub const B_7: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P7,
    };

    pub const C_SHARP_7: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P7,
    };
    pub const D_SHARP_7: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P7,
    };
    pub const F_SHARP_7: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P7,
    };
    pub const G_SHARP_7: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P7,
    };
    pub const A_SHARP_7: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P7,
    };

    pub const D_FLAT_7: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P7,
    };
    pub const E_FLAT_7: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P7,
    };
    pub const G_FLAT_7: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P7,
    };
    pub const A_FLAT_7: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P7,
    };
    pub const B_FLAT_7: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P7,
    };

    pub const C_8: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P8,
    };
    pub const D_8: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P8,
    };
    pub const E_8: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P8,
    };
    pub const F_8: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P8,
    };
    pub const G_8: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P8,
    };
    pub const A_8: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P8,
    };
    pub const B_8: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P8,
    };

    pub const C_SHARP_8: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P8,
    };
    pub const D_SHARP_8: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P8,
    };
    pub const F_SHARP_8: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P8,
    };
    pub const G_SHARP_8: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P8,
    };
    pub const A_SHARP_8: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P8,
    };

    pub const D_FLAT_8: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P8,
    };
    pub const E_FLAT_8: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P8,
    };
    pub const G_FLAT_8: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P8,
    };
    pub const A_FLAT_8: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P8,
    };
    pub const B_FLAT_8: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P8,
    };

    pub const C_9: Self = Self {
        pitch: Pitch::C,
        octave: Octave::P9,
    };
    pub const D_9: Self = Self {
        pitch: Pitch::D,
        octave: Octave::P9,
    };
    pub const E_9: Self = Self {
        pitch: Pitch::E,
        octave: Octave::P9,
    };
    pub const F_9: Self = Self {
        pitch: Pitch::F,
        octave: Octave::P9,
    };
    pub const G_9: Self = Self {
        pitch: Pitch::G,
        octave: Octave::P9,
    };
    pub const A_9: Self = Self {
        pitch: Pitch::A,
        octave: Octave::P9,
    };
    pub const B_9: Self = Self {
        pitch: Pitch::B,
        octave: Octave::P9,
    };

    pub const C_SHARP_9: Self = Self {
        pitch: Pitch::C_SHARP,
        octave: Octave::P9,
    };
    pub const D_SHARP_9: Self = Self {
        pitch: Pitch::D_SHARP,
        octave: Octave::P9,
    };
    pub const F_SHARP_9: Self = Self {
        pitch: Pitch::F_SHARP,
        octave: Octave::P9,
    };
    pub const G_SHARP_9: Self = Self {
        pitch: Pitch::G_SHARP,
        octave: Octave::P9,
    };
    pub const A_SHARP_9: Self = Self {
        pitch: Pitch::A_SHARP,
        octave: Octave::P9,
    };

    pub const D_FLAT_9: Self = Self {
        pitch: Pitch::D_FLAT,
        octave: Octave::P9,
    };
    pub const E_FLAT_9: Self = Self {
        pitch: Pitch::E_FLAT,
        octave: Octave::P9,
    };
    pub const G_FLAT_9: Self = Self {
        pitch: Pitch::G_FLAT,
        octave: Octave::P9,
    };
    pub const A_FLAT_9: Self = Self {
        pitch: Pitch::A_FLAT,
        octave: Octave::P9,
    };
    pub const B_FLAT_9: Self = Self {
        pitch: Pitch::B_FLAT,
        octave: Octave::P9,
    };
}

