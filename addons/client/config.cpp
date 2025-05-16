#include "script_component.hpp"

class CfgPatches {
  class ADDON {
    name = COMPONENT_NAME;
    units[] = {};
    weapons[] = {};
    requiredAddons[] = {
        "cba_main",
        "ace_main",
        "armatak_main"
      };
    requiredVersion = REQUIRED_VERSION;
    author = PROJECT_AUTHOR;
    url = "https://github.com/valmojr/armatak";
  };
};

class Extended_PostInit_EventHandlers {
    class armatak_main {
        init = "call compileScript ['\armatak\armatak\armatak_client\initPlayerLocal.sqf']";
    };
};

#include "CfgEventHandlers.hpp"