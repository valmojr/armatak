params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer) exitWith {
	_warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Connecting to TAK Server TCP Socket..."];
	[[_warning, 1.5]] call CBA_fnc_notify;

	_tak_server_instance_address = _logic getVariable "armatak_module_tak_server_instance_address";
	_tak_server_instance_port = _logic getVariable "armatak_module_tak_server_instance_port";

	_tak_server_fulladdress = _tak_server_instance_address + ":" + (str _tak_server_instance_port);

	missionNamespace setVariable ["_atak_server_instance", _tak_server_fulladdress];
	missionNamespace setVariable ["_group_colors", ["White", "Yellow", "Orange", "Magenta", "Red", "Maroon", "Purple", "DarkBlue", "Blue", "Cyan", "Teal", "Green", "DarkGreen", "Brown"]];

	"armatak" callExtension ["cot_router:start", [_tak_server_fulladdress]];

	/*
	[{
		[{
			_syncedUnits = missionNamespace getVariable "_armatak_marked_units";
			_markers = [];

			{
				if (unitIsUAV _x) then {
					_marker = _x call armatak_fnc_extract_drone_info;
					_markers append [_marker];
				} else {
					_marker = _x call armatak_fnc_extract_info;
					_markers append [_marker];
				};
			} forEach _syncedUnits;

			_request = "armatak" callExtension ["ots_api:post", [_markers]];
		}, 1, []] call CBA_fnc_addPerFrameHandler;
	}, [], 1] call CBA_fnc_waitAndExecute;
	*/
};

true;
