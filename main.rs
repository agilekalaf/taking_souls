fn main() {
    taking_souls(3,0);
}
fn taking_souls(miles: i32, souls: i32){
  if miles >= 8 {
      println!("Miles run: {}", miles);
      println!("Souls taken: {}", souls);
  } else {
    println!("Stay hard, keep running!");
    taking_souls(miles + 1,souls + 1);
  }
}