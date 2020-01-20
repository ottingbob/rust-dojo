
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
  // `&'static str` is a reference to a string allocated in ro memory
  author: &'static str,
  title:  &'static str,
  year:            u32,
}

fn borrow_book(book: &Book) {
  println!("I immutably borrowed {} by {} - {} edition", book.title, book.author, book.year);
}

fn new_edition(book: &mut Book){
  book.year = 2024;
  println!("I mutably borroed {} by {} - {} edition", book.title, book.author, book.year);
}

fn main() {
  let immutable_book = Book {
    author: "Doglas Winslow Dogworth",
    title: "eins zwei gesuffe",
    year: 1921,
  };

  let mut mutabook = immutable_book;
  borrow_book(&immutable_book);
  borrow_book(&mutabook);
  new_edition(&mut mutabook);
  // new_edition(&mut immutable_book);
}