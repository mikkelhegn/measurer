#include "SPI.h"
#include "WiFiSSLClient.h"
#include <WiFiS3.h>
#include <DHT22.h>
#include <ArduinoHttpClient.h>
#include <ArduinoJson.h>
#include <NTPClient.h>
#include "ArduinoGraphics.h"
#include "Arduino_LED_Matrix.h"

// Boardname
const char name[] = "Board 1";
const char short_name[] = "1";

// Matrix
ArduinoLEDMatrix matrix;

// Sonsor
#define pinDATA 12
DHT22 dht22(pinDATA);

// Wifi
WiFiSSLClient wifi;
//WiFiClient wifi;

// Setting up Time server using 'pool.ntp.org'
WiFiUDP ntpUDP;
NTPClient timeClient(ntpUDP);

// HttpClient
char serverAddress[] = "temphum-li5ce2mr.fermyon.app";
int port = 443;
HttpClient client = HttpClient(wifi, serverAddress, port);
int status = WL_IDLE_STATUS;
JsonDocument doc;

void setup() {
  Serial.begin(9600);
  status = WiFi.begin("Sagavaga", "Sagavagamaximus");
  pinMode(LED_BUILTIN, OUTPUT);
  matrix.begin();
}

void loop() {
  // Update time
  timeClient.update();

  matrix.beginDraw();
  matrix.stroke(0xFFFFFFFF);
  matrix.textFont(Font_4x6);
  matrix.beginText(1, 1, 0xFFFFFF);
  matrix.println(short_name);
  matrix.endText();
  matrix.endDraw();

  // Wait to send the data package
  delay(1800000);
  //delay(5000);
  
  // Turn light on
  digitalWrite(LED_BUILTIN, HIGH);

  // Get sensor data
  float t = dht22.getTemperature();
  float h = dht22.getHumidity();

  // Construct JSON
  doc["device_id"] = name;
  doc["epoch_time"] = timeClient.getEpochTime();
  doc["humidity"] = h;
  doc["temperature"] = t;
  char output[256];
  serializeJson(doc, output);

  // Send request
  String contentType = "application/json";
  String postData = output;
  client.post("/api", contentType, postData);
  client.responseStatusCode();
  client.responseBody();

  matrix.beginDraw();
  matrix.stroke(0xFFFFFFFF);
  matrix.textScrollSpeed(60);
  // add the text
  char humidity[8];
  dtostrf(h, 3, 1, humidity);
  char temperature[8];
  dtostrf(t, 3, 1, temperature);
  char str[20];
  strcpy (str," b: ");
  strcat (str,name);
  strcat (str," t: ");
  strcat (str,temperature);
  strcat (str," h: ");
  strcat (str,humidity);
  puts (str);

  matrix.textFont(Font_4x6);
  matrix.beginText(0, 1, 0xFFFFFF);
  matrix.println(str);
  matrix.endText(SCROLL_LEFT);
  matrix.endDraw();

  // Turn light off
  digitalWrite(LED_BUILTIN, LOW);
}