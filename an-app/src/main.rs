// THE FOLLOWING CODE DOES NOT COMPILE
// (and is overall quite different from valid Rust code)

use std::{thread::sleep, time::Duration};

// fn main() {
//     let mut app = AppV1 {
//         title: "Jack in the box".to_string(),
//         ticks_left: 4,
//         running: true,
//     };

//     println!("=== You are now playing {} ===", app.title);

//     loop {
//         app.update();
//         app.render();

//         if !app.running {
//             break;
//         }
//         sleep(Duration::from_secs(1));
//     }
// }
enum UpdateResult {
    None,
    QuitApplication,
}
trait Client {
    fn update(&mut self) -> UpdateResult;
    fn render(&self);
}

impl Client for MyClient {
    fn update(&mut self) -> UpdateResult {
        self.ticks_left -= 1;
        if self.ticks_left == 0 {
            return UpdateResult::QuitApplication;
        }
        UpdateResult::None
    }

    fn render(&self) {
        if self.ticks_left > 0 {
            println!("You turn the crank...");
        } else {
            println!("Jack POPS OUT OF THE BOX");
        }
    }
}

struct App {
    client: Box<dyn Client>,
    state: AppState,
}

struct AppState {
    title: String,
}
struct MyClient {
    ticks_left: usize,
}
impl App {
    fn run(&mut self) {
        println!("=== You are now playing {} ===", self.state.title);
        loop {
            let status = self.client.update();
            self.client.render();

            match status {
                UpdateResult::QuitApplication => return,
                UpdateResult::None => {}
            }
            sleep(Duration::from_secs(1));
        }
    }
}

fn main() {
    let client = MyClient { ticks_left: 4 };
    let state = AppState {
        title: "Jack in the box".to_string(),
    };
    let mut app = App {
        client: Box::new(client),
        state,
    };

    app.run();
}

// struct AppV1 {
//     title: String,
//     ticks_left: usize,
//     running: bool,
// }
// impl AppV1 {
//     fn update(&mut self) {
//         self.ticks_left -= 1;
//         if self.ticks_left == 0 {
//             self.running = false;
//         }
//     }

//     fn render(&self) {
//         if self.ticks_left > 0 {
//             println!("You turn the crank...");
//         } else {
//             println!("Jack POPS OUT OF THE BOX");
//         }
//     }
// }
