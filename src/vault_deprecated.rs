// "{"auth":null,"data":{"foo":"bar"},"lease_duration":3600,"lease_id":"","renewable":false}"
use serde_json::Value;

pub fn _vault_kv(key: &str) -> String {
    let data = _vault_data("v1/secret/test");
    data.get("data")
        .unwrap()
        .get(key)
        .unwrap()
        .to_string()
        .replace("\"", "")
}
pub fn _vault_data(key: &str) -> Value {
    let client = rusty_vault::api::client::Client::new()
        .with_token("TOKEN")
        .with_addr("http://127.0.0.1:8200");
    client.request_get(key).unwrap().response_data.unwrap()
}
