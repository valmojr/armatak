#include "..\script_component.hpp"

if (!hasInterface) exitWith {};

_local_address = "armatak" callExtension ["local_ip", []] select 0;

SETVAR(player,localAddress,_local_address);

[{
	"armatak" callExtension ["websocket:location", [player call FUNC(extractClientPosition)]];
}, 1, []] call CBA_fnc_addPerFrameHandler;
