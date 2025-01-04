params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer) exitWith {
	_warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Connecting to API Provider..."];
	[[_warning, 1.5]] call CBA_fnc_notify;

	_atak_instance = _logic getVariable "armatak_module_api_instance";
	_atak_instance_protocol = _logic getVariable "armatak_module_api_instance_protocol";
	_atak_instance_address = _logic getVariable "armatak_module_api_instance_address";
	_atak_instance_port = _logic getVariable "armatak_module_api_instance_port";

	_atak_fulladdress = _atak_instance_protocol + ":" + "/" + "/" + _atak_instance_address + ":" + (str _atak_instance_port);
	_atak_api_username = _logic getVariable "armatak_module_api_instance_username";
	_atak_api_password = _logic getVariable "armatak_module_api_instance_password";

	if (_atak_instance == "ots") then {
		missionNamespace setVariable ["_atak_server_instance", _atak_fulladdress];
		missionNamespace setVariable ["_atak_server_instance_username", _atak_api_username];
		missionNamespace setVariable ["_atak_server_instance_password", _atak_api_password];

		_atak_server_instance_token = call armatak_fnc_extract_auth_token;

		if ((_atak_server_instance_token == "") and !(["ERROR", _atak_server_instance_token, false] call BIS_fnc_inString)) then {
			private _warning = format ["<t color='#FF0000'>ARMATAK</t><br/> %1", "Connection Failed"];
			[[_warning, 2]] call CBA_fnc_notify;
		} else {
			private _warning = format ["<t color='#2B7319'>ARMATAK</t><br/> %1", "Connected"];
			[[_warning, 2]] call CBA_fnc_notify;
		};

		_syncUnits = synchronizedObjects _logic;

		missionNamespace setVariable ["_armatak_marked_units", _syncUnits];

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
	};
};

true;
