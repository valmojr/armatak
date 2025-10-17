#include "..\script_component.hpp"

params ["_logic"];

_socket_is_running = player getVariable [QGVAR(eudConnected), false];

if (_socket_is_running) exitWith {
	["Socket is already running", "error", "UDP Socket"] call EFUNC(main,notify);
	closeDialog 1;
};

disableSerialization;

_udp_client_instance_address = ctrlText 16961;
_udp_client_instance_port = ctrlText 16962;

_udp_client_fulladdress = ((_udp_client_instance_address) + ":" + (_udp_client_instance_port));

player setVariable [QGVAR(udp_client_address), _udp_client_fulladdress];
player setVariable [QGVAR(eudConnected), true];

"armatak" callExtension ["udp_client:start", [_udp_client_fulladdress]];

[{
	if (player getVariable [QGVAR(eudConnected), false]) then {
		"armatak" callExtension ["udp_client:send_gps_cot", [player call FUNC(extractClientPosition)]];
	};
}, 0.1, []] call CBA_fnc_addPerFrameHandler;

deleteVehicle _logic;
closeDialog 1;
