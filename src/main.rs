#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Genre {
    Fantasy,
    ScienceFiction,
    Detective,
    NonFiction,
}
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
    genre: Genre,
    read: bool,
}

impl Book {
    fn collect_as_description(&self) -> String {
        format!("{} - {} - ({})", self.author, self.title, self.year)
    }

    fn mark_as_read(&mut self) {
        self.read = true
    }

    fn is_old(&self) -> bool {
        self.year < 1950
    }
}

fn find_oldest(books: &[Book]) -> Option<&Book> {
    if books.is_empty() {
        None
    } else {
        let mut tmp = &books[0];
        for item in books {
            if item.year < tmp.year {
                tmp = item;
            }
        }
        Some(tmp)
    }
}

fn find_read_to_genre(books: &[Book], style: Genre) -> Option<&Book> {
    for book in books {
        if book.genre == style && !book.read {
            return Some(book)
        }
    }
    None
}

fn main() {
    let mut data_1 = [
        Book {
            title: String::from("Дуб зелёный"),
            author: String::from("Пушкин"),
            year: 1953,
            genre: Genre::Fantasy,
            read: true,
        },
        Book {
            title: String::from("Преступление и наказание"),
            author: String::from("Достоевский"),
            year: 1866,
            genre: Genre::Detective,
            read: false,
        },
        Book {
            title: String::from("Дюна"),
            author: String::from("Герберт"),
            year: 1965,
            genre: Genre::ScienceFiction,
            read: true,
        },
        Book {
            title: String::from("Сапиенс. Краткая история человечества"),
            author: String::from("Харари"),
            year: 2011,
            genre: Genre::NonFiction,
            read: false,
        },
        Book {
            title: String::from("Властелин колец"),
            author: String::from("Толкин"),
            year: 1954,
            genre: Genre::Fantasy,
            read: true,
        },
    ];

    for book in &data_1 {
        println!("{}", book.collect_as_description())
    }

    data_1[1].mark_as_read();

    match find_oldest(&data_1) {
        None => println!("Самая старая книга не найдена!"),
        Some(res) => println!("{}", res.collect_as_description()),
    }

    if let Some(res) = find_read_to_genre(&data_1, Genre::Detective) {
        println!("{}", res.collect_as_description());
    } else {
        println!("Такой книги не нашлось!")
    };

    match find_oldest(&[]) {
        Some(_) => println!("Данные присутствуют!"),
        None => println!("Срез пуст!"),
    };
}
