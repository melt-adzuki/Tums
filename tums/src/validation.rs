use crate::consts::{UNISTR_FIRST, UNISTR_LAST, UNISTR_SECOND};

pub(crate) trait IsUni {
    fn is_uni(&self) -> bool;
}

impl IsUni for Vec<&str> {
    fn is_uni(&self) -> bool {
        self.first().is_some_and(|s| *s == UNISTR_FIRST)
            && self.get(1).is_some_and(|s| *s == UNISTR_SECOND)
            && self.last().is_some_and(|s| *s == UNISTR_LAST)
    }
}
