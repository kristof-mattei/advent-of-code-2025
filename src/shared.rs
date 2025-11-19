use std::cmp::Ordering;

pub mod grids;
pub mod quest;
pub mod solution;
pub mod tree;
pub mod utils;

pub trait Quest {
    fn part_1(&self, input: &str) -> QuestSolution;
    fn part_2(&self, input: &str) -> QuestSolution;
    fn part_3(&self, input: &str) -> QuestSolution;
}

pub enum QuestSolution {
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    ISize(isize),
    USize(usize),
    String(String),
    Vec(Vec<String>),
    Manual,
    None,
}

impl std::fmt::Debug for QuestSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::I32(arg0) => write!(f, "{}i32", arg0),
            Self::U32(arg0) => write!(f, "{}u32", arg0),
            Self::I64(arg0) => write!(f, "{}i64", arg0),
            Self::U64(arg0) => write!(f, "{}u64", arg0),
            Self::ISize(arg0) => write!(f, "{}isize", arg0),
            Self::USize(arg0) => write!(f, "{}usize", arg0),
            Self::String(ref arg0) => write!(f, "\"{}\"", arg0),
            Self::Vec(ref arg0) => write!(f, "{:?}", arg0),
            Self::Manual => write!(f, "Manual"),
            Self::None => write!(f, "None"),
        }
    }
}

impl QuestSolution {
    #[must_use]
    pub fn has_solution(&self) -> bool {
        !matches!(*self, QuestSolution::None)
    }
}

impl PartialEq<QuestSolution> for QuestSolution {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *self {
            QuestSolution::I32(ref i) => i == other,
            QuestSolution::U32(ref i) => i == other,
            QuestSolution::I64(ref i) => i == other,
            QuestSolution::U64(ref i) => i == other,
            QuestSolution::ISize(ref i) => i == other,
            QuestSolution::USize(ref i) => i == other,
            QuestSolution::String(ref i) => i == other,
            QuestSolution::Vec(ref i) => i == other,
            QuestSolution::None => matches!(other, &QuestSolution::None),
            QuestSolution::Manual => matches!(other, &QuestSolution::Manual),
        }
    }
}

impl From<i32> for QuestSolution {
    fn from(v: i32) -> Self {
        QuestSolution::I32(v)
    }
}

impl From<u32> for QuestSolution {
    fn from(v: u32) -> Self {
        QuestSolution::U32(v)
    }
}

impl From<i64> for QuestSolution {
    fn from(v: i64) -> Self {
        QuestSolution::I64(v)
    }
}

impl From<u64> for QuestSolution {
    fn from(v: u64) -> Self {
        QuestSolution::U64(v)
    }
}

impl From<isize> for QuestSolution {
    fn from(v: isize) -> Self {
        QuestSolution::ISize(v)
    }
}

impl From<usize> for QuestSolution {
    fn from(v: usize) -> Self {
        QuestSolution::USize(v)
    }
}

impl From<Vec<String>> for QuestSolution {
    fn from(v: Vec<String>) -> Self {
        QuestSolution::Vec(v)
    }
}

impl From<String> for QuestSolution {
    fn from(v: String) -> Self {
        QuestSolution::String(v)
    }
}

impl From<&'_ str> for QuestSolution {
    fn from(v: &'_ str) -> Self {
        QuestSolution::String(v.into())
    }
}

impl From<Option<QuestSolution>> for QuestSolution {
    fn from(value: Option<QuestSolution>) -> Self {
        match value {
            Some(v) => v,
            None => QuestSolution::None,
        }
    }
}

impl std::fmt::Display for QuestSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match *self {
            QuestSolution::I32(other) => other.to_string(),
            QuestSolution::U32(other) => other.to_string(),
            QuestSolution::I64(other) => other.to_string(),
            QuestSolution::U64(other) => other.to_string(),
            QuestSolution::ISize(other) => other.to_string(),
            QuestSolution::USize(other) => other.to_string(),
            QuestSolution::String(ref other) => other.to_owned(),
            QuestSolution::Vec(ref other) => format!("\n{}", other.join("\n")),
            QuestSolution::Manual => "Manual".to_owned(),
            QuestSolution::None => "None".to_owned(),
        };

        write!(f, "{}", string)
    }
}

impl std::cmp::PartialEq<QuestSolution> for i32 {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::I32(other) => *self == other,
            QuestSolution::U32(other) => i64::from(*self) == i64::from(other),
            QuestSolution::I64(other) => i64::from(*self) == other,
            QuestSolution::U64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::USize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for u32 {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::I32(other) => i64::from(*self) == i64::from(other),
            QuestSolution::U32(other) => *self == other,
            QuestSolution::I64(other) => i64::from(*self) == other,
            QuestSolution::U64(other) => u64::from(*self) == other,
            QuestSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::USize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for i64 {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::I32(other) => *self == Self::from(other),
            QuestSolution::U32(other) => *self == Self::from(other),
            QuestSolution::I64(other) => *self == other,
            QuestSolution::U64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::USize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for u64 {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::I32(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::U32(other) => *self == Self::from(other),
            QuestSolution::I64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::U64(other) => *self == other,
            QuestSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::USize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for isize {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::I32(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::U32(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::I64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::U64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::ISize(other) => *self == other,
            QuestSolution::USize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for usize {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::I32(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::U32(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::I64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::U64(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            QuestSolution::USize(other) => *self == other,
            QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for String {
    fn eq(&self, other: &QuestSolution) -> bool {
        match *other {
            QuestSolution::String(ref s) => s == self,
            QuestSolution::I32(_)
            | QuestSolution::U32(_)
            | QuestSolution::I64(_)
            | QuestSolution::U64(_)
            | QuestSolution::ISize(_)
            | QuestSolution::USize(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<QuestSolution> for Vec<String> {
    fn eq(&self, other: &QuestSolution) -> bool {
        matches!(*other, QuestSolution::Vec(ref v) if {
            if v.len() != self.len() {
                return false;
            }

            for (l, r) in self.iter().zip(v) {
                if l != r {
                    return false;
                }
            }

            true
        })
    }
}

impl std::cmp::PartialOrd<QuestSolution> for i32 {
    fn partial_cmp(&self, other: &QuestSolution) -> Option<Ordering> {
        match *other {
            QuestSolution::I32(ref other) => self.partial_cmp(other),
            QuestSolution::U32(other) => i64::from(*self).partial_cmp(&i64::from(other)),
            QuestSolution::I64(ref other) => i64::from(*self).partial_cmp(other),
            QuestSolution::U64(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into the smaller i32, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            QuestSolution::USize(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into the smaller i32, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            QuestSolution::ISize(_)
            | QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<QuestSolution> for u32 {
    fn partial_cmp(&self, other: &QuestSolution) -> Option<Ordering> {
        match *other {
            QuestSolution::I32(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is negative, so we are by definition Greater
                    Some(Ordering::Greater)
                }
            },
            QuestSolution::U32(ref other) => self.partial_cmp(other),
            QuestSolution::I64(ref other) => i64::from(*self).partial_cmp(other),
            QuestSolution::U64(ref other) => u64::from(*self).partial_cmp(other),
            QuestSolution::USize(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into the smaller u32, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            QuestSolution::ISize(_)
            | QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<QuestSolution> for u64 {
    fn partial_cmp(&self, other: &QuestSolution) -> Option<Ordering> {
        match *other {
            QuestSolution::I32(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is negative, so we are by definition Greater
                    Some(Ordering::Greater)
                }
            },
            QuestSolution::U32(other) => self.partial_cmp(&u64::from(other)),
            QuestSolution::I64(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is negative, so we are by definition Greater
                    Some(Ordering::Greater)
                }
            },
            QuestSolution::U64(ref other) => self.partial_cmp(other),
            QuestSolution::USize(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into usize, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            QuestSolution::ISize(_)
            | QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<QuestSolution> for usize {
    fn partial_cmp(&self, other: &QuestSolution) -> Option<Ordering> {
        match *other {
            QuestSolution::I32(other) => {
                if other.is_negative() {
                    Some(Ordering::Greater)
                } else if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is positive, but doesn't fit into usize, so we're Less
                    Some(Ordering::Less)
                }
            },
            QuestSolution::U32(other) => {
                // if this fails, that means that usize is smaller than u32
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into usize
                    Some(Ordering::Less)
                }
            },
            QuestSolution::I64(other) => {
                if other.is_negative() {
                    Some(Ordering::Greater)
                } else if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is positive, but doesn't fit into usize, so we're Less
                    Some(Ordering::Less)
                }
            },
            QuestSolution::U64(other) => {
                // if this fails, that means that usize is smaller than u64
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into usize
                    Some(Ordering::Less)
                }
            },
            QuestSolution::USize(other) => other.partial_cmp(self),
            QuestSolution::ISize(_)
            | QuestSolution::String(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<QuestSolution> for String {
    fn partial_cmp(&self, other: &QuestSolution) -> Option<Ordering> {
        match *other {
            QuestSolution::String(ref s) => s.partial_cmp(self),
            QuestSolution::I32(_)
            | QuestSolution::U32(_)
            | QuestSolution::I64(_)
            | QuestSolution::U64(_)
            | QuestSolution::ISize(_)
            | QuestSolution::USize(_)
            | QuestSolution::Vec(_)
            | QuestSolution::Manual
            | QuestSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<QuestSolution> for Vec<String> {
    fn partial_cmp(&self, other: &QuestSolution) -> Option<Ordering> {
        match *other {
            QuestSolution::Vec(ref v) => {
                if v.len() != self.len() {
                    return None;
                }

                for (l, r) in self.iter().zip(v) {
                    if l != r {
                        return None;
                    }
                }

                Some(Ordering::Equal)
            },
            QuestSolution::I32(_)
            | QuestSolution::U32(_)
            | QuestSolution::I64(_)
            | QuestSolution::U64(_)
            | QuestSolution::ISize(_)
            | QuestSolution::USize(_)
            | QuestSolution::String(_)
            | QuestSolution::Manual
            | QuestSolution::None => None,
        }
    }
}
