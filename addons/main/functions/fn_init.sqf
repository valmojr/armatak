params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (_activated) exitWith {
	private _warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Connecting..."];
	[[_warning, 1.5]] call CBA_fnc_notify;

	_atak_fts_address = _logic getVariable "armatak_module_fts_api_instance_address";
	_atak_fts_protocol = _logic getVariable "armatak_module_fts_api_instance_protocol";
	_atak_fts_port = _logic getVariable "armatak_module_fts_api_instance_port";

	_atak_fts_fulladdress = _atak_fts_protocol + ":" + "/" + "/" + _atak_fts_address + ":" + (str _atak_fts_port);
	_atak_fts_bearer_token = _logic getVariable "armatak_module_fts_api_instance_token";

	missionNamespace setVariable ["_atak_server_instance",_atak_fts_fulladdress];
	missionNamespace setVariable ["_atak_server_instance_token",_atak_fts_bearer_token];

	[{
		_uid = _x getVariable "_atak_uid";
		if (isNull _uid) then {
			player call armatak_fnc_postGeoObject;
		} else {
			player call armatak_fnc_putGeoObject;
		};
	},2,[]] call CBA_fnc_addPerFrameHandler;
};