// function name: armatak_fnc_draw_rectangle
// function author: Valmo
// function description: Sends an ATAK Drawing Tools rectangle CoT from an Arma center, width, length, and bearing.
//
// Arguments:
// 0: Center position or object <ARRAY|OBJECT>
// 1: Width in meters <NUMBER>
// 2: Length in meters <NUMBER>
// 3: Bearing in degrees <NUMBER> (default: 0)
// 4: Callsign/title <STRING> (default: "ArmaTAK Rectangle")
// 5: Stale time in seconds <NUMBER> (default: 86400)
// 6: Stroke color as signed ARGB int <NUMBER> (default: -1)
// 7: Fill color as signed ARGB int <NUMBER> (default: -1761607681)
// 8: Stroke weight <NUMBER> (default: 3)
// 9: MilSym SIDC for tactical overlay <STRING> (default: "")
//
// Example:
// [screenToWorld [0.5, 0.5], 200, 500, 30, "Engagement Area"] call armatak_fnc_draw_rectangle;
//
// Public: Yes

params [
	["_center", objNull, [objNull, []]],
	["_width", 100, [0]],
	["_length", 100, [0]],
	["_bearing", 0, [0]],
	["_callsign", "ArmaTAK Rectangle", [""]],
	["_staleSeconds", 86400, [0]],
	["_strokeColor", -1, [0]],
	["_fillColor", -1761607681, [0]],
	["_strokeWeight", 3, [0]],
	["_milsym", "", [""]]
];

private _centerPos = if (_center isEqualType objNull) then {
	getPos _center
} else {
	_center
};

if ((count _centerPos) < 2) exitWith {""};

private _altitude = _centerPos param [2, 0, [0]];
private _halfWidth = (_width max 1) / 2;
private _halfLength = (_length max 1) / 2;
private _sin = sin _bearing;
private _cos = cos _bearing;
private _forward = [_sin, _cos, 0];
private _right = [_cos, -_sin, 0];
private _offsets = [
	[(_right vectorMultiply -_halfWidth), (_forward vectorMultiply _halfLength)],
	[(_right vectorMultiply _halfWidth), (_forward vectorMultiply _halfLength)],
	[(_right vectorMultiply _halfWidth), (_forward vectorMultiply -_halfLength)],
	[(_right vectorMultiply -_halfWidth), (_forward vectorMultiply -_halfLength)]
];
private _points = [];

{
	private _offset = (_x select 0) vectorAdd (_x select 1);
	_points pushBack ([_centerPos select 0, _centerPos select 1, _altitude] vectorAdd _offset);
} forEach _offsets;

private _centerReal = [_centerPos select 0, _centerPos select 1, _altitude] call armatak_client_fnc_convertClientLocation;
private _pointStrings = [];

{
	private _realLocation = [_x select 0, _x select 1, _x select 2] call armatak_client_fnc_convertClientLocation;
	_pointStrings pushBack format ["%1,%2,%3", _realLocation select 0, _realLocation select 1, _realLocation select 2];
} forEach _points;

private _uuid = "armatak" callExtension ["uuid", []] select 0;
private _payload = [
	_uuid,
	"u-d-r",
	_centerReal select 0,
	_centerReal select 1,
	_centerReal select 2,
	_pointStrings joinString ";",
	_callsign,
	_staleSeconds max 1,
	_strokeColor,
	_fillColor,
	_strokeWeight max 1,
	"solid",
	false,
	_milsym
];

"armatak" callExtension ["tcp_socket:draw:rectangle", [_payload]];
