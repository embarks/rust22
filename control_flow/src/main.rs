fn if_statement() {
  let temp = 35;
  println!("It's {} degrees outside", temp);

  if temp > 30 {
      println!("It's warm outside!");
  } else if temp < 10 {
      println!("It's cold outside!");
  } else {
      println!("It's a lovely day!");
  }

  let day = if temp > 20 { "sunny" } else { "cloudy" };
  println!("Today is {}", day);

  println!("it is {}", if temp > 20 { "hot" } else if temp < 10 { "cold" } else {"OK"});
  println!("it is very {}", 
    if temp > 20 {
      if temp > 30 {"very hot"} else {"hot"}
    } else if temp < 10 {
      "cold"
    } else {
      "OK"
    });
}

fn main () {
println!("If statements!");
if_statement();
}