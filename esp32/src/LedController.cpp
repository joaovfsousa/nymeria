#include <Arduino.h>
#include <map>
#include <LedController.h>

LedInfo::LedInfo(int pin, LedState state)
    : pin(pin), state(state){};

LedController::LedController()
{
  std::map<Led, LedInfo> ledMap;

  ledMap[Led::Green] = LedInfo(5, LedState::Off);
  ledMap[Led::Yellow] = LedInfo{4, LedState::Off};
  ledMap[Led::Red] = LedInfo{2, LedState::Off};

  this->ledMap = ledMap;
}

void LedController::set(Led led, LedState state)
{
  LedInfo ledInfo = ledMap[led];

  int digitalState = state == LedState::On ? HIGH : LOW;

  ledInfo.state = state;
  digitalWrite(ledInfo.pin, digitalState);
}

void LedController::setOnly(Led led, LedState state)
{
  for (auto const &x : ledMap)
  {
    set(x.first, LedState::Off);
  }

  set(led, state);
}

void LedController::blink(boolean isStopSignal)
{
  unsigned int currentMillis = millis();
  if (currentMillis - this->lastBlink < this->blinkInterval && !isStopSignal)
  {
    return;
  }

  this->lastBlink = currentMillis;

  int digitalState = allLedsState == LedState::On ? LOW : HIGH;

  for (auto const &x : ledMap)
  {
    set(x.first, allLedsState);
  }

  allLedsState = allLedsState == LedState::On ? LedState::Off : LedState::On;
}

void LedController::stopBlinking()
{
  if (this->allLedsState == LedState::On)
  {
    blink(true);
  }
}

void LedController::setup()
{
  for (auto const &x : ledMap)
  {
    pinMode(x.second.pin, OUTPUT);
  }
}