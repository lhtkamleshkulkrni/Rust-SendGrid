use sendgrid::SGClient;
use sendgrid::{Destination, Mail};

fn main() {
    // let mut env_vars = std::env::vars();
    // let api_key_check = env_vars.find(|var| var.0 == "SENDGRID_API_KEY");
    // let api_key: String;
    // match api_key_check {
    //     Some(key) => api_key = key.1,
    //     None => panic!("Must supply API key in environment variables to use!"),
    // }


    let api_key = ("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").to_string();

   let sg = SGClient::new(api_key);

    let mut x_smtpapi = String::new();
    x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);

    let mail_info = Mail::new()
        .add_to(Destination {
            address: "rahulsi@leewayhertz.com",
            name: "you there",
        })
        .add_from("kamlesh@leewayhertz.com")
        .add_subject("Rust is rad")
        .add_html("<h1>Hello from Kamlesh!</h1>")
        .add_from_name("Test")
        .add_header("x-cool".to_string(), "indeed")

        .add_x_smtpapi(&x_smtpapi);

    match sg.send(mail_info) {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {:?}", body),
    };
}