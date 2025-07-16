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

class CfgVehicles {
    class Man;
    class CAManBase: Man {
        class ACE_SelfActions {
            class danceParty {
                displayName = "Connect to EUD";
                condition = "!(player getVariable ['armatak_client_eudConnected', false])";
                exceptions[] = {};
                statement = "createDialog 'armatak_udp_socket_start_dialog'";
                icon = "";
            };
        };
    };
};

#include "CfgEventHandlers.hpp"
#include "dialog.hpp"
