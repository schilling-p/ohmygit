use std::io::Write;
use axum::http::{header, StatusCode};
use axum::response::Response;

use error::AppError;

pub fn format_git_advertisement(service: &str, body: &[u8]) -> Vec<u8> {
    let mut formatted_output: Vec<u8> = Vec::new();

    let service_line = format!("# service={}\n", service);
    let pkt_line_len = service_line.len() + 4;
    write!(&mut formatted_output, "{:04x}", pkt_line_len).unwrap();
    formatted_output.extend_from_slice(service_line.as_bytes());
    formatted_output.extend_from_slice(b"0000");
    formatted_output.extend_from_slice(body);

    formatted_output
}

pub fn build_git_advertisement_response(service: &str, formatted_output: Vec<u8>) -> Result<Response, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, format!("application/x-{}-advertisement", service))
        .header(header::CACHE_CONTROL, "no-cache")
        .body(formatted_output.into())
        .unwrap())
}

pub fn build_git_pack_response(service: &str, output: Vec<u8>) -> Result<Response, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, format!("application/x-{}-result", service))
        .header(header::CACHE_CONTROL, "no-cache")
        .body(output.into())
        .unwrap()
    )
}