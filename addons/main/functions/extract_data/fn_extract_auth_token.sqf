private _atak_server_instance = missionNamespace getVariable "_atak_server_instance";
private _atak_server_instance_username = missionNamespace getVariable "_atak_server_instance_username";
private _atak_server_instance_password = missionNamespace getVariable "_atak_server_instance_password";

private _authentication = [_atak_server_instance, _atak_server_instance_username, _atak_server_instance_password];

_atak_server_instance_token = ("armatak" callExtension ["get_auth_token", _authentication]) select 0;

if ((_atak_server_instance_token != "") and !(["error", _atak_server_instance_token, false] call BIS_fnc_inString)) then {
	missionNamespace setVariable ["_atak_server_instance_token", _atak_server_instance_token];

	private _warning = format ["<t color='#2B7319'>ARMATAK</t><br/> %1", "Authorized"];
	[[_warning, 1.5]] call CBA_fnc_notify;
} else {
	missionNamespace setVariable ["_atak_server_instance_token", _atak_server_instance_token];

	private _warning = format ["<t color='#FF0000'>ARMATAK</t><br/> %1", "Failed to get Auth Token"];
	[[_warning, 1.5]] call CBA_fnc_notify;
};

_atak_server_instance_token