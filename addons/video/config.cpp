#include "script_component.hpp"

class CfgPatches {
  class ADDON {
    name = COMPONENT_NAME;
    units[] = {
      QGVAR(videoModule)
    };
    weapons[] = {};
    requiredAddons[] = {
        "cba_main",
        "ace_main",
        "armatak_main",
        "armatak_server"
      };
    requiredVersion = REQUIRED_VERSION;
    author = PROJECT_AUTHOR;
    url = "https://github.com/valmojr/armatak";
  };
};

#include "CfgEventHandlers.hpp"
//#include "CfgVehicles.hpp"
