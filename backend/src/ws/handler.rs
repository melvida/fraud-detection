use crate::AppState;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_ws::Message;
use futures::StreamExt;

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> HttpResponse {
    let (res, mut session, mut msg_stream) = match actix_ws::handle(&req, stream) {
        Ok(res) => res,
        Err(e) => {
            log::error!("WebSocket error: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    let mut rx = data.broadcast_tx.subscribe();
    log::info!("Client connected to fraud alerts stream");

    actix_rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Close(_) => {
                    log::info!("Client disconnected");
                    break;
                }
                _ => {}
            }
        }
    });

    actix_rt::spawn(async move {
        while let Ok(alert) = rx.recv().await {
            let msg = serde_json::to_string(&alert).unwrap_or_default();
            if let Err(e) = session.text(msg).await {
                log::warn!("Failed to send alert to client: {}", e);
                break;
            }
        }
    });

    res
}
