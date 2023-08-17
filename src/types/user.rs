use super::*;
use crate::{
    dto::{EmailDTO, PhoneDTO, SingDto, SlotDTO, TaskDTO, UserDTO},
    errors::{RegisterResult, SingUpResult},
    DOMEN, PASSWORD_HEADER, USERNAME_HEADER,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, StatusCode,
};

impl User {
    fn build_headers(&self) -> HeaderMap {
        User::build_headers_from_dto(SingDto {
            username: self.username(),
            password: self.password(),
        })
    }

    fn build_headers_from_dto(dto: SingDto) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            USERNAME_HEADER,
            HeaderValue::from_str(dto.username.as_str()).unwrap(),
        );
        headers.insert(
            PASSWORD_HEADER,
            HeaderValue::from_str(dto.password.as_str()).unwrap(),
        );
        headers
    }

    pub async fn check_username(username: String) -> bool {
        let client = Client::new();
        let request = client
            .get(DOMEN.to_string() + "/users/check/" + username.as_str())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        responce.status() == StatusCode::OK
    }

    pub async fn register(user: UserDTO) -> RegisterResult {
        let client = Client::new();
        let request = client
            .post(DOMEN.to_string() + "/users/register")
            .json(&user)
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        if responce.status() == StatusCode::OK {
            return Ok(responce.json().await.unwrap());
        }
        Err(responce.json().await.unwrap())
    }

    async fn refresh(&mut self) {
        let new_data = User::get(SingDto {
            username: self.username(),
            password: self.password(),
        })
        .await
        .unwrap();
        *self = new_data;
    }

    pub async fn get(sing_data: SingDto) -> SingUpResult {
        let client = Client::new();
        let responce = client
            .get(format!("{}/users/user", DOMEN))
            .headers(User::build_headers_from_dto(sing_data))
            .send()
            .await
            .unwrap();
        match responce.status() {
            StatusCode::OK => Ok(responce.json().await.unwrap()),
            _ => Err(responce.json().await.unwrap()),
        }
    }

    pub async fn add_slot(&mut self, dto: SlotDTO) -> Result<(), String> {
        let url = format!("{}/users/slots/activate", DOMEN.to_string());
        let client = Client::new();
        let request = client
            .post(url)
            .json(&dto)
            .headers(self.build_headers())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        match responce.status() {
            StatusCode::OK => {
                self.refresh().await;
                Ok(())
            }
            _ => Err(responce.text().await.unwrap()),
        }
    }

    pub async fn remove_slot(&mut self, subject: Subjects) -> Result<(), String> {
        let url = format!("{}/users/slots/deactivate/{:?}", DOMEN.to_string(), subject);
        let client = Client::new();
        let request = client
            .delete(url)
            .headers(self.build_headers())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        self.refresh().await;
        match responce.status() {
            StatusCode::OK => {
                self.refresh().await;
                Ok(())
            }
            _ => Err(responce.text().await.unwrap()),
        }
    }

    pub fn location_info(&self) -> LocationData {
        self.location_data.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }

    pub fn first_name(&self) -> &str {
        self.frirst_name.as_ref()
    }

    pub fn last_name(&self) -> &str {
        self.frirst_name.as_ref()
    }

    pub fn class(&self) -> i16 {
        self.class
    }

    pub fn school(&self) -> i32 {
        self.school
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn slots_component(&mut self) -> &mut SlotsComponent {
        &mut self.slots_component
    }

    pub fn tasks_component(&self) -> &TasksComponent {
        &self.tasks_component
    }

    //Сеттеры
    pub async fn change_email(&mut self, email: String) -> Result<(), String> {
        let url = format!("{}/users/change/email/{}", DOMEN.to_string(), email);
        let client = Client::new();
        let request = client
            .post(url)
            .json(&EmailDTO { email })
            .headers(self.build_headers())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        match responce.status() {
            StatusCode::OK => {
                self.refresh().await;
                Ok(())
            }
            _ => Err(responce.text().await.unwrap()),
        }
    }

    pub async fn change_phone_number(&mut self, phone: Option<String>) -> Result<(), String> {
        let url = format!(
            "{}/users/change/phone/{}",
            DOMEN.to_string(),
            match phone.clone() {
                Some(phone) => phone,
                None => "".into(),
            }
        );
        let client = Client::new();
        let request = client
            .post(url)
            .json(&PhoneDTO { phone })
            .headers(self.build_headers())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        match responce.status() {
            StatusCode::OK => {
                self.refresh().await;
                Ok(())
            }
            _ => Err(responce.text().await.unwrap()),
        }
    }

    //Работа с тасками
    pub async fn publish_task(&mut self, dto: TaskDTO) -> Result<Task, String> {
        let url = format!("{}/users/publish_task", DOMEN.to_string(),);
        let client = Client::new();
        let request = client
            .post(url)
            .headers(self.build_headers())
            .json(&dto)
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        match responce.status() {
            StatusCode::OK => {
                self.refresh().await;
                responce.json().await.unwrap()
            }
            _ => Err(responce.text().await.unwrap()),
        }
    }

    pub async fn accept_task(&mut self, task: Task) -> Result<Transaction, String> {
        let url = format!("{}/users/accept_task/{}", DOMEN.to_string(), task.uuid());
        let client = Client::new();
        let request = client
            .post(url)
            .headers(self.build_headers())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        match responce.status() {
            StatusCode::OK => {
                let result = responce.json().await.unwrap();
                self.refresh().await;
                result
            }
            _ => Err(responce.text().await.unwrap()),
        }
    }

    //рейтинг
    pub async fn rate(username: String, mark: u8) -> Result<(), String> {
        if mark > 5 {
            panic!("Wrong mark")
        }
        let url = format!(
            "{}/users/rate/{}/user/{}",
            DOMEN.to_string(),
            mark,
            username
        );
        let client = Client::new();
        let request = client.post(url).build().unwrap();
        let responce = client.execute(request).await.unwrap();
        match responce.status() {
            StatusCode::OK => Ok(()),
            _ => Err(responce.text().await.unwrap()),
        }
    }

    pub fn raiting(&self) -> f64 {
        if self.raiting.len() == 0 {
            return 0.0;
        }
        let mut sum: f64 = 0.0;
        for mark in self.raiting.clone() {
            sum += mark as f64;
        }
        sum / self.raiting.len() as f64
    }
}
