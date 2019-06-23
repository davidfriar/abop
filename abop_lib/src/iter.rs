use std::iter::Peekable;

pub trait IterExt: Iterator {
    fn context_iter(self) -> ContextIter<Self, Self::Item>
    where
        Self: Sized;
}

pub struct ContextIter<I, T>
where
    I: Iterator<Item = T>,
{
    iter: Peekable<I>,
    prev: Option<T>,
}

impl<I, T> Iterator for ContextIter<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (Option<T>, T, Option<T>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(next) => {
                let before = self.prev.clone();
                let after: Option<T> = match self.iter.peek() {
                    None => None,
                    Some(x) => Some((*x).clone()),
                };
                self.prev = Some(next.clone());
                Some((before, next, after))
            }
        }
    }
}

impl<T> IterExt for T
where
    T: Iterator,
{
    fn context_iter(self) -> ContextIter<Self, Self::Item> {
        ContextIter {
            iter: self.peekable(),
            prev: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IterExt;

    #[test]
    fn testit() {
        let v = vec![1.0, 2.0, 3.0];
        let result: Vec<_> = v.into_iter().context_iter().collect::<Vec<_>>();
        eprintln!("{:?}", result);

        assert_eq!(
            result,
            vec![
                (None, 1.0, Some(2.0)),
                (Some(1.0), 2.0, Some(3.0)),
                (Some(2.0), 3.0, None)
            ]
        );
    }

}
