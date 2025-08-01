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
    /*
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
    */
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
                let mut custom_parts = s.trim().splitn(2, ':');
                let raw_inc_or_exp = custom_parts.next().ok_or(TransactionErr::CategoryParse)?;
                let raw_name = custom_parts.next().ok_or(TransactionErr::CategoryParse)?;

                let inc_or_exp = {
                    if raw_inc_or_exp.eq_ignore_ascii_case("income") {
                        "Income"
                    } else {
                        "Expense"
                    }
                };

                Ok(Category::Custom {
                    income_or_expense: inc_or_exp.trim().to_string(),
                    name: raw_name.trim().to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_parsing() {
        let test_cases = vec![
            ("food", Category::Food),
            ("FOOD", Category::Food),
            ("Food", Category::Food),
            ("transport", Category::Transport),
            ("entertainment", Category::Entertainment),
            ("shopping", Category::Shopping),
            ("bills", Category::Bills),
            ("healthcare", Category::Healthcare),
            ("automotive", Category::Automotive),
            ("salary", Category::Salary),
            ("investment", Category::Investment),
            ("freelance", Category::Freelance),
        ];

        for (input, expected) in test_cases {
            let result: Result<Category, _> = input.parse();
            assert!(result.is_ok(), "Failed to parse '{}'", input);
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn test_custom_category_parsing() {
        let result: Result<Category, _> = "income:Bonus".parse();
        assert!(result.is_ok());
        
        match result.unwrap() {
            Category::Custom { income_or_expense, name } => {
                assert_eq!(income_or_expense, "Income");
                assert_eq!(name, "Bonus");
            }
            _ => panic!("Expected Custom category"),
        }
    }

    #[test]
    fn test_custom_category_parsing_expense() {
        let result: Result<Category, _> = "expense:Pet Care".parse();
        assert!(result.is_ok());
        
        match result.unwrap() {
            Category::Custom { income_or_expense, name } => {
                assert_eq!(income_or_expense, "Expense");
                assert_eq!(name, "Pet Care");
            }
            _ => panic!("Expected Custom category"),
        }
    }

    #[test]
    fn test_custom_category_parsing_case_insensitive() {
        let result: Result<Category, _> = "INCOME:Bonus".parse();
        assert!(result.is_ok());
        
        match result.unwrap() {
            Category::Custom { income_or_expense, name } => {
                assert_eq!(income_or_expense, "Income");
                assert_eq!(name, "Bonus");
            }
            _ => panic!("Expected Custom category"),
        }
    }

    #[test]
    fn test_invalid_category_parsing() {
        let result: Result<Category, _> = "InvalidCategory".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_category_invalid_format() {
        let result: Result<Category, _> = "income".parse(); // Missing colon and name
        assert!(result.is_err());
    }

    #[test]
    fn test_category_display() {
        let test_cases = vec![
            (Category::Food, "Food"),
            (Category::Transport, "Transport"),
            (Category::Entertainment, "Entertainment"),
            (Category::Shopping, "Shopping"),
            (Category::Bills, "Bills"),
            (Category::Healthcare, "Healthcare"),
            (Category::Automotive, "Automotive"),
            (Category::Salary, "Salary"),
            (Category::Investment, "Investment"),
            (Category::Freelance, "Freelance"),
        ];

        for (category, expected) in test_cases {
            assert_eq!(category.to_string(), expected);
        }
    }

    #[test]
    fn test_custom_category_display() {
        let custom = Category::Custom {
            income_or_expense: "Income".to_string(),
            name: "Bonus".to_string(),
        };
        assert_eq!(custom.to_string(), "Bonus");
    }

    /*
    #[test]
    fn test_is_income() {
        // Income categories
        assert!(Category::Salary.is_income());
        assert!(Category::Investment.is_income());
        assert!(Category::Freelance.is_income());
        
        let income_custom = Category::Custom {
            income_or_expense: "Income".to_string(),
            name: "Bonus".to_string(),
        };
        assert!(income_custom.is_income());

        // Expense categories
        assert!(!Category::Food.is_income());
        assert!(!Category::Transport.is_income());
        assert!(!Category::Entertainment.is_income());
        assert!(!Category::Shopping.is_income());
        assert!(!Category::Bills.is_income());
        assert!(!Category::Healthcare.is_income());
        assert!(!Category::Automotive.is_income());
        
        let expense_custom = Category::Custom {
            income_or_expense: "Expense".to_string(),
            name: "Pet Care".to_string(),
        };
        assert!(!expense_custom.is_income());
    }

    #[test]
    fn test_category_type() {
        // Income categories
        assert_eq!(Category::Salary.category_type(), "Income");
        assert_eq!(Category::Investment.category_type(), "Income");
        assert_eq!(Category::Freelance.category_type(), "Income");
        
        let income_custom = Category::Custom {
            income_or_expense: "Income".to_string(),
            name: "Bonus".to_string(),
        };
        assert_eq!(income_custom.category_type(), "Income");

        // Expense categories
        assert_eq!(Category::Food.category_type(), "Expense");
        assert_eq!(Category::Transport.category_type(), "Expense");
        assert_eq!(Category::Entertainment.category_type(), "Expense");
        assert_eq!(Category::Shopping.category_type(), "Expense");
        assert_eq!(Category::Bills.category_type(), "Expense");
        assert_eq!(Category::Healthcare.category_type(), "Expense");
        assert_eq!(Category::Automotive.category_type(), "Expense");
        
        let expense_custom = Category::Custom {
            income_or_expense: "Expense".to_string(),
            name: "Pet Care".to_string(),
        };
        assert_eq!(expense_custom.category_type(), "Expense");
    }
    */

    #[test]
    fn test_category_serialization() {
        let category = Category::Food;
        let json = serde_json::to_string(&category).unwrap();
        let deserialized: Category = serde_json::from_str(&json).unwrap();
        assert_eq!(category, deserialized);
    }

    #[test]
    fn test_custom_category_serialization() {
        let category = Category::Custom {
            income_or_expense: "Income".to_string(),
            name: "Bonus".to_string(),
        };
        let json = serde_json::to_string(&category).unwrap();
        let deserialized: Category = serde_json::from_str(&json).unwrap();
        assert_eq!(category, deserialized);
    }
}
