/* // Define a trait for greeting behavior
trait Greeter {
    fn greet(&self) -> String;
    fn language_code(&self) -> &'static str;
}

// English implementation
struct EnglishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self) -> String {
        "Hello and welcome to my tech blog!".to_string()
    }

    fn language_code(&self) -> &'static str {
        "en"
    }
}

// Finnish implementation/*  */
struct FinnishGreeter;

impl Greeter for FinnishGreeter {
    fn greet(&self) -> String {
        "Hei ja tervetuloa tekniikka-blogiini!".to_string()
    }

    fn language_code(&self) -> &'static str {
        "fi"
    }
}

// French implementation
struct FrenchGreeter;

impl Greeter for FrenchGreeter {
    fn greet(&self) -> String {
        "Bonjour et bienvenue sur mon blog technique!".to_string()
    }

    fn language_code(&self) -> &'static str {
        "fr"
    }
}

// Italian implementation
struct ItalianGreeter;

impl Greeter for ItalianGreeter {
    fn greet(&self) -> String {
        "Ciao e benvenuti nel mio blog tecnico!".to_string()
    }

    fn language_code(&self) -> &'static str {
        "it"
    }
}

// German implementation
struct GermanGreeter;

impl Greeter for GermanGreeter {
    fn greet(&self) -> String {
        "Hallo und willkommen auf meinem Tech-Blog!".to_string()
    }

    fn language_code(&self) -> &'static str {
        "de"
    }
}

// Blog greeting service
struct BlogGreetingService {
    greeters: Vec<Box<dyn Greeter>>,
}

impl BlogGreetingService {
    fn new() -> Self {
        Self {
            greeters: vec![
                Box::new(EnglishGreeter),
                Box::new(FinnishGreeter),
                Box::new(FrenchGreeter),
                Box::new(ItalianGreeter),
                Box::new(GermanGreeter),
            ],
        }
    }

    fn greet_all(&self) {
        println!("🌍 Welcome to My Tech Blog - Multilingual Greetings:\n");

        for greeter in &self.greeters {
            println!(
                "[{}] {}",
                greeter.language_code().to_uppercase(),
                greeter.greet()
            );
        }

        println!("\n🚀 Ready to explore the world of modern software development!");
    }
}

fn main() {
    let blog_service = BlogGreetingService::new();
    blog_service.greet_all();
}
 */
