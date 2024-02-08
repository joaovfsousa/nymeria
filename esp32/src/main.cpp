#include <Arduino.h>
#include "ESPAsyncWebServer.h"
#include <map>
#include "LedController.h"
#include "WifiHelper.h"
#include "State.h"

Led lastLedOn = Led::Green;

AsyncWebServer server(80);

LedController ledController = LedController();

StateController stateController = StateController();

void setup()
{
  ledController.setup();

  Serial.begin(9600);

  connectToWifi(ledController);

  server.on(
      "^\\/devices\\/([a-z0-9]+)\\/state$",
      HTTP_POST,
      [](AsyncWebServerRequest *request) {},
      NULL,
      [](AsyncWebServerRequest *request, uint8_t *data, size_t len, size_t index, size_t total)
      {
        String deviceId = request->pathArg(0);

        State deviceState = getStateFromString(std::string((char *)data, len));

        stateController.setDeviceState(deviceId.c_str(), deviceState);

        State state = stateController.getState();

        Led led = getLedFromState(state);

        ledController.setOnly(led, LedState::On);
        lastLedOn = led;

        request->send(200, "text/plain", getStateString(state).c_str());
      });

  server.on(
      "/state",
      HTTP_GET,
      [](AsyncWebServerRequest *request)
      {
        State state = stateController.getState();

        request->send(200, "text/plain", getStateString(state).c_str());
      });

  server.on(
      "/reset",
      HTTP_POST,
      [](AsyncWebServerRequest *request)
      {
        stateController.resetState();

        State state = stateController.getState();

        Led led = getLedFromState(state);

        ledController.setOnly(led, LedState::On);
        lastLedOn = led;

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
