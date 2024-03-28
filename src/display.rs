use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug)]
pub struct Show<T>(pub T);

impl<T: Display> Display for Show<Option<T>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(x) => write!(f, "{}", x),
            None => Ok(()),
        }
    }
}

impl<'ast, T, F: Fn(&mut fmt::Formatter, &'ast T) -> fmt::Result> Display for Show<(&'ast [T], F)> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0 .0 {
            (self.0 .1)(f, x)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Show<(T, usize)> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.0 .1 {
            write!(f, "{}", self.0 .0)?;
        }
        Ok(())
    }
}
