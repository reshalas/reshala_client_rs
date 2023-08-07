use crate::DOMEN;
use crate::dto::SlotLimitDTO;
use super::Slot;
use super::Subjects;
use super::User;

impl Slot {
    pub fn uuid(&self) -> String {
        self.id.clone()
    }

    pub fn limit(&self) -> Option<i32> {
        self.limit
    }

    pub async fn update_limit(&mut self, limit: Option<i32>) {
        let client = reqwest::Client::new();
        let request = client
            .post(DOMEN.to_string() + "/slots/" + self.uuid().as_str() + "/update_limit")
            .json(&SlotLimitDTO{limit})
            .build()
            .unwrap();
        client.execute(request).await.unwrap();
    }

    pub fn subject(&self) -> Subjects {
        self.subject.clone()
    }

    pub fn owner_username(&self) -> String {
        self.owner_username.clone()
    }

    pub async fn owner(&self) -> User {
        let client = reqwest::Client::new();
        let request = client
            .get(DOMEN.to_string() + "/slots/" + self.uuid().as_str() + "/owner")
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        responce.json::<User>().await.unwrap()
    }
}
