pub mod molecule;
pub mod daylight;

// https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod test_readme {
  macro_rules! external_doc_test {
    ($x:expr) => {
        #[doc = $x]
        extern {}
    };
  }

  external_doc_test!(include_str!("../README.md"));
}