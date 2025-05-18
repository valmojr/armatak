#include "script_component.hpp"

class CfgPatches {
  class ADDON {
    name = COMPONENT_NAME;
    units[] = {
      "armatak_module_core",
      "armatak_module_callsign"
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

class CfgMods {
    class PREFIX {
      name = "Arma Team Awareness Kit";
      author = PROJECT_AUTHOR;
      logo = "logo_ca.paa";
      logoOver = "logo_hover_ca.paa";
      tooltip	= "ARMATAK";
      tooltipOwned = "ARMATAK";
      picture = "picture.paa";
      actionName = "GitHub";
      action = "https://github.com/valmojr/armatak";
      overview = "ARMATAK Addons allows Arma 3 sessions to be parsed to TAK Clients";
      hideName = 0;
      hidePicture	= 0;
      dlcColor[] = { 0.23, 0.39, 0.30, 1 };
      logoSmall	= "logo_small_ca.paa";
    };
};

#include "CfgEventHandlers.hpp"
#include "CfgVehicles.hpp"