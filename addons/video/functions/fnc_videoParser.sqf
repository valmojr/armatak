#include "..\script_component.hpp"

params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer) exitWith {
	private _instance_address = GETVAR(_logic,GVAR(instanceAddress),false);
	private _instance_port = GETVAR(_logic,GVAR(instancePort),false);
	private _instance_auth_user = GETVAR(_logic,GVAR(instanceAuthUser),false);
	private _instance_auth_pass = GETVAR(_logic,GVAR(instanceAuthPassword),false);

	SETMVAR(GVAR(instanceAddress),_instance_address);
	SETMVAR(GVAR(instancePort),_instance_port);
	SETMVAR(GVAR(instanceAuthUser),_instance_auth_user);
	SETMVAR(GVAR(instanceAuthPassword),_instance_auth_pass);

	_startAction = [
		QGVAR(startStream),
		"Start Video Feed",
		"",
		{
			_uuid = (_this select 0) call armatak_fnc_extract_uuid; 
			_uuid_short = _uuid select [0, 8]; 
			_role = roleDescription (_this select 0); 
			_name = name (_this select 0); 

			_role = [_role] call BIS_fnc_filterString;
			_name = [_name] call BIS_fnc_filterString; 
			
			_stream_path = _name + "_" + _role + "_" + _uuid_short; 

			armatak_mediamtx_video_stream_instance_address = GETMVAR(instance_address,false);
			armatak_mediamtx_video_stream_instance_port = missionNamespace getVariable "instance_port";
			armatak_mediamtx_video_stream_instance_auth_user = missionNamespace getVariable "instance_auth_user";
			armatak_mediamtx_video_stream_instance_auth_pass = missionNamespace getVariable "instance_auth_pass";

			"armatak" callExtension ["video_stream:start", [armatak_mediamtx_video_stream_instance_address, armatak_mediamtx_video_stream_instance_port, _stream_path, armatak_mediamtx_video_stream_instance_auth_user, armatak_mediamtx_video_stream_instance_auth_pass]];
			(_this select 0) setVariable ["armatak_video_feed_is_streaming", true];
		},
		{
			(_this select 0) getVariable "armatak_video_feed_is_streaming" == false
		}
	] call ace_interact_menu_fnc_createAction;
	[
		"Man",
		1,
		["ACE_SelfActions"],
		_startAction,
		true
	] call ace_interact_menu_fnc_addActionToClass;

	_stopAction = [
		"ArmatakStopStream",
		"Stop Video Feed",
		"",
		{
			"armatak" callExtension ["video_stream:stop", []];
			SETVAR(_this select 0,GVAR(isStreaming),false);
		},
		{
			GETVAR((this select 0),GVAR(isStreaming),false)
		}
	] call ace_interact_menu_fnc_createAction;
	[
		"Man",
		1,
		["ACE_SelfActions"],
		_stopAction,
		true
	] call ace_interact_menu_fnc_addActionToClass;
	if (isMultiplayer) then {
		{
			SETVAR(_x,GVAR(isStreaming),false);
		} forEach playableUnits;
	} else {
		SETVAR(player,GVAR(isStreaming),false);
	};
};

true;
