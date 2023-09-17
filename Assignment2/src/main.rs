use std::borrow::BorrowMut;
use crate::Person::person;
use crate::Book::book;
use crate::Library::library;

mod Person;
mod Book;
mod Library;

fn main() {
   let shubham = person{
      name: String::from("Shubham"),
      age: 30
   };
   shubham.name_age_of_person();
   

   let harryPotter = book{
      title : String::from("Harry Potter"),
      author : String::from("JK Rowling"),
      is_available: true,
      borrowerName : String::from("")

   };

   let thinkAbtIt = book{
      title : String::from("thinkAbtIt"),
      author : String::from("Anonymous"),
      is_available: true,
      borrowerName : String::from("")
   };
   let book3 = book{
      title : String::from("book3"),
      author : String::from("noBook"),
      is_available: true ,
      borrowerName : String::from("") 
   };
   let  bookvec :Vec<book> = Vec::new();
   

   let mut  bookLibrary = library{
      listbooks : bookvec
   };
   bookLibrary.listbooks.push(harryPotter);
   bookLibrary.listbooks.push(thinkAbtIt);
   bookLibrary.listbooks.push(book3);

   print!("\n Listing all books \n");
   bookLibrary.listAllBooks();

  
   print!("Checking out a book \n");
   bookLibrary.checkOutBook("thinkAbtIt","Shubham");

   print!("Printng all checkout books \n");
   bookLibrary.listCheckoutBooks()

}
