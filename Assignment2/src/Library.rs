use crate::Book::book;

pub struct library {
   pub listbooks : Vec<book>
 }
 
 impl library {
    pub fn checkOutBook(&mut self,bookname: &str,borrowerName: &str){
       for bookindex in & mut self.listbooks{
          if bookindex.title == bookname && bookindex.is_available {
             bookindex.is_available=false;
             bookindex.borrowerName=String::from(borrowerName);
          }
       }
    }
 }
 impl library {
    pub fn returnBook(&mut self,bookname: &str){
       for book in & mut self.listbooks{
             if book.title == bookname {
                book.is_available=true;
             }
       }
    }
 }
 impl library {
    pub fn listAllBooks(& self){
       for book in & self.listbooks{
          if book.is_available {
             print!("{} is available  \n",book.title);
          }
       }
    }
 }
 impl library {
    pub fn listCheckoutBooks(self){
       for book in self.listbooks{
          if !book.is_available {
             print!("{} is checkout and not available and borrowed by {} \n ",book.title,book.borrowerName);
          }
       }
    }
 }