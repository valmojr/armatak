#include "..\script_component.hpp"

params ["_function", ["_data", "", [""]]];

if (!hasInterface) exitWith {};

private _payload = [_data] call FUNC(parseMavlinkCallbackData);
private _uav = getConnectedUAV player;
if (isNull _uav) then {
	_uav = player getVariable [QGVAR(broadcastingUav), objNull];
};

if (isNull _uav) exitWith {
	"armatak" callExtension ["log", [["warn", format ["Ignoring MAVLINK UDP callback %1 because no UAV is connected: %2", _function, _data]]]];
};

private _number = {
	params ["_key", ["_default", 0]];
	private _raw = _payload getOrDefault [_key, str _default];
	private _value = parseNumber _raw;
	if (!finite _value) exitWith {_default};
	_value
};

private _uavGroup = {
	params ["_vehicle"];
	private _crew = crew _vehicle;
	if (_crew isEqualTo []) exitWith {grpNull};
	group (_crew select 0)
};

private _clearWaypoints = {
	params ["_group"];
	if (isNull _group) exitWith {};
	for "_i" from ((count waypoints _group) - 1) to 0 step -1 do {
		deleteWaypoint [_group, _i];
	};
};

private _clearUavRoute = {
	private _group = [_uav] call _uavGroup;
	if (isNull _group) exitWith {false};
	[_group] call _clearWaypoints;
	_uav setVariable ["armatak_uas_mission_items", [], true];
	true
};

private _geoToAtl = {
	params ["_vehicle", "_lat", "_lon", ["_alt", -1]];

	private _current = [_vehicle] call EFUNC(client,extractClientPosition);
	private _currentLat = _current select 1;
	private _currentLon = _current select 2;
	private _currentAtl = getPosATL _vehicle;

	private _northM = (_lat - _currentLat) * 111320;
	private _eastM = (_lon - _currentLon) * (111320 * (cos _currentLat));

	[
		(_currentAtl select 0) + _eastM,
		(_currentAtl select 1) + _northM,
		if (_alt >= 0) then {_alt} else {_currentAtl select 2}
	]
};

private _commandMove = {
	params ["_vehicle", "_positionAtl", ["_type", "MOVE"], ["_radius", 80], ["_completion", 50]];

	private _group = [_vehicle] call _uavGroup;
	if (isNull _group) exitWith {false};

	[_group] call _clearWaypoints;

	_vehicle engineOn true;
	_vehicle setVariable ["armatak_uas_armed", true, true];
	_vehicle setFuel ((fuel _vehicle) max 0.1);
	_vehicle flyInHeight ((_positionAtl select 2) max 10);
	_vehicle doMove _positionAtl;

	private _wp = _group addWaypoint [_positionAtl, 0];
	_wp setWaypointType _type;
	_wp setWaypointBehaviour "CARELESS";
	_wp setWaypointCombatMode "BLUE";
	_wp setWaypointSpeed "NORMAL";
	_wp setWaypointCompletionRadius _completion;

	if (_type == "LOITER") then {
		_wp setWaypointLoiterRadius (_radius max 25);
		_wp setWaypointLoiterType "CIRCLE_L";
	};

	true
};

private _appendMissionWaypoint = {
	params ["_vehicle", "_positionAtl", "_command", "_seq", ["_radius", 80]];

	private _group = [_vehicle] call _uavGroup;
	if (isNull _group) exitWith {false};

	private _type = switch (_command) do {
		case 17;
		case 18;
		case 19;
		case 31: {"LOITER"};
		case 21: {"MOVE"};
		default {"MOVE"};
	};

	private _wp = _group addWaypoint [_positionAtl, 0];
	_wp setWaypointType _type;
	_wp setWaypointBehaviour "CARELESS";
	_wp setWaypointCombatMode "BLUE";
	_wp setWaypointSpeed "NORMAL";
	_wp setWaypointCompletionRadius 35;

	if (_type == "LOITER") then {
		_wp setWaypointLoiterRadius (_radius max 25);
		_wp setWaypointLoiterType "CIRCLE_L";
	};
	if (_command == 21) then {
		_wp setWaypointStatements ["true", "(vehicle this) land 'LAND'"];
	};

	private _items = _vehicle getVariable ["armatak_uas_mission_items", []];
	_items pushBack [_seq, _command, _positionAtl];
	_vehicle setVariable ["armatak_uas_mission_items", _items, true];

	true
};

private _commandName = _payload getOrDefault ["command_name", "UNKNOWN"];
private _command = [_payload getOrDefault ["command", "-1"]] call BIS_fnc_parseNumber;
private _callsign = [_uav] call armatak_fnc_extract_marker_callsign;

private _applySpeed = {
	params ["_speed"];
	if (_speed <= 0) exitWith {false};
	_uav limitSpeed _speed;
	systemChat format ["ATAK SPEED %1m/s %2", round _speed, _callsign];
	true
};

private _applyMode = {
	params ["_mode"];

	switch (_mode) do {
		case 4: {
			_uav engineOn true;
			_uav setVariable ["armatak_uas_armed", true, true];
			_uav setFuel ((fuel _uav) max 0.1);
			systemChat format ["ATAK GUIDED %1", _callsign];
		};
		case 5: {
			private _pos = getPosATL _uav;
			[_uav, _pos, "LOITER", 80, 25] call _commandMove;
			systemChat format ["ATAK LOITER %1", _callsign];
		};
		case 6;
		case 21;
		case 27: {
			private _home = _uav getVariable ["armatak_uas_home_atl", getPosATL _uav];
			_home set [2, ((_home select 2) max 60)];
			[_uav, _home, "MOVE", 80, 60] call _commandMove;
			systemChat format ["ATAK RTL %1", _callsign];
		};
		case 9: {
			private _pos = getPosATL _uav;
			_pos set [2, 0];
			[_uav, _pos, "MOVE", 30, 20] call _commandMove;
			_uav flyInHeight 0;
			systemChat format ["ATAK LAND %1", _callsign];
		};
		default {
			"armatak" callExtension ["log", [["info", format ["Unhandled MAVLINK mode %1 for UAV %2", _mode, _uav]]]];
		};
	};
};

private _setHomeFromGeo = {
	params ["_lat", "_lon", "_alt"];
	if (_lat == 0 && {_lon == 0}) exitWith {false};
	private _homeAtl = [_uav, _lat, _lon, _alt] call _geoToAtl;
	_uav setVariable ["armatak_uas_home_atl", _homeAtl, true];
	_uav setVariable ["armatak_uas_home_geo", [_lat, _lon, _alt], true];
	systemChat format ["ATAK HOME %1", _callsign];
	true
};

switch (_function) do {
	case "COMMAND_LONG": {
		switch (_command) do {
			case 176: {
				private _mode = ["param2", -1] call _number;
				if (_mode < 0) then {
					_mode = ["param1", -1] call _number;
				};
				[_mode] call _applyMode;
			};
			case 178: {
				private _speed = ["param2", -1] call _number;
				if (_speed <= 0) then {
					_speed = ["param1", -1] call _number;
				};
				[_speed] call _applySpeed;
			};
			case 179: {
				private _useCurrent = (["param1", 0] call _number) >= 1;
				if (_useCurrent) then {
					private _pos = [_uav] call EFUNC(client,extractClientPosition);
					private _relAlt = ((getPosATL _uav) select 2) max 0;
					private _homeAtl = getPosATL _uav;
					_uav setVariable ["armatak_uas_home_atl", _homeAtl, true];
					_uav setVariable ["armatak_uas_home_geo", [_pos select 1, _pos select 2, (_pos select 3) - _relAlt], true];
					systemChat format ["ATAK HOME %1", _callsign];
				} else {
					[["param5", 0] call _number, ["param6", 0] call _number, ["param7", 0] call _number] call _setHomeFromGeo;
				};
			};
			case 400: {
				private _doArm = (["param1", 0] call _number) >= 1;
				_uav engineOn _doArm;
				_uav setVariable ["armatak_uas_armed", _doArm, true];
				if (_doArm) then {
					_uav setFuel ((fuel _uav) max 0.1);
				};
				systemChat format ["ATAK %1 %2", ["DISARM", "ARM"] select _doArm, _callsign];
				"armatak" callExtension ["log", [["info", format ["Applied MAVLINK ARM=%1 to UAV %2", _doArm, _uav]]]];
			};
			case 22: {
				private _alt = (["param7", 75] call _number) max 10;
				private _pos = getPosATL _uav;
				_pos set [2, _alt];
				[_uav, _pos, "MOVE", 80, 25] call _commandMove;
				_uav setVelocityModelSpace [0, 15, 8];
				systemChat format ["ATAK TAKEOFF %1m %2", round _alt, _callsign];
			};
			case 21: {
				[9] call _applyMode;
			};
			case 20: {
				[6] call _applyMode;
			};
			case 16: {
				private _lat = ["param5", 0] call _number;
				private _lon = ["param6", 0] call _number;
				private _alt = ["param7", -1] call _number;
				private _pos = [_uav, _lat, _lon, _alt] call _geoToAtl;
				[_uav, _pos, "MOVE", 80, 50] call _commandMove;
				systemChat format ["ATAK MOVE %1", _callsign];
			};
			case 17: {
				private _lat = ["param5", 0] call _number;
				private _lon = ["param6", 0] call _number;
				private _alt = ["param7", -1] call _number;
				private _radius = abs (["param3", 80] call _number);
				private _pos = [_uav, _lat, _lon, _alt] call _geoToAtl;
				[_uav, _pos, "LOITER", _radius, 30] call _commandMove;
				systemChat format ["ATAK LOITER %1", _callsign];
			};
			case 43000: {
				private _speed = ["param2", -1] call _number;
				if (_speed <= 0) then {
					_speed = ["param1", -1] call _number;
				};
				[_speed] call _applySpeed;
			};
			case 43001: {
				private _alt = ["param1", -1] call _number;
				if (_alt < 0) then {
					_alt = ["param7", -1] call _number;
				};
				private _pos = getPosATL _uav;
				_pos set [2, _alt max 10];
				[_uav, _pos, "MOVE", 80, 25] call _commandMove;
				systemChat format ["ATAK ALT %1m %2", round (_pos select 2), _callsign];
			};
			case 43002: {
				private _heading = ["param1", -1] call _number;
				if (_heading >= 0) then {
					_uav setDir _heading;
					systemChat format ["ATAK HDG %1 %2", round _heading, _callsign];
				};
			};
			default {
				"armatak" callExtension ["log", [["info", format ["Unhandled MAVLINK COMMAND_LONG %1 (%2): %3", _command, _commandName, _data]]]];
			};
		};
	};
	case "COMMAND_INT": {
		private _lat = (["x", 0] call _number) / 1e7;
		private _lon = (["y", 0] call _number) / 1e7;
		private _alt = ["z", -1] call _number;

		switch (_command) do {
			case 16: {
				private _pos = [_uav, _lat, _lon, _alt] call _geoToAtl;
				[_uav, _pos, "MOVE", 80, 50] call _commandMove;
				systemChat format ["ATAK MOVE %1", _callsign];
			};
			case 17;
			case 192: {
				private _radius = abs (["param3", 80] call _number);
				private _direction = ["CIRCLE_L", "CIRCLE_R"] select ((["param4", 0] call _number) < 0);
				private _pos = [_uav, _lat, _lon, _alt] call _geoToAtl;
				private _type = ["MOVE", "LOITER"] select (_radius > 1);
				[_uav, _pos, _type, _radius, 30] call _commandMove;
				if (_type == "LOITER") then {
					private _group = [_uav] call _uavGroup;
					private _waypoints = waypoints _group;
					if (_waypoints isNotEqualTo []) then {
						(_waypoints select -1) setWaypointLoiterType _direction;
					};
				};
				systemChat format ["ATAK %1 %2", _type, _callsign];
			};
			default {
				"armatak" callExtension ["log", [["info", format ["Unhandled MAVLINK COMMAND_INT %1 (%2): %3", _command, _commandName, _data]]]];
			};
		};
	};
	case "MISSION_COUNT": {
		private _count = ["count", 0] call _number;
		[] call _clearUavRoute;
		systemChat format ["ATAK ROUTE %1 pts %2", round _count, _callsign];
		"armatak" callExtension ["log", [["info", format ["Receiving MAVLINK mission count=%1 for UAV %2", _count, _uav]]]];
	};
	case "MISSION_CLEAR_ALL": {
		[] call _clearUavRoute;
		systemChat format ["ATAK ROUTE CLEAR %1", _callsign];
	};
	case "MISSION_SET_CURRENT": {
		private _seq = ["seq", 0] call _number;
		"armatak" callExtension ["log", [["info", format ["MAVLINK mission set current seq=%1 for UAV %2", _seq, _uav]]]];
	};
	case "MISSION_ITEM";
	case "MISSION_ITEM_INT": {
		private _seq = ["seq", 0] call _number;
		private _missionCommand = ["command", -1] call _number;
		private _lat = ["lat", 0] call _number;
		private _lon = ["lon", 0] call _number;
		private _alt = ["alt", -1] call _number;

		if (_lat == 0 && {_lon == 0}) exitWith {
			"armatak" callExtension ["log", [["warn", format ["Ignoring MAVLINK mission item at zero coordinate: %1", _data]]]];
		};

		private _pos = [_uav, _lat, _lon, _alt] call _geoToAtl;
		private _radius = abs (["param3", 80] call _number);
		[_uav, _pos, _missionCommand, _seq, _radius] call _appendMissionWaypoint;
		_uav engineOn true;
		_uav setVariable ["armatak_uas_armed", true, true];
		_uav setFuel ((fuel _uav) max 0.1);
		_uav flyInHeight ((_pos select 2) max 10);
		systemChat format ["ATAK ROUTE WP %1 %2", round _seq, _callsign];
		"armatak" callExtension ["log", [["info", format ["Added MAVLINK mission item seq=%1 command=%2 posATL=%3 for UAV %4", _seq, _missionCommand, _pos, _uav]]]];
	};
	case "SET_HOME_POSITION": {
		[["lat", 0] call _number, ["lon", 0] call _number, ["alt", 0] call _number] call _setHomeFromGeo;
	};
	case "SET_POSITION_TARGET_GLOBAL_INT": {
		private _lat = ["lat", 0] call _number;
		private _lon = ["lon", 0] call _number;
		private _alt = ["alt", -1] call _number;
		private _pos = [_uav, _lat, _lon, _alt] call _geoToAtl;
		[_uav, _pos, "MOVE", 80, 40] call _commandMove;
		systemChat format ["ATAK GUIDED MOVE %1", _callsign];
	};
	case "SET_MODE": {
		private _mode = ["custom_mode", -1] call _number;
		[_mode] call _applyMode;
	};
	case "COMMAND_ACK": {
		"armatak" callExtension ["log", [["info", format ["Received MAVLINK COMMAND_ACK %1 (%2): %3", _command, _commandName, _data]]]];
	};
	case "MANUAL_CONTROL": {
		"armatak" callExtension ["log", [["info", format ["Received MAVLINK MANUAL_CONTROL: %1", _data]]]];
	};
	default {
		"armatak" callExtension ["log", [["info", format ["Unhandled MAVLINK UDP callback %1: %2", _function, _data]]]];
	};
};
