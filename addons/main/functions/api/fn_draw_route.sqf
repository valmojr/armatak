// function name: armatak_fnc_draw_route
// function author: Valmo
// function description: Sends an ATAK navigable route CoT.
//
// Arguments:
// 0: Positions or objects <ARRAY>
// 1: Callsign/title <STRING> (default: "ArmaTAK Route")
// 2: Stale time in seconds <NUMBER> (default: 86400)
// 3: Color as signed ARGB int <NUMBER> (default: -1)
// 4: Stroke weight <NUMBER> (default: 3)
// 5: Navigation method <STRING> (default: "Driving")
// 6: Route type <STRING> (default: "Primary")
// 7: Direction/planning method <STRING> (default: "Infil")
// 8: Checkpoint interval among route points <NUMBER> (default: 5)
// 9: Optional registration scope <STRING> (default: "")
//
// Example:
// [[pos player, screenToWorld [0.5, 0.5]], "Patrol Route"] call armatak_fnc_draw_route;
//
// Public: Yes

params [
	["_points", [], [[]]],
	["_callsign", "ArmaTAK Route", [""]],
	["_staleSeconds", 86400, [0]],
	["_color", -1, [0]],
	["_strokeWeight", 3, [0]],
	["_method", "Driving", [""]],
	["_routeType", "Primary", [""]],
	["_direction", "Infil", [""]],
	["_checkpointInterval", 5, [0]],
	["_scope", "", [""]]
];

if ((count _points) < 2) exitWith {""};

private _pointStrings = [];

{
	private _position = if (_x isEqualType objNull) then {
		getPos _x
	} else {
		_x
	};

	if ((count _position) >= 2) then {
		private _altitude = _position param [2, 0, [0]];
		private _realLocation = [_position select 0, _position select 1, _altitude] call armatak_client_fnc_convertClientLocation;
		_pointStrings pushBack format ["%1,%2,%3", _realLocation select 0, _realLocation select 1, _realLocation select 2];
	};
} forEach _points;

if ((count _pointStrings) < 2) exitWith {""};

private _uuid = "armatak" callExtension ["uuid", []] select 0;
private _pad = {
	params ["_value", "_digits"];

	private _text = str (floor _value);
	while {(count _text) < _digits} do {
		_text = "0" + _text;
	};

	_text
};

private _nowDate = systemTimeUTC;
private _year = _nowDate param [0, 1970, [0]];
private _month = _nowDate param [1, 1, [0]];
private _day = _nowDate param [2, 1, [0]];
private _hour = _nowDate param [3, 0, [0]];
private _minute = _nowDate param [4, 0, [0]];
private _second = _nowDate param [5, 0, [0]];
private _millisecond = _nowDate param [6, 0, [0]];
private _now = format [
	"%1-%2-%3T%4:%5:%6.%7Z",
	_year,
	[_month, 2] call _pad,
	[_day, 2] call _pad,
	[_hour, 2] call _pad,
	[_minute, 2] call _pad,
	[_second, 2] call _pad,
	[_millisecond, 3] call _pad
];
private _stale = "9999-12-31T23:59:59.999Z";

private _links = "";
private _routeWaypoints = [];
private _lastIndex = (count _pointStrings) - 1;
private _checkpointNumber = 1;
private _checkpointIntervalSafe = _checkpointInterval max 1;

{
	private _linkUuid = "armatak" callExtension ["uuid", []] select 0;
	private _pointCallsign = "";
	if (_forEachIndex isEqualTo 0) then {
		_pointCallsign = format ["%1 SP", _callsign];
	} else {
		if (_forEachIndex isEqualTo _lastIndex) then {
			_pointCallsign = "VDO";
		} else {
			if ((_forEachIndex mod _checkpointIntervalSafe) isEqualTo 0) then {
				_pointCallsign = format ["CP%1", _checkpointNumber];
				_checkpointNumber = _checkpointNumber + 1;
			};
		};
	};

	private _linkType = ["b-m-p-w", "b-m-p-c"] select (_pointCallsign isEqualTo "");
	_links = _links + format [
		"<link uid=""%1"" callsign=""%2"" type=""%3"" point=""%4"" remarks="""" relation=""c""/>",
		_linkUuid,
		_pointCallsign,
		_linkType,
		_x
	];

	if (_pointCallsign isNotEqualTo "") then {
		_routeWaypoints pushBack [_linkUuid, _pointCallsign, _x];
	};
} forEach _pointStrings;

private _xml = format [
	"<?xml version=""1.0"" encoding=""UTF-8"" ?><event version=""2.0"" uid=""%1"" type=""b-m-r"" time=""%2"" start=""%2"" stale=""%3"" how=""h-e"" access=""Undefined""><point lat=""0.0"" lon=""0.0"" hae=""9999999.0"" ce=""9999999.0"" le=""9999999.0""/><detail>%4<link_attr planningmethod=""%5"" color=""%6"" method=""%7"" prefix=""CP"" style=""0"" type=""Vehicle"" stroke=""%8"" direction=""%5"" routetype=""%9"" order=""Ascending Check Points""/><creator uid=""ARMATAK"" callsign=""ArmaTAK"" time=""%2"" type=""a-f-G-U-C""/><strokeColor value=""%6""/><strokeWeight value=""%8""/><strokeStyle value=""solid""/><labels_on value=""false""/><__routeinfo><__navcues/></__routeinfo><color value=""%6""/><remarks/><contact callsign=""%10""/><archive/><height_unit>1</height_unit></detail></event>",
	_uuid,
	_now,
	_stale,
	_links,
	_direction,
	_color,
	_method,
	_strokeWeight max 1,
	_routeType,
	_callsign
];

"armatak" callExtension ["log", [["info", format ["Sending ATAK route '%1' with %2 points (%3 bytes)", _callsign, count _pointStrings, count _xml]]]];
"armatak" callExtension ["tcp_socket:send_payload", [_xml]];
[_scope, _uuid, "b-m-r", 0, 0, 9999999] call armatak_fnc_register_cot;

{
	_x params ["_waypointUid", "_waypointCallsign", "_pointString"];
	private _pointParts = _pointString splitString ",";
	if ((count _pointParts) >= 3) then {
		private _waypointXml = format [
			"<?xml version=""1.0"" encoding=""UTF-8"" ?><event version=""2.0"" uid=""%1"" type=""b-m-p-w"" time=""%2"" start=""%2"" stale=""%3"" how=""h-e""><point lat=""%4"" lon=""%5"" hae=""%6"" ce=""9999999.0"" le=""9999999.0""/><detail><contact callsign=""%7""/><remarks/><archive/><link relation=""p-p"" type=""b-m-r"" uid=""%8""/></detail></event>",
			_waypointUid,
			_now,
			_stale,
			_pointParts select 0,
			_pointParts select 1,
			_pointParts select 2,
			_waypointCallsign,
			_uuid
		];
		"armatak" callExtension ["tcp_socket:send_payload", [_waypointXml]];
		[_scope, _waypointUid, "b-m-p-w", parseNumber (_pointParts select 0), parseNumber (_pointParts select 1), parseNumber (_pointParts select 2)] call armatak_fnc_register_cot;
	};
} forEach _routeWaypoints;

_uuid
