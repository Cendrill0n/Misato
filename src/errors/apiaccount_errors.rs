use misato::models::apiaccount_model::ApiAccountError;

pub struct Error {
    pub content: ApiAccountError,
}

impl<'r> rocket::response::Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self.content).unwrap();
        rocket::Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::new(self.content.code))
            .ok()
    }
}
