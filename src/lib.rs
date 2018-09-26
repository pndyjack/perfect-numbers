#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
  Abundant,
  Perfect,
  Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
  match num {
    0 => return None,
    1 => return Some(Classification::Deficient),
    _ => {}
  }
  if check_if_prime(num) {
    return Some(Classification::Deficient);
  }
  let range: Vec<u64> = (1..num).collect();
  let factors: Vec<u64> = range
    .into_iter()
    .map(|factor| {
      if num % factor == 0 {
        return factor;
      } else {
        0
      }
    })
    .collect();
  let sum: u64 = factors.iter().sum();
  match (num == sum, num < sum, num > sum) {
    (true, _, _) => Some(Classification::Perfect),
    (_, true, _) => Some(Classification::Abundant),
    (_, _, true) => Some(Classification::Deficient),
    _ => None,
  }
}

pub fn check_if_prime(n: u64) -> bool {
  if n <= 1 {
    return false;
  }
  if n <= 3 {
    return true;
  }
  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }
  let mut i = 5;
  while i * i <= n {
    if n % i == 0 || n % (i + 2) == 0 {
      return false;
    }
    i += 6;
  }
  true
}
