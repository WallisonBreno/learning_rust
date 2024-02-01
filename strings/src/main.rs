use unicode_segmentation::UnicodeSegmentation;

fn main() {
   
   let s1 = String::new();
   let s2 = "initial contents";
   let s3 = s2.to_string();
   let s4 = String::from("initial contents");

   let mut s = String::from("foo");
   s.push_str("bar");
   s.push('!');

   let s1 = String::from("Hello, ");
   let s2 = String::from("World!");
   //Não pega a ownership de s1
   let s3 = format!("{}{}",s1,s2);
   //Pega a ownership de s1
   let s3: String = s1 + &s2;

   //Itera sob os bytes de cada caractere
   for b in "Зд".bytes() {
        println!("{b}");
   }
   //Itera sob os scalar values de cada caractere
   for b in "Зд".chars() {
        println!("{b}");
   }
    //Itera sob cada caractere
    for g in "Зд".graphemes(true) {
        println!("{}", g);
    }
    
}
