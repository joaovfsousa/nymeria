#include <Arduino.h>
#include "WiFi.h"
#include "HTTPClient.h"

int RED = 2;
int YELLOW = 4;
int GREEN = 5;

const char *ssid = "your_ssid_here";
const char *password = "your_password_here";

HTTPClient http;

int currentLed = GREEN;
int areAllLedsOn = 0;

String url = "http://192.168.0.111/led-status";

void turnOnGreen()
{
  digitalWrite(GREEN, HIGH);
  digitalWrite(YELLOW, LOW);
  digitalWrite(RED, LOW);
  currentLed = GREEN;
}

void turnOnRed()
{
  digitalWrite(GREEN, LOW);
  digitalWrite(YELLOW, LOW);
  digitalWrite(RED, HIGH);
  currentLed = RED;
}

void turnOnYellow()
{
  digitalWrite(GREEN, LOW);
  digitalWrite(YELLOW, HIGH);
  digitalWrite(RED, LOW);
  currentLed = YELLOW;
}

void toggleAllLeds()
{
  int value = LOW;

  if (!areAllLedsOn)
  {
    value = HIGH;
    areAllLedsOn = 1;
  }
  else
  {
    areAllLedsOn = 0;
  }

  digitalWrite(GREEN, value);
  digitalWrite(YELLOW, value);
  digitalWrite(RED, value);
}

void connectToWifi()
{
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);

  digitalWrite(LED_BUILTIN, HIGH);
  while (WiFi.status() != WL_CONNECTED)
  {
    delay(300);
    toggleAllLeds();
  }

  if (areAllLedsOn)
  {
    toggleAllLeds();
  }

  digitalWrite(LED_BUILTIN, LOW);
}

void setup()
{
  Serial.begin(9600);
  pinMode(2, OUTPUT);
  pinMode(4, OUTPUT);
  pinMode(5, OUTPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  connectToWifi();
  turnOnYellow();
}

void loop()
{
  if (WiFi.status() != WL_CONNECTED)
  {
    connectToWifi();
  }

  http.setTimeout(10000);
  http.setConnectTimeout(10000);
  http.begin(url.c_str());

  int httpResCode = http.GET();

  if (httpResCode == 200)
  {
    String payload = http.getString();
    if (payload == "free")
    {
      turnOnGreen();
    }
    else if (payload == "maybe")
    {
      turnOnYellow();
    }
    else if (payload == "busy")
    {
      turnOnRed();
    }
  }
  else
  {
    Serial.println(httpResCode);
    turnOnYellow();
  }

  http.end();

  delay(15000);
};
