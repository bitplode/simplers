pub mod string;

#[cfg(test)]
mod tests {
  use crate::string::string_extensions::StringExt;

  #[test]
  fn test_index_of() {
    let name = "Joe Schmo ain't no ordinary Schmo".to_string();
    let loc = name.index_of("Schmo", None);
    assert_eq!(loc, Some(4));

    let loc2 = name.index_of("Schmo", Some(loc.unwrap() + 1));
    assert_eq!(loc2, Some(28));
  }
}
