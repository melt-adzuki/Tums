use std::ops::Not;

use anyhow::*;
use regex::Regex;

pub(crate) trait IsUni {
    fn is_uni(&self) -> Result<bool>;
}

impl IsUni for Vec<String> {
    fn is_uni(&self) -> Result<bool> {
        if self
            .first()
            .zip(self.last())
            .is_some_and(|s| {
                s.0.starts_with("思慮深いウニ「俺が思うに...」")
                    && s.1.ends_with(
                        "大気圏に突入する犬「ウオオオオオオオオオオオオオオオオオオオオオオオオ",
                    )
            })
            .not()
        {
            return Ok(false);
        }

        for line in self[1..self.len() - 1].iter() {
            let reg = Regex::new(r"^(\S|\s)+「(\S|\s)+」$")?;

            if reg.is_match(line) {
                continue;
            }

            return Ok(false);
        }

        Ok(true)
    }
}
