class CfgPatches {
  class armatak_main {
    units[] = {"armatak_module_core","armatak_module_callsign"};
    weapons[] = {""};
    author = "ARMATAK Team";
    url = "https://github.com/valmojr/armatak";
    requiredAddons[] =
      {
        "cba_common"
      };
    requiredVersion = 0.5;
  };
};

#include "CfgCommands.hpp"
#include "CfgFunctions.hpp"
#include "CfgVehicles.hpp"