use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn send_reset_email(to: &str, token: &str) {
    dotenv().ok();

    let email_username = env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME must be set");
    let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");

    // HTML body
    let html_body = format!(
        r#"
        <html>
        <head>
            <title>Password Reset</title>
        </head>
        <body>
            <p>Click the link below to reset your password:</p>
            <p><a href="http://localhost:3000/reset-password/{}">Reset Password</a></p>
            <p>Copy the link and paste it in your browser.</p>
            <p>http://localhost:3000/reset-password/{}</p>
        </body>
        </html>
        "#,
        token, token
    );

    // Create the email
    let email = Message::builder()
        .from(email_username.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Password Reset")
        .header(ContentType::TEXT_HTML)
        .body(html_body)
        .unwrap();

    // Configure the SMTP client
    let creds = Credentials::new(email_username.clone(), email_password);
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}
