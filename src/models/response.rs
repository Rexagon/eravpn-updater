#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub data: T,
    pub success: bool,
}

impl<T> ResponseBody<T> {
    pub fn new(data: T, success: bool) -> ResponseBody<T> {
        ResponseBody { data, success }
    }
}
