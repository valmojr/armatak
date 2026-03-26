#include "..\script_component.hpp"

params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer) exitWith {
	if (missionNamespace getVariable ["armatak_tcp_socket_is_running", false]) exitWith {
		["Socket was called twice", "error", "TCP Socket"] call EFUNC(main,notify);
	};

	["Connecting to TCP Socket", "success", "TCP Socket"] call EFUNC(main,notify);

	_tak_server_instance_address = _logic getVariable [QGVAR(moduleInstanceAddress), "localhost"];
	_tak_server_instance_port = _logic getVariable [QGVAR(moduleInstancePort), 8088];
	_tak_server_fulladdress = _tak_server_instance_address + ":" + (str _tak_server_instance_port);

	"armatak" callExtension ["tcp_socket:start", [_tak_server_fulladdress]];

	missionNamespace setVariable ["armatak_server_syncedUnits", synchronizedObjects _logic];
	_tak_server_fulladdress call FUNC(startCotRouter);
};

true
