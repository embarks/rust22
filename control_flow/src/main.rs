fn if_statement() {
  let temp = 20;
  if temp > 30 {
      println!("It's warm outside!");
  } else if temp < 10 {
      println!("It's cold outside!");
  } else {
      println!("It's a lovely day!");
  }
}

fn main () {
println!("If statements!");
if_statement();
}