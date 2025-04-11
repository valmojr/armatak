params [
	["_logic", objNull, [objNull]],
	["_units", [], [[]]],
	["_activated", true, [true]]
];

if (isServer) exitWith {
	armatak_module_mediamtx_video_stream_instance_address = _logic getVariable "armatak_module_mediamtx_video_stream_instance_address";
	armatak_module_mediamtx_video_stream_instance_port = _logic getVariable "armatak_module_mediamtx_video_stream_instance_port";
	armatak_module_mediamtx_video_stream_instance_auth_user = _logic getVariable "armatak_module_mediamtx_video_stream_instance_auth_user";
	armatak_module_mediamtx_video_stream_instance_auth_pass = _logic getVariable "armatak_module_mediamtx_video_stream_instance_auth_pass";

	missionNamespace setVariable ["armatak_mediamtx_video_stream_instance_address", armatak_module_mediamtx_video_stream_instance_address];
	missionNamespace setVariable ["armatak_mediamtx_video_stream_instance_port", armatak_module_mediamtx_video_stream_instance_port];
	missionNamespace setVariable ["armatak_mediamtx_video_stream_instance_auth_user", armatak_module_mediamtx_video_stream_instance_auth_user];
	missionNamespace setVariable ["armatak_mediamtx_video_stream_instance_auth_pass", armatak_module_mediamtx_video_stream_instance_auth_pass];

	_startAction = [
		"ArmatakStartStream",
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

			armatak_mediamtx_video_stream_instance_address = missionNamespace getVariable "armatak_mediamtx_video_stream_instance_address";
			armatak_mediamtx_video_stream_instance_port = missionNamespace getVariable "armatak_mediamtx_video_stream_instance_port";
			armatak_mediamtx_video_stream_instance_auth_user = missionNamespace getVariable "armatak_mediamtx_video_stream_instance_auth_user";
			armatak_mediamtx_video_stream_instance_auth_pass = missionNamespace getVariable "armatak_mediamtx_video_stream_instance_auth_pass";

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
			(_this select 0) setVariable ["armatak_video_feed_is_streaming", false];
		},
		{
			(_this select 0) getVariable "armatak_video_feed_is_streaming"
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
			_x setVariable ["armatak_video_feed_is_streaming", false];
		} forEach playableUnits;
	} else {
		player setVariable ["armatak_video_feed_is_streaming", false];
	};
};

true;