#include "script_component.hpp"

if (!hasInterface) exitWith {};

_local_address = "armatak" callExtension ["local_ip", []] select 0;

"armatak" callExtension ["websocket:start", []];

SETVAR(player,GVAR(localAddress),_local_address);
SETVAR(player,GVAR(eudConnected),false);

[{
	"armatak" callExtension ["websocket:location", [player call armatak_client_fnc_extractClientPosition]];
}, 1, []] call CBA_fnc_addPerFrameHandler;
