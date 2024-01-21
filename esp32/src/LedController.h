#ifndef LED_CONTROLLER_H
#define LED_CONTROLLER_H

#include <Arduino.h>
#include <map>

enum LedState
{
  On,
  Off
};

enum Led
{
  Green,
  Yellow,
  Red
};

class LedInfo
{
public:
  int pin;
  LedState state;

  LedInfo(){};

  LedInfo(int pin, LedState state);
};

class LedController
{
public:
  LedController();

  void set(Led led, LedState state);
  void setOnly(Led led, LedState state);

  void blink(boolean isStopSignal);
  void stopBlinking();
  void setup();

private:
  std::map<Led, LedInfo> ledMap;
  LedState allLedsState = LedState::Off;
  unsigned int lastBlink = 0;
  unsigned int blinkInterval = 500;
};

#endif