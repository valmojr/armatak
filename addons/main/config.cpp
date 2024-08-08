class CfgPatches {
  class armatak_main {
    units[] = {
      "TAG_Module_Nuke",
      "armatak_module_callsign"
    };
    weapons[] = {""};
    author = "Valmo";
    url = "https://github.com/valmojr/armatak";
    requiredAddons[] =
      {
        "cba_main",
        "ace_main"
      };
    requiredVersion = 1.0;
  };
};

#include "CfgCommands.hpp"
#include "CfgFunctions.hpp"
#include "CfgVehicles.hpp"