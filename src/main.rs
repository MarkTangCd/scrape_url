use std::fs;

// fn fib_num_compute(a: &mut i32, b: &mut i32) {
//   let c = *a + *b;
//   *a = *b;
//   *b = c;
// }

// fn fib_loop(n: u8) {
//   let mut a = 1;
//   let mut b = 1;
//   let mut i = 2u8;
  
//   loop {
//      fib_num_compute(&mut a, &mut b) ;
//       i += 1;
      
//       println!("next val is {}", b);
      
//       if i >= n {
//           break;
//       }
//   }
// }

// fn fib_while(n: u8) {
//   let (mut a, mut b, mut i) = (1, 1, 2);
  
//   while i < n {
//       fib_num_compute(&mut a, &mut b) ;
//       i += 1;
      
//       println!("next val is {}", b);
//   }
// }

// fn fib_for(n: u8) {
//   let (mut a, mut b) = (1, 1);
  
//   for _i in 2..n {
//       fib_num_compute(&mut a, &mut b) ;
//       println!("next val is {}", b);
//   }
// }

// fn main() {
//   let n = 10;
//   fib_loop(n);
//   fib_while(n);
//   fib_for(n);
// }

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 3 {
    println!("Usage: url outpath");
  }

  args.iter().for_each(|arg|{
    println!("{}", arg);
  });

  let url = &args[1];
  let output = &args[2];

  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}

// error

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//   let url = "https://www.rust-lang.org/";
//   let output = "rust.md";

//   println!("Fetching url: {}", url);
//   let body = reqwest::blocking::get(url)?.text()?;

//   println!("Converting html to markdown...");
//   let md = html2md::parse_html(&body);

//   fs::write(output, md.as_bytes())?;
//   println!("Converted markdown has been saved in {}.", output);

//   Ok(())
// }