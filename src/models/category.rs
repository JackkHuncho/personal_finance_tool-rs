use super::transaction::TransactionErr;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Food,
    Transport,
    Entertainment,
    Shopping,
    Bills,
    Healthcare,
    Automotive,
    Salary,
    Investment,
    Freelance,
    Custom {
        income_or_expense: String,
        name: String,
    },
}

impl Category {
    pub fn is_income(&self) -> bool {
        match self {
            Self::Salary => true,
            Self::Investment => true,
            Self::Freelance => true,
            Self::Custom {
                income_or_expense,
                name: _,
            } => {
                if income_or_expense.eq_ignore_ascii_case("income") {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn category_type(&self) -> &'static str {
        match self.is_income() {
            true => "Income",
            false => "Expense",
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Food => write!(f, "Food"),
            Self::Transport => write!(f, "Transport"),
            Self::Entertainment => write!(f, "Entertainment"),
            Self::Shopping => write!(f, "Shopping"),
            Self::Bills => write!(f, "Bills"),
            Self::Healthcare => write!(f, "Healthcare"),
            Self::Automotive => write!(f, "Automotive"),
            Self::Salary => write!(f, "Salary"),
            Self::Investment => write!(f, "Investment"),
            Self::Freelance => write!(f, "Freelance"),
            Self::Custom {
                income_or_expense: _,
                name,
            } => write!(f, "{name}"),
        }
    }
}

impl std::str::FromStr for Category {
    type Err = TransactionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let up = s.trim().to_uppercase();
        match up.as_str() {
            "FOOD" => Ok(Category::Food),
            "TRANSPORT" => Ok(Category::Transport),
            "ENTERTAINMENT" => Ok(Category::Entertainment),
            "SHOPPING" => Ok(Category::Shopping),
            "BILLS" => Ok(Category::Bills),
            "HEALTHCARE" => Ok(Category::Healthcare),
            "AUTOMOTIVE" => Ok(Category::Automotive),
            "SALARY" => Ok(Category::Salary),
            "INVESTMENT" => Ok(Category::Investment),
            "FREELANCE" => Ok(Category::Freelance),
            _ => {
                let mut parts = s.trim().splitn(2, ':');
                let raw_flag = parts.next().ok_or(TransactionErr::CategoryParse)?;
                let raw_name = parts.next().ok_or(TransactionErr::CategoryParse)?;

                let flag = {
                    if raw_flag.eq_ignore_ascii_case("income") {
                        "Income"
                    } else {
                        "Expense"
                    }
                };

                Ok(Category::Custom {
                    income_or_expense: flag.trim().to_string(),
                    name: raw_name.trim().to_string(),
                })
            }
        }
    }
}
