#include "..\script_component.hpp"

params ["_logic"];

if (missionNamespace getVariable ["armatak_tcp_socket_is_running", false]) exitWith {
	["Socket was called twice", "error", "TCP Socket"] call EFUNC(main,notify);
	closeDialog 1;
};

disableSerialization;

["Connecting to authenticated TAK socket", "success", "TCP Socket"] call EFUNC(main,notify);

_tak_server_instance_address = ctrlText 14100;
_tak_server_enrollment_port = ctrlText 14101;
_tak_server_enrollment_username = ctrlText 14102;
_tak_server_enrollment_password = ctrlText 14103;

"armatak" callExtension [
	"tcp_socket:start_enroll_mtls",
	[
		_tak_server_instance_address,
		_tak_server_instance_address,
		_tak_server_enrollment_port,
		_tak_server_enrollment_username,
		_tak_server_enrollment_password,
		""
	]
];

_tak_server_instance_address call FUNC(startCotRouter);
deleteVehicle _logic;
closeDialog 1;
