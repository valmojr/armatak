// function name: armatak_fnc_draw_ellipse
// function author: Valmo
// function description: Sends an ATAK Drawing Tools ellipse or circle CoT.
//
// Arguments:
// 0: Center position or object <ARRAY|OBJECT>
// 1: Major radius in meters <NUMBER>
// 2: Minor radius in meters <NUMBER>
// 3: Rotation angle in degrees <NUMBER> (default: 0)
// 4: Callsign/title <STRING> (default: "ArmaTAK Ellipse")
// 5: Stale time in seconds <NUMBER> (default: 86400)
// 6: Stroke color as signed ARGB int <NUMBER> (default: -1)
// 7: Fill color as signed ARGB int <NUMBER> (default: -1761607681)
// 8: Stroke weight <NUMBER> (default: 3)
// 9: MilSym SIDC for tactical overlay <STRING> (default: "")
// 10: CoT type <STRING> (default: "u-d-c-e")
//
// Example:
// [screenToWorld [0.5, 0.5], 250, 100, 45, "Support by Fire"] call armatak_fnc_draw_ellipse;
//
// Public: Yes

params [
	["_center", objNull, [objNull, []]],
	["_major", 100, [0]],
	["_minor", 50, [0]],
	["_angle", 0, [0]],
	["_callsign", "ArmaTAK Ellipse", [""]],
	["_staleSeconds", 86400, [0]],
	["_strokeColor", -1, [0]],
	["_fillColor", -1761607681, [0]],
	["_strokeWeight", 3, [0]],
	["_milsym", "", [""]],
	["_cotType", "u-d-c-e", [""]]
];

private _position = if (_center isEqualType objNull) then {
	getPos _center
} else {
	_center
};

if ((count _position) < 2) exitWith {""};

private _altitude = _position param [2, 0, [0]];
private _realLocation = [_position select 0, _position select 1, _altitude] call armatak_client_fnc_convertClientLocation;
private _uuid = "armatak" callExtension ["uuid", []] select 0;
private _payload = [
	_uuid,
	_cotType,
	_realLocation select 0,
	_realLocation select 1,
	_realLocation select 2,
	_major max 1,
	_minor max 1,
	_angle,
	_callsign,
	_staleSeconds max 1,
	_strokeColor,
	_fillColor,
	_strokeWeight max 1,
	_milsym
];

"armatak" callExtension ["tcp_socket:draw:ellipse", [_payload]];
