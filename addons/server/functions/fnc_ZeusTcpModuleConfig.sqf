#include "..\script_component.hpp"

params ["_logic"];

if (missionNamespace getVariable ["armatak_tcp_socket_is_running", false]) exitWith {
	["Socket was called twice", "error", "TCP Socket"] call EFUNC(main,notify);
	closeDialog 1;
};

disableSerialization;

["Connecting to TCP Socket", "success", "TCP Socket"] call EFUNC(main,notify);

_tak_server_instance_address = ctrlText 14000;
_tak_server_instance_port = ctrlText 14001;
_tak_server_fulladdress = _tak_server_instance_address + ":" + _tak_server_instance_port;

"armatak" callExtension ["tcp_socket:start", [_tak_server_fulladdress]];

_tak_server_fulladdress call FUNC(startCotRouter);
deleteVehicle _logic;
closeDialog 1;
