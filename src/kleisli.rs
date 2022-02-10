pub struct Kleisli<A, B> {
  run: Box<dyn FnOnce(A) -> B>,
}

impl<A: 'static, B: 'static> Kleisli<A, B> {
  pub fn new<F>(f: F) -> Kleisli<A, B>
  where
    F: FnOnce(A) -> B + 'static,
  {
    Kleisli { run: Box::new(f) }
  }

  pub fn run(self, a: A) -> B {
    (self.run)(a)
  }

  pub fn map<C: 'static, F>(self, f: F) -> Kleisli<A, C>
  where
    F: FnOnce(B) -> C + 'static,
  {
    Kleisli::new(|a: A| f(self.run(a)))
  }

  pub fn then<C: 'static>(self, k: Kleisli<B, C>) -> Kleisli<A, C> {
    Kleisli::new(|a: A| k.run(self.run(a)))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn kleisli_then() {
    let k1 = Kleisli::new(|a: i32| (a + 1).to_string());
    let k2 : Kleisli<String, i32> = Kleisli::new(|a: String| a.parse().unwrap());
    let k = k1.then(k2);
    assert_eq!(k.run(1), 2);
  }

  #[test]
  fn kleisli_to_string() {
    let k: Kleisli<i32, String> = Kleisli::new(|a: i32| a.to_string());
    assert_eq!(k.run(1), "1");
  }

}
