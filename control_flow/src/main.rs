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

fn while_loop() {

  let mut x = 1;
  while x < 1000 {
    x *= 2;
    if x == 64 { continue; }
    println!("x = {}", x);
  }

  let mut y = 0;
  loop {
    y += 1;
    println!("loop {}!", y);
    if y == 5<<1 { break; }
  }
}

fn main () {
  // println!("If statements!");
  // if_statement();

  println!("While and loop!");
  while_loop();
}