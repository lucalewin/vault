use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

pub async fn send_verification_mail(email: String, code: String) {
    // let url = std::env::var("NO_REPLY_EMAIL_PASSWORD").expect("NO_REPLY_EMAIL_PASSWORD must be set");

    // let message = MessageBuilder::new()
    //     .from(("No Reply", "noreply@lucalewin.dev"))
    //     .to(("User", email.as_ref()))
    //     .subject("Verify your Email Address!")
    //     .text_body(code);

    // SmtpClientBuilder::new("mail.lucalewin.dev", 587)
    //     .implicit_tls(false)
    //     .credentials(("noreply", "TODO"))
    //     .connect()
    //     .await
    //     .unwrap()
    //     .send(message)
    //     .await.unwrap();
}
