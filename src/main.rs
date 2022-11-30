pub mod oauth;


// https://adventofcode.com/auth/github
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup cookie store
    let cookie_store = {
        let file = std::fs::File::open("cookies.json")
            .map(std::io::BufReader::new)
            .unwrap();
        // use re-exported version of `CookieStore` for crate compatibility
        reqwest_cookie_store::CookieStore::load_json(file).unwrap()
    };
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()
        .unwrap();

    // Check if user is authenticated

    // Check if session cookie is present in the cookie store
    let mut user_authenticated: bool = false;
    let mut session_cookie_present: bool = false;
    let aoc_url = reqwest::Url::parse("https://adventofcode.com").unwrap();
    {
        let store = cookie_store.lock().unwrap();
        for c in store.iter_any() {
            if c.domain.matches(&aoc_url) && c.name() == "session" {
                println!("AOC Session cookie is PRESENT");
                session_cookie_present = true;
                break;
            }
        }
    }
    
    if session_cookie_present {
        // Check if session cookie is still valid
        let home_page_res = client
            .get(aoc_url)
            .send()
            .unwrap();
        let document = scraper::Html::parse_document(&home_page_res.text().unwrap());
        let div_selector = scraper::Selector::parse(r#"body>header>div>div.user"#).unwrap();
        match document.select(&div_selector).nth(0) {
            Some(_) => {
                user_authenticated = true;
                println!("AOC Session cookie is VALID");
            },
            None => {
                eprintln!("AOC Session cookie is INVALID");
            }
        };
    }
    else
    {
        println!("AOC Session cookie NOT FOUND"); 
    }
    
    if !user_authenticated {
        oauth::authenticate(&client);
        // Write store back to disk
        let mut writer = std::fs::File::create("cookies.json")
            .map(std::io::BufWriter::new)
            .unwrap();
            
        {
            let store = cookie_store.lock().unwrap();
            store.save_json(&mut writer).unwrap();
        }
    }
    
        
    //get exercise
    // let y21_day15_res = client
    //     .get("https://adventofcode.com/2021/day/15")
    //     .send()
    //     .unwrap();
    // println!("y21_day15_res ========================================\n {:?}\n", &y21_day15_res.text()?);


    Ok(())
    
}