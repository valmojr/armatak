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

	["Connecting to authenticated TAK socket", "success", "TCP Socket"] call EFUNC(main,notify);

	_tak_server_instance_address = _logic getVariable [QGVAR(moduleInstanceAddress), "localhost"];
	_tak_server_enrollment_port = _logic getVariable [QGVAR(moduleEnrollmentPort), 8446];
	_tak_server_enrollment_username = _logic getVariable [QGVAR(moduleEnrollmentUsername), ""];
	_tak_server_enrollment_password = _logic getVariable [QGVAR(moduleEnrollmentPassword), ""];

	"armatak" callExtension [
		"tcp_socket:start_enroll_mtls",
		[
			_tak_server_instance_address,
			_tak_server_instance_address,
			str _tak_server_enrollment_port,
			_tak_server_enrollment_username,
			_tak_server_enrollment_password,
			""
		]
	];

	missionNamespace setVariable ["armatak_server_syncedUnits", synchronizedObjects _logic];
	_tak_server_instance_address call FUNC(startCotRouter);
};

true
