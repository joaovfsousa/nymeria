#include "State.h"

std::map<std::string, State> stringToState = {
    {"free", State::Free},
    {"maybe", State::Maybe},
    {"busy", State::Busy}};

std::map<State, std::string> stateToString = {
    {State::Free, "free"},
    {State::Maybe, "maybe"},
    {State::Busy, "busy"}};

StateController::StateController()
{
  this->deviceStates = std::map<std::string, State>();
}

void StateController::setDeviceState(std::string deviceId, State state)
{
  this->deviceStates[deviceId] = state;
}

State StateController::getState()
{
  State highestState = State::Free;

  for (const auto &pair : this->deviceStates)
  {
    if (pair.second > highestState)
    {
      highestState = pair.second;
    }
  }

  return highestState;
}

void StateController::resetState()
{
  this->deviceStates.clear();
}

std::string getStateString(State state)
{
  return stateToString[state];
}

State getStateFromString(std::string state)
{
  return stringToState[state];
}

Led getLedFromState(State state)
{
  std::map<State, Led> stateToLed = {
      {State::Free, Led::Green},
      {State::Maybe, Led::Yellow},
      {State::Busy, Led::Red}};

  return stateToLed[state];
}