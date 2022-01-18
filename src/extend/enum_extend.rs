use strum::IntoEnumIterator;

pub fn generic_iterator<E, I, F>(pred: F)
  where E: IntoEnumIterator<Iterator=I>,
        I: Iterator<Item=E>,
        F: Fn(E) {
  for e in E::iter() {
    pred(e)
  }
}