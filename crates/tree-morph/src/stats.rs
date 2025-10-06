use std::fmt::{self, Display};

pub struct Statistics {
    // (Checked, Applied)
    rule_count: Vec<(usize, usize)>,
    rule_names: Vec<String>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            rule_count: Vec::new(),
            rule_names: Vec::new(),
        }
    }

    pub fn set_rule_names(mut self, rule_names: Vec<String>) -> Self {
        self.rule_names = rule_names;
        self
    }

    pub fn rule_check(&mut self, index: usize) {
        if index >= self.rule_count.len() {
            self.rule_count.resize(index + 1, (0, 0));
        }
        let (checked, applied) = self.rule_count[index];
        self.rule_count[index] = (checked + 1, applied);
    }

    pub fn rule_applied(&mut self, index: usize) {
        let (checked, applied) = self.rule_count[index];
        self.rule_count[index] = (checked, applied + 1);
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("Morph Stats:");
        for (index, (checked, applied)) in self.rule_count.iter().enumerate() {
            let name: &str = match self.rule_names.get(index) {
                Some(name) => name,
                None => &format!("Unnamed Rule {}", index),
            };
            if let Err(err) = f.write_str(&format!(
                "{}\n\tChecked:\t{}\n\tApplied:\t{}\n\tApply Rate:\t{:.1}%\n",
                name,
                checked,
                applied,
                (*applied as f64 / *checked as f64) * 100.0
            )) {
                return Err(err);
            }
        }
        Ok(())
    }
}
