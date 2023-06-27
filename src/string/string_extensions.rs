pub trait StringExt {
  fn substr(&self, index: usize, length: usize) -> Self;
  fn substring(&self, from: usize, to: usize) -> Self;
  fn left(&self, length: usize) -> Self;
  fn right(&self, length: usize) -> Self;
  fn reverse(&self) -> Self;
  fn index_of(&self, pat: &str, index: Option<usize>) -> Option<usize>;
  fn last_index_of(&self, pat: &str, index: Option<usize>) -> Option<usize>;
  fn char_at(&self, index: usize) -> Option<char>;
}

impl StringExt for String {
  fn substr(&self, index: usize, length: usize) -> Self {
    let mut stop_index = index + length;

    if stop_index > self.len() {
      stop_index = self.len()
    }

    String::from(&self[index..stop_index])
  }

  fn substring(&self, from: usize, to: usize) -> Self {
    self.chars().skip(from).take(to - from).collect::<Self>()
  }

  fn left(&self, length: usize) -> Self {
    self.substr(0, length)
  }

  fn right(&self, length: usize) -> Self {
    let index = if length >= self.len() {
      0
    } else {
      self.len() - length
    };
    self.substr(index, length)
  }

  fn reverse(&self) -> Self {
    self.chars().rev().collect()
  }

  fn index_of(&self, pat: &str, index: Option<usize>) -> Option<usize> {
    match index {
      Some(num) =>
        self.chars().skip(num).collect::<Self>().find(pat).map(|x| x + num),
      None => self.find(pat)
    }
  }

  fn last_index_of(&self, pat: &str, index: Option<usize>) -> Option<usize> {
    self
      .reverse()
      .index_of(pat, index)
      .map(|loc| self.len() - loc - 1)
  }

  fn char_at(&self, index: usize) -> Option<char> {
    self.chars().nth(index)
  }
}
