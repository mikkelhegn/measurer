use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

#[derive(Deserialize, Serialize)]
struct Data {
    device_id: String,
    epoch_time: i64,
    humidity: f64,
    temperature: f64,
}

#[derive(Deserialize, Serialize, Default)]
struct DeviceMeasure {
    device: String,
    measure: String,
    data: Vec<MeasureData>,
}

#[derive(Deserialize, Serialize, Default)]
struct MeasureData {
    time: i64,
    value: f64,
}

#[http_component]
fn handle_temp(req: Request) -> anyhow::Result<impl IntoResponse> {
    let connection = Connection::open_default().expect("Cannot open SQLite connection");

    // If we get data for the visualizer
    if req.query() == "visualizer" {
        let mut device_measures: Vec<DeviceMeasure> = vec![];

        let devices_result = connection
            .execute("SELECT DISTINCT device_id FROM data", &[])
            .expect("Failed to get devices");

        let devices: Vec<_> = devices_result
            .rows()
            .map(|row| {
                row.get::<&str>("device_id")
                    .expect("Failed to parse device_id")
            })
            .collect();

        for device in devices {
            let humidity_result = connection
                .execute(
                    "SELECT epoch_time, humidity FROM data WHERE device_id = ? AND epoch_time > ?",
                    &[
                        Value::Text(device.to_string()),
                        Value::Integer((Utc::now() - Duration::days(30)).timestamp()),
                    ],
                )
                .expect("Failed to get humidity_data");

            let humidity_data = humidity_result
                .rows()
                .map(|row| MeasureData {
                    time: row
                        .get::<i64>("epoch_time")
                        .expect("Failed to parse epoch_time"),
                    value: row
                        .get::<f64>("humidity")
                        .expect("Failed to parse humidity"),
                })
                .collect();

            device_measures.push(DeviceMeasure {
                device: device.to_string(),
                measure: "humidity".to_string(),
                data: humidity_data,
            });

            let temperature_result = connection
                .execute(
                    "SELECT epoch_time, temperature FROM data WHERE device_id = ? AND epoch_time > ?",
                    &[Value::Text(device.to_string()), Value::Integer((Utc::now() - Duration::days(30)).timestamp())],
                )
                .expect("Failed to get temperature_data");

            let temperature_data = temperature_result
                .rows()
                .map(|row| MeasureData {
                    time: row
                        .get::<i64>("epoch_time")
                        .expect("Failed to parse epoch_time"),
                    value: row
                        .get::<f64>("temperature")
                        .expect("Failed to parse temperature"),
                })
                .collect();

            device_measures.push(DeviceMeasure {
                device: device.to_string(),
                measure: "temperature".to_string(),
                data: temperature_data,
            });
        }

        Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&device_measures).expect("Failed to deserialize data"))
            .build())
    } else {
        // If it's just sending data...
        let data: Data = serde_json::from_slice(req.body()).expect("Failed to serialize data");
        let utc: DateTime<Utc> =
            DateTime::from_timestamp(data.epoch_time, 0).expect("Failed to convert time");
        println!("{:?}: Storing data from {:?}", utc, data.device_id);

        let params = [
            Value::Text(data.device_id),
            Value::Integer(data.epoch_time),
            Value::Real(data.humidity),
            Value::Real(data.temperature),
        ];

        connection
        .execute(
            "INSERT INTO data (device_id, epoch_time, humidity, temperature) VALUES (?, ?, ?, ?)",
            params.as_slice(),
        )
        .expect("Failed to execute SQLite statement");

        Ok(Response::builder().status(200).build())
    }
}
