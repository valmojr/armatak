#include "script_component.hpp"

class CfgPatches {
  class ADDON {
    name = COMPONENT_NAME;
    units[] = {
			QGVAR(coreModule),
			QGVAR(coreModuleCurator),
      QGVAR(markEntity)
		};
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

#include "CfgEventHandlers.hpp"
#include "dialog.hpp"
#include "CfgVehicles.hpp"
