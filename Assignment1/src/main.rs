struct Book{
  title: String,
  author: String,
  is_available: bool
}
struct Person {
   name: String,
   age :u32
}
struct Library {
   listbooks : Vec<Book>
}
fn main() {
   let shubham = Person{
      name: String::from("Shubham"),
      age: 30
   };
   let harryPotter = Book{
      title : String::from("Harry Potter"),
      author : String::from("JK Rowling"),
      is_available: true      
   };
   let  bookvec :Vec<Book> = Vec::new();

   let bookLibrary = Library{
      listbooks : bookvec
   };



   
}
