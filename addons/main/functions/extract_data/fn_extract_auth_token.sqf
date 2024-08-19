_atak_server_instance_username = missionNamespace getVariable "_atak_server_instance_username";
_atak_server_instance_password = missionNamespace getVariable "_atak_server_instance_password";

_authentication = [_atak_server_instance_username, _atak_server_instance_password];

_atak_server_instance_token = "armatak" callExtension ["get_auth_token", _authentication];

if (_atak_server_instance_token != "") then {
	missionName setVariable ["_atak_server_instance_token", _atak_server_instance_token];

	private _warning = format ["<t color='#FF0000'>ARMATAK</t><br/> %1", "Authorized"];
	[[_warning, 1.5]] call CBA_fnc_notify;
} else {
	private _warning = format ["<t color='#FF0000'>ARMATAK</t><br/> %1", "Failed to get Auth Token"];
	[[_warning, 1.5]] call CBA_fnc_notify;
};

_atak_server_instance_token