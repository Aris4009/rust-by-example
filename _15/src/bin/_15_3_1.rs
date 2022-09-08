fn main() {
    let inmutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutbook = inmutabook;

    borrow_book(&inmutabook);

    borrow_book(&mutbook);

    new_edition(&mut mutbook);

    // new_edition(&mut inmutabook);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}
