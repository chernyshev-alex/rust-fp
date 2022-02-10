
fn compose<A, B, C>(f: impl Fn(&A) -> B, g: impl Fn(&B) -> C) -> impl Fn(&A) -> C {
  move |a: &A| g(&f(a))
}

fn then<A, B, C>(f: impl Fn(&A) -> B, g: impl Fn(&B) -> C) -> impl Fn(&A) -> C {
  compose(f, g)
}

fn combine<A, B, C>(f: impl Fn(&A) -> B, g: impl Fn(&A) -> C) -> impl Fn(&A) -> (B, C) {
  move |v: &A| (f(v), g(v))
}

#[cfg(test)]
mod test {
  use super::*;

  // @see https://typelevel.org/cats/typeclasses/arrow.html

  #[test]
  fn arrow_mean_and_variance() {

    fn mean_and_variance(v: &Vec<i32>) -> (f32, f32) {  
      let fsum = |v: &Vec<i32>| v.iter().sum::<i32>();
      let flen = |v: &Vec<i32>| v.iter().len() as i32;
    
      let mean = then(combine(fsum, flen), |&(s, l)| s as f32 / l as f32);
    
      let pow2 = |v: &Vec<i32>| v.iter().map(|x| x * x).collect::<Vec<i32>>();
      let variance = then(combine(then(pow2, &mean), &mean), |(x, y)| x - y * y);
    
      let mav = combine(&mean, variance);
      mav(v)
    }

    let xs = vec![1, 2, 3, 4];
    let result = mean_and_variance(&xs);
    assert_eq!(result, (2.5, 1.25));
  }

  #[test]
  fn mean_and_variance_simple() {
    let mean = |v: &Vec<i32>| v.iter().sum::<i32>() as f32 / v.len() as f32;
    let variance = |v: &Vec<i32>| mean(&v.iter().map(|x| x * x).collect()) - mean(v).powf(2.0);

    let xs = vec![1, 2, 3, 4];
    let result = (mean(&xs), variance(&xs));
    assert_eq!(result, (2.5, 1.25));
  }
}
