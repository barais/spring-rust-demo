use anyhow::Context;

use spring::plugin::Service;
use spring_mail::{AsyncTransport, Mailer, Message, Response, header::ContentType};

use spring_web::error::Result;

use crate::config::mail::EmailConfig;

#[derive(Clone, Service)]
pub struct MailService {
    #[inject(config)]
    pub email_config: EmailConfig,

    #[inject(component)]
    pub mailer: Mailer,
}

impl MailService {
    pub async fn send_mail(&self, to: String) -> Result<Response> {
        let email = Message::builder()
            .from(self.email_config.from.parse().unwrap())
            .reply_to(self.email_config.from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject("Happy new year")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be happy!"))
            .unwrap();

        let resp = self.mailer.send(email).await.context("send mail failed")?;
        Ok(resp)
    }
}
