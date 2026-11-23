use crate::{arguments::PaymentProvider, client::ProductClient, task_access};
use anyhow::{ensure, Result};
use serde_json::{json, Value};
use uuid::Uuid;

pub fn checkout(
    client: &ProductClient,
    service: &str,
    id: Uuid,
    provider: PaymentProvider,
) -> Result<Value> {
    let access = task_access::read(id, service)?;
    let path = match provider {
        PaymentProvider::Creem => "/api/v2/task/checkout/open",
        PaymentProvider::Paypal => "/api/v2/task/paypal/prepare",
    };
    client.post(path, json!({"taskId":id}), Some(&access.key))
}

pub fn read(client: &ProductClient, service: &str, id: Uuid) -> Result<Value> {
    let access = task_access::read(id, service)?;
    client.post(
        "/api/v2/task/payment/read",
        json!({"taskId":id}),
        Some(&access.key),
    )
}

pub fn verify(
    client: &ProductClient,
    service: &str,
    id: Uuid,
    checkout_id: Option<String>,
    provider: PaymentProvider,
) -> Result<Value> {
    ensure!(
        !matches!(provider, PaymentProvider::Paypal) || checkout_id.is_none(),
        "--checkout-id belongs only to Creem; PayPal uses the saved order"
    );
    let access = task_access::read(id, service)?;
    let path = match provider {
        PaymentProvider::Creem => "/api/v2/task/payment/verify",
        PaymentProvider::Paypal => "/api/v2/task/paypal/confirm",
    };
    let mut body = json!({"taskId":id});
    if let Some(checkout_id) = checkout_id {
        body["checkoutId"] = json!(checkout_id);
    }
    client.post(path, body, Some(&access.key))
}
