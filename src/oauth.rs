use std::collections::HashMap;
use std::io;
use scraper;

#[derive(Debug)]
struct Form<'a> {
    action: &'a str,
    inputs: HashMap<&'a str, &'a str>
}

fn parse_form<'a>(document: &'a scraper::Html)  -> Form {
    let form_selector = scraper::Selector::parse(r#"form"#).unwrap();
    let form_element = document.select(&form_selector).nth(0).unwrap();
    let action: &'a str = form_element.value().attr("action").unwrap();
    // println!("Form action: {:#?}", action);
    
    let input_selector = scraper::Selector::parse(r#"input"#).unwrap();
    let inputs: HashMap<&'a str, &'a str> = form_element.select(&input_selector)
        .map(|input| {
            let name = match input.value().attr("name") {
                Some(name) => name,
                None => ""
            };
            let value = match input.value().attr("value") {
                Some(val) => val,
                _ => ""
            };
            (name, value)
        })
        .collect();

    Form {
        action,
        inputs
    }
}

fn parse_aoc_callback(response: String) -> reqwest::Url {
    let document = scraper::Html::parse_document(&response);
    let link_selector = scraper::Selector::parse(r#"a#js-manual-authorize-redirect"#).unwrap();
    let link_element = document.select(&link_selector).nth(0).unwrap();

    reqwest::Url::parse(link_element.value().attr("href").unwrap()).unwrap()
}

pub fn authenticate(client: &reqwest::blocking::Client) -> () {
    let page1_res = client
        .get("https://adventofcode.com/auth/github")
        .send()
        .unwrap();

    let document = scraper::Html::parse_document(&page1_res.text().unwrap());
    let mut form = parse_form(&document);

    println!("Github username: ");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input");
    
    println!("Github password: ");
    let password = rpassword::read_password().unwrap();

    if let Some(x) = form.inputs.get_mut("login") { *x = username.trim_end(); }
    if let Some(x) = form.inputs.get_mut("password") { *x = password.as_str(); }
    
    // println!("form: {:?}", form);
    let password_auth_url = reqwest::Url::parse(format!("https://{}{}", "github.com", form.action).as_str()).unwrap();
    let password_auth_res = client
        .post(password_auth_url)
        .form(&form.inputs)
        .send()
        .unwrap();

    let document = scraper::Html::parse_document(&password_auth_res.text().unwrap());
    let mut form = parse_form(&document);
        
    println!("Enter Github OTP");
    let mut github_otp = String::new();

    io::stdin()
        .read_line(&mut github_otp)
        .expect("Failed to read input");

    //fill one time password input
    if let Some(x) = form.inputs.get_mut("otp") { *x = &github_otp; }
    
    let two_factor_auth_url = reqwest::Url::parse(format!("https://{}{}", "github.com", form.action).as_str()).unwrap();
    let two_factor_auth_res = client
        .post(two_factor_auth_url)
        .form(&form.inputs)
        .send()
        .unwrap();

    let callback_url = parse_aoc_callback(String::from(&two_factor_auth_res.text().unwrap()));
    
    client
        .get(callback_url)
        .send()
        .unwrap();

}
