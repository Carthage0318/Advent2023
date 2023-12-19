use crate::AdventErr::{Compute, InputParse};
use crate::{AdventErr, AdventResult};
use std::cmp::{max, min};

#[derive(Debug, Copy, Clone)]
pub(super) struct Part {
    pub(super) x: u64,
    pub(super) m: u64,
    pub(super) a: u64,
    pub(super) s: u64,
}

impl Part {
    fn get_value_for(self, category: Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    pub(super) fn rating_sum(self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Category {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Category {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => return Err(InputParse(format!("Unrecognized category '{value}'"))),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Destination {
    Workflow(usize),
    Accept,
    Reject,
}

#[derive(Debug, Copy, Clone)]
pub(super) enum Rule {
    LessThan(Category, u64, Destination),
    GreaterThan(Category, u64, Destination),
    Jump(Destination),
}

impl Rule {
    fn apply_to(self, part: Part) -> Option<Destination> {
        match self {
            Self::LessThan(category, value, destination) => {
                if part.get_value_for(category) < value {
                    Some(destination)
                } else {
                    None
                }
            }

            Self::GreaterThan(category, value, destination) => {
                if part.get_value_for(category) > value {
                    Some(destination)
                } else {
                    None
                }
            }

            Self::Jump(destination) => Some(destination),
        }
    }
}

pub(super) struct Workflow {
    pub(super) name: String,
    pub(super) rules: Vec<Rule>,
}

impl Workflow {
    pub(super) fn apply_to(&self, part: Part) -> AdventResult<Destination> {
        for rule in &self.rules {
            if let Some(destination) = rule.apply_to(part) {
                return Ok(destination);
            }
        }

        Err(Compute(format!(
            "Fell through all rules of workflow {}",
            self.name
        )))
    }

    pub(super) fn empty(name: String) -> Self {
        Self {
            name,
            rules: vec![],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum SortResult {
    Accepted,
    Rejected,
}

#[derive(Debug, Copy, Clone)]
pub(super) struct CopyRange<T: Copy> {
    pub(super) start: T,
    pub(super) end: T,
}

impl<T: Copy> CopyRange<T> {
    pub(super) fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl CopyRange<u64> {
    fn restrict(self, restriction: Restriction) -> Option<Self> {
        match restriction {
            Restriction::LessThan(value) => {
                if self.start >= value {
                    None
                } else {
                    Some(CopyRange::new(self.start, min(self.end, value)))
                }
            }

            Restriction::GreaterThan(value) => {
                if self.end <= value + 1 {
                    None
                } else {
                    Some(CopyRange::new(max(self.start, value + 1), self.end))
                }
            }

            Restriction::LessOrEqual(value) => {
                if self.start > value {
                    None
                } else {
                    Some(CopyRange::new(self.start, min(self.end, value + 1)))
                }
            }

            Restriction::GreaterOrEqual(value) => {
                if self.end <= value {
                    None
                } else {
                    Some(CopyRange::new(max(self.start, value), self.end))
                }
            }
        }
    }

    pub(super) fn size(self) -> u64 {
        self.end - self.start
    }
}

#[derive(Debug, Copy, Clone)]
enum Restriction {
    LessThan(u64),
    GreaterThan(u64),
    LessOrEqual(u64),
    GreaterOrEqual(u64),
}

impl Restriction {
    fn invert(self) -> Self {
        match self {
            Restriction::LessThan(x) => Restriction::GreaterOrEqual(x),
            Restriction::GreaterThan(x) => Restriction::LessOrEqual(x),
            Restriction::GreaterOrEqual(x) => Restriction::LessThan(x),
            Restriction::LessOrEqual(x) => Restriction::GreaterThan(x),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct PartRange {
    x: CopyRange<u64>,
    m: CopyRange<u64>,
    a: CopyRange<u64>,
    s: CopyRange<u64>,
}

impl PartRange {
    pub(super) fn new(valid_range: CopyRange<u64>) -> Self {
        Self {
            x: valid_range,
            m: valid_range,
            a: valid_range,
            s: valid_range,
        }
    }

    pub(super) fn apply_rule(mut self, rule: Rule) -> Option<(Self, Destination)> {
        match rule {
            Rule::Jump(destination) => Some((self, destination)),
            Rule::LessThan(category, value, destination) => {
                let current_range = self.get(category);
                let new_range = current_range.restrict(Restriction::LessThan(value))?;
                *self.get_mut(category) = new_range;
                Some((self, destination))
            }
            Rule::GreaterThan(category, value, destination) => {
                let current_range = self.get(category);
                let new_range = current_range.restrict(Restriction::GreaterThan(value))?;
                *self.get_mut(category) = new_range;
                Some((self, destination))
            }
        }
    }

    pub(super) fn apply_rule_inverse(mut self, rule: Rule) -> Option<Self> {
        match rule {
            Rule::Jump(_) => Some(self),
            Rule::LessThan(category, value, _) => {
                let current_range = self.get(category);
                let new_range = current_range.restrict(Restriction::LessThan(value).invert())?;
                *self.get_mut(category) = new_range;
                Some(self)
            }
            Rule::GreaterThan(category, value, _) => {
                let current_range = self.get(category);
                let new_range = current_range.restrict(Restriction::GreaterThan(value).invert())?;
                *self.get_mut(category) = new_range;
                Some(self)
            }
        }
    }

    fn get(self, category: Category) -> CopyRange<u64> {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn get_mut(&mut self, category: Category) -> &mut CopyRange<u64> {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    pub(super) fn size(self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}
