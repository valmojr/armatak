class CfgPatches {
  class armatak_main {
    units[] = {"TAG_Module_Nuke","armatak_module_callsign"};
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