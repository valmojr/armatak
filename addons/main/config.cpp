class CfgPatches {
  class armatak_main {
    units[] = {
      "armatak_module_core",
      "armatak_module_callsign"
    };
    weapons[] = {""};
    author = "Valmo";
    url = "https://github.com/valmojr/armatak";
    requiredAddons[] = {
        "cba_main",
        "ace_main"
      };
    requiredVersion = 2.06;
  };
};

class Extended_PostInit_EventHandlers {
    class armatak_main {
        init = "call compileScript ['\armatak\armatak\armatak_main\initPlayerLocal.sqf']";
    };
};

#include "CfgFunctions.hpp"
#include "CfgVehicles.hpp"
#include "Cfg3den.hpp"
#include "dialog.hpp"