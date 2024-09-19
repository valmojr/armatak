use log::info;

use crate::structs::Marker;

pub fn get_uuid() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}

pub async fn post_marker(marker: Marker) -> String {
    let placeholder = format!("{} - {}", marker.uid, marker.name);

    info!("{}", placeholder);

    return placeholder
}
