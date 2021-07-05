use protos::types::alerts::Alert as ProtoAlert;

#[derive(Debug, serde::Serialize)]
pub struct Alert {
    id: String,
    cvss_score: f32,
    provider: String,
    product: String,
    published_at: String,
    updated_at: String,
    description: String,
    starred: Option<bool>,
}

impl From<ProtoAlert> for Alert {
    fn from(mut alert: ProtoAlert) -> Self {
        Self {
            id: alert.take_id(),
            cvss_score: alert.get_cvss_score(),
            provider: alert.take_provider(),
            product: alert.take_product(),
            published_at: alert.take_published_at(),
            updated_at: alert.take_updated_at(),
            description: alert.take_description(),
            starred: if alert.has_starred() {
                Some(alert.get_starred())
            } else {
                None
            },
        }
    }
}

impl<T> From<&T> for Alert
where
    T: Clone + Into<Alert>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}
