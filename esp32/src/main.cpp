#include <Arduino.h>
#include "ESPAsyncWebServer.h"
#include <map>
#include "LedController.h"
#include "WifiHelper.h"

Led lastLedOn = Led::Green;

AsyncWebServer server(80);

LedController ledController = LedController();

constexpr unsigned int str2int(char *str, int h = 0)
{
  return !str[h] ? 5381 : (str2int(str, h + 1) * 33) ^ str[h];
}

void setup()
{
  ledController.setup();

  Serial.begin(9600);

  connectToWifi(ledController);

  server.on(
      "/change-status",
      HTTP_POST,
      [](AsyncWebServerRequest *request) {},
      NULL,
      [](AsyncWebServerRequest *request, uint8_t *data, size_t len, size_t index, size_t total)
      {
        std::map<std::string, Led> ledMap = {
            {"free", Led::Green},
            {"maybe", Led::Yellow},
            {"busy", Led::Red}};

        Led led = ledMap[std::string((char *)data, len)];

        lastLedOn = led;

        ledController.setOnly(led, LedState::On);

        request->send(204);
      });

  Serial.println("Server starting at port 80...");
  server.begin();
  Serial.println("Server started at port 80");

  ledController.setOnly(lastLedOn, LedState::On);
}

void loop()
{
  if (WiFi.status() != WL_CONNECTED)
  {
    reconnectWifi();
    ledController.blink(false);
  }
  else
  {
    ledController.setOnly(lastLedOn, LedState::On);
    delay(1000);
  }
};
