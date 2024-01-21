#include "WiFi.h"
#include "LedController.h"

const char *ssid = "your_ssid_here";
const char *password = "your_password_here";

unsigned long reconnectPreviousMillis = 0;
unsigned long reconnectInterval = 30000;

void connectToWifi(LedController ledController)
{
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED)
  {
    delay(300);
    ledController.blink(false);
  }

  ledController.stopBlinking();

  Serial.print("Connected to the WiFi network, IP: ");
  Serial.println(WiFi.localIP());
}

void reconnectWifi()
{
  unsigned long currentMillis = millis();
  if (currentMillis - reconnectPreviousMillis >= reconnectInterval)
  {
    Serial.println("Reconnecting to WiFi...");
    WiFi.disconnect();
    WiFi.reconnect();
    reconnectPreviousMillis = currentMillis;
  }
}