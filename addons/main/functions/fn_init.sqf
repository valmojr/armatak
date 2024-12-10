params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer && _activated) exitWith {
	private _warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Connecting to OTS Server..."];
	[[_warning, 1.5]] call CBA_fnc_notify;

	private _atak_instance_protocol = _logic getVariable "armatak_module_api_instance_protocol";
	private _atak_instance_address = _logic getVariable "armatak_module_api_instance_address";
	private _atak_instance_port = _logic getVariable "armatak_module_api_instance_port";

	private _atak_fulladdress = _atak_instance_protocol + ":" + "/" + "/" + _atak_instance_address + ":" + (str _atak_instance_port);
	private _atak_api_username = _logic getVariable "armatak_module_api_instance_username";
	private _atak_api_password = _logic getVariable "armatak_module_api_instance_password";

	missionNamespace setVariable ["_atak_server_instance", _atak_fulladdress];
	missionNamespace setVariable ["_atak_server_instance_username", _atak_api_username];
	missionNamespace setVariable ["_atak_server_instance_password", _atak_api_password];
	/*
	_atak_server_instance_token = call armatak_fnc_extract_auth_token;
	 
	if (_atak_server_instance_token == "") then {
		private _warning = format ["<t color='#FF0000'>ARMATAK</t><br/> %1", "Connection Failed"];
		[[_warning, 2]] call CBA_fnc_notify;
	} else {
		private _warning = format ["<t color='#2B7319'>ARMATAK</t><br/> %1", "Connected"];
		[[_warning, 2]] call CBA_fnc_notify;
	};
	*/
};

true;