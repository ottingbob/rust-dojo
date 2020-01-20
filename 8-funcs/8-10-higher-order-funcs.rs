
fn is_odd(n: u32) -> bool {
  n % 2 == 1
}

fn main() {
  println!("Find the sum of all the squared odd numbers under 1000");
  let upper = 1000;

  // imperative style
  let mut acc = 0;
  for n in 0.. {
    let n_squared = n * n;

    if n_squared >= upper {
      // Break loop if exceeded the upper limit
      break;
    } else if is_odd(n_squared) {
      // Accumulate value, it it's odd
      acc += n_squared;
    }
  }

  println!("imperative style: {}", acc);

  // functional style
  let sum_of_squared_odd_nbrs: u32 = 
    (0..).map(|n| n * n)
         .take_while(|&n_squared| n_squared < upper)
         .filter(|&n_squared| is_odd(n_squared))
         .fold(0, |acc, n_squared| acc + n_squared);
  println!("functional style: {}", sum_of_squared_odd_nbrs);
}
