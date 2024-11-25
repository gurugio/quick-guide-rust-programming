use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

trait GenSerialData {
    fn get_input(&mut self);
    fn generate(&self) -> Option<&str>;
}

struct UserID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for UserID {
    fn get_input(&mut self) {
        println!("Please input {}-digits User ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}

struct ProductID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for ProductID {
    fn get_input(&mut self) {
        println!("Please input {}-digits Product ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}

//fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) { // If you want to use Vec<Box<dyn GenSerialData>> in main function
fn collect_data(items: &mut [Box<dyn GenSerialData>]) {
    for item in items.iter_mut() {
        item.get_input();
    }
}

// &[&dyn GenSerialData] is wrong!
//fn generate_serial(items: &Vec<Box<dyn GenSerialData>>) -> String { // If you want to use Vec<Box<dyn GenSerialData>> in main function
fn generate_serial(items: &[Box<dyn GenSerialData>]) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(item.generate().unwrap());
    }
    data
}

fn main() {
    println!("hello");

    let userid = UserID { digit: 4, id: None };
    let productid = ProductID { digit: 8, id: None };

    // Vec<&dyn GenSerialData> is wrong!
    //let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(productid)];
    let mut items: [Box<dyn GenSerialData>; 2] = [Box::new(userid), Box::new(productid)];

    collect_data(&mut items);
    let serial = generate_serial(&items);
    println!("Serial generated: {}", serial);
}
