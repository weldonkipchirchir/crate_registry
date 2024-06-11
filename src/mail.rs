use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, response::Response},
    SmtpTransport, Transport,
};
use tera::Context;
pub struct HtmlMailer {
    pub smtp_host: String,
    pub credentials: Credentials,
    pub template_engine: tera::Tera,
}

impl HtmlMailer {
    pub fn send_email(
        self,
        to: &String,
        template_name: &str,
        context: &Context,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let html_body = self.template_engine.render(template_name, &context)?;

        let message = lettre::Message::builder()
            .subject("crate registry digest")
            .from("crate_registry <info@crate_registry.com>".parse()?)
            .to(to.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(self.credentials)
            .build();

        mailer.send(&message).map_err(|err| err.into())
    }
}
