use crate::order::Order;

/// Send order confirmation notification.
/// This is fire-and-forget — caller ignores errors intentionally.
pub async fn send_order_confirmation(
    service_url: &str, order: &Order
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let payload = serde_json::json!({
        "type": "order_confirmation",
        "order_id": order.id,
        "customer_id": order.customer_id,
        "total": order.total.to_string(),
    });

    client
        .post(format!("{}/notify", service_url))
        .json(&payload)
        .send()
        .await?;

    Ok(())
}
