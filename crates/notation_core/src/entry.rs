use serde::{Serialize, Deserialize};

use crate::prelude::{Duration, Note, Solfege, Chord, Roman, Signature, Tempo};

pub trait Entry {
    fn duration(&self) -> Duration {
        Duration::Zero
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum CoreEntry {
    Rest (Duration),
    Note (Note, Duration),
    Solfege (Solfege, Duration),
    Chord (Chord, Duration),
    Roman (Roman, Duration),
    Signature (Signature),
    Tempo (Tempo),
}

impl CoreEntry {
    pub fn duration(&self) -> Duration {
        match self {
            CoreEntry::Rest(duration) => *duration,
            CoreEntry::Note(_, duration) => *duration,
            CoreEntry::Solfege(_, duration) => *duration,
            CoreEntry::Chord(_, duration) => *duration,
            CoreEntry::Roman(_, duration) => *duration,
            CoreEntry::Signature(_) => Duration::Zero,
            CoreEntry::Tempo(_) => Duration::Zero,
        }
    }
}

impl Entry for CoreEntry {
    fn duration(&self) -> Duration {
        self.duration()
    }
}

impl CoreEntry {
    /// Returns `true` if the entry is [`Rest`].
    pub fn is_rest(&self) -> bool {
        matches!(self, Self::Rest(..))
    }

    /// Returns `true` if the entry is [`Note`].
    pub fn is_note(&self) -> bool {
        matches!(self, Self::Note(..))
    }

    /// Returns `true` if the entry is [`Solfege`].
    pub fn is_solfege(&self) -> bool {
        matches!(self, Self::Solfege(..))
    }

    /// Returns `true` if the entry is [`Chord`].
    pub fn is_chord(&self) -> bool {
        matches!(self, Self::Chord(..))
    }

    /// Returns `true` if the entry is [`Roman`].
    pub fn is_roman(&self) -> bool {
        matches!(self, Self::Roman(..))
    }

    /// Returns `true` if the entry is [`Signature`].
    pub fn is_signature(&self) -> bool {
        matches!(self, Self::Signature(..))
    }

    /// Returns `true` if the entry is [`Tempo`].
    pub fn is_tempo(&self) -> bool {
        matches!(self, Self::Tempo(..))
    }
}

impl CoreEntry {
    pub fn as_rest(&self) -> Option<&Duration> {
        if let Self::Rest(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_note(&self) -> Option<&Note> {
        if let Self::Note(v, _) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_solfege(&self) -> Option<&Solfege> {
        if let Self::Solfege(v, _) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_chord(&self) -> Option<&Chord> {
        if let Self::Chord(v, _) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_roman(&self) -> Option<&Roman> {
        if let Self::Roman(v, _) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Duration> for CoreEntry {
    fn from(v: Duration) -> Self {
        Self::Rest(v)
    }
}

impl From<(Note, Duration)> for CoreEntry {
    fn from(v: (Note, Duration)) -> Self {
        Self::Note(v.0, v.1)
    }
}

impl From<(Solfege, Duration)> for CoreEntry {
    fn from(v: (Solfege, Duration)) -> Self {
        Self::Solfege(v.0, v.1)
    }
}

impl From<(Chord, Duration)> for CoreEntry {
    fn from(v: (Chord, Duration)) -> Self {
        Self::Chord(v.0, v.1)
    }
}

impl From<(Roman, Duration)> for CoreEntry {
    fn from(v: (Roman, Duration)) -> Self {
        Self::Roman(v.0, v.1)
    }
}

impl From<Signature> for CoreEntry {
    fn from(v: Signature) -> Self {
        Self::Signature(v)
    }
}

impl From<Tempo> for CoreEntry {
    fn from(v: Tempo) -> Self {
        Self::Tempo(v)
    }
}
