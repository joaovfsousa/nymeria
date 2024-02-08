#ifndef STATE_H
#define STATE_H

#include <map>
#include "LedController.h"

enum State
{
  Free,
  Maybe,
  Busy
};

class StateController
{
public:
  StateController();

  void setDeviceState(std::string deviceId, State state);
  State getState();

  void resetState();

private:
  std::map<std::string, State> deviceStates;
};

std::string getStateString(State state);
State getStateFromString(std::string state);
Led getLedFromState(State state);

#endif