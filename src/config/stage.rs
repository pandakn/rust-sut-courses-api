use std::fmt;

use anyhow::Result;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Prodcution,
}
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Prodcution => "Prodcution",
        };
        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Prodcution" => Ok(Stage::Prodcution),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}
