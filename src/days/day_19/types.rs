use crate::AdventErr::{Compute, InputParse};
use crate::{AdventErr, AdventResult};

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
