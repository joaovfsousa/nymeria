#include <Arduino.h>
#include "WiFi.h"
#include "ESPAsyncWebServer.h"

int RED = 2;
int YELLOW = 4;
int GREEN = 5;

const char *ssid = "your_ssid_here";
const char *password = "your_password_here";

int currentLed = GREEN;
int areAllLedsOn = 0;

AsyncWebServer server(80);

constexpr unsigned int str2int(char *str, int h = 0)
{
  return !str[h] ? 5381 : (str2int(str, h + 1) * 33) ^ str[h];
}

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

  Serial.print("Connected to the WiFi network, IP: ");
  Serial.println(WiFi.localIP());
}

void setup()
{
  Serial.begin(9600);
  pinMode(2, OUTPUT);
  pinMode(4, OUTPUT);
  pinMode(5, OUTPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  connectToWifi();

  turnOnGreen();

  server.on(
      "/change-status",
      HTTP_POST,
      [](AsyncWebServerRequest *request) {},
      NULL,
      [](AsyncWebServerRequest *request, uint8_t *data, size_t len, size_t index, size_t total)
      {
        int hash = str2int((char *)data);

        switch (hash)
        {
        case str2int("free"):
          turnOnGreen();
          break;
        case str2int("maybe"):
          turnOnYellow();
          break;
        case str2int("busy"):
          turnOnRed();
          break;
        }

        request->send(204);
      });

  server.begin();
}

void loop()
{
  if (WiFi.status() != WL_CONNECTED)
  {
    connectToWifi();
  }
};
