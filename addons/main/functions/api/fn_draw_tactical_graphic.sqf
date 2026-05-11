// function name: armatak_fnc_draw_tactical_graphic
// function author: Valmo
// function description: Sends an ATAK Drawing Tools shape with a MilSym tactical graphic overlay.
//
// Arguments:
// 0: Positions or objects <ARRAY>
// 1: MilSym SIDC <STRING>
// 2: Callsign/title <STRING> (default: "ArmaTAK Tactical Graphic")
// 3: Closed polygon <BOOL> (default: false)
// 4: Stale time in seconds <NUMBER> (default: 86400)
// 5: Stroke color as signed ARGB int <NUMBER> (default: -1)
// 6: Fill color as signed ARGB int <NUMBER> (default: -1761607681)
// 7: Stroke weight <NUMBER> (default: 3)
//
// Example:
// [[pos player, screenToWorld [0.5, 0.5]], "GFGPOLAGM-----X", "Axis of Advance"] call armatak_fnc_draw_tactical_graphic;
//
// Public: Yes

params [
	["_points", [], [[]]],
	["_milsym", "", [""]],
	["_callsign", "ArmaTAK Tactical Graphic", [""]],
	["_closed", false, [true]],
	["_staleSeconds", 86400, [0]],
	["_strokeColor", -1, [0]],
	["_fillColor", -1761607681, [0]],
	["_strokeWeight", 3, [0]]
];

if (_milsym isEqualTo "") exitWith {""};
if ((count _points) < 2) exitWith {""};

private _pointStrings = [];
private _center = [];

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

		if (_center isEqualTo []) then {
			_center = _realLocation;
		};
	};
} forEach _points;

if ((count _pointStrings) < 2) exitWith {""};

private _uuid = "armatak" callExtension ["uuid", []] select 0;
private _payload = [
	_uuid,
	"u-d-f",
	_center select 0,
	_center select 1,
	_center select 2,
	_pointStrings joinString ";",
	_callsign,
	_staleSeconds max 1,
	_strokeColor,
	_fillColor,
	_strokeWeight max 1,
	"solid",
	_closed,
	_milsym
];

"armatak" callExtension ["tcp_socket:draw:vector", [_payload]];
