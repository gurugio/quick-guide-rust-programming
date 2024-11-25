#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title.clone(),
            author: self.author.clone(),
            published: self.published,
        }
    }
}

//fn print_info(item: &dyn Clone) {
//    println!("item implements Clone trait");
//}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
    book_clone.clone_from(&another);
    println!("{:?}", book_clone);

    //print_info(&book);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Painting {
        title: String,
        author: String,
        published: i32,
    }

    /* Solution for Exercise1
    impl Clone for Painting {
        fn clone(&self) -> Self {
            Painting {
                title: self.title.clone(),
                author: self.author.clone(),
                published: self.published,
            }
        }
    }*/

    fn show_painting_to_seoul(mypainting: Painting) {
        let newname = mypainting.title + " in Seoul";
        println!("Dear Seoul, See my new painting! {}", newname);
    }

    fn show_painting_to_newyork(mypainting: Painting) {
        let newname = mypainting.title + " in New York";
        println!("Dear New York, See my new painting! {}", newname);
    }

    fn show_painting_to_london(mypainting: Painting) {
        let newname = mypainting.title + " in London";
        println!("Dear London, See my new painting! {}", newname);
    }

    #[test]
    fn test_trait_clone() {
        let mypainting = Painting {
            title: "My Starry Night".to_owned(),
            author: "Vencent Gauguin".to_owned(),
            published: 2024,
        };

        /* Fix build errors */
        show_painting_to_seoul(mypainting);
        // Exercise1 // show_painting_to_newyork(mypainting);
        // Exercise1 // show_painting_to_london(mypainting);

        /* Solution for Exercise1
        show_painting_to_seoul(mypainting.clone());
        show_painting_to_newyork(mypainting.clone());
        show_painting_to_london(mypainting.clone());
         */
    }
}
