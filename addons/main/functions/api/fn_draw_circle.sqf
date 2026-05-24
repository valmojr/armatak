// function name: armatak_fnc_draw_circle
// function author: Valmo
// function description: Sends an ATAK Drawing Tools circle CoT.
//
// Arguments:
// 0: Center position or object <ARRAY|OBJECT>
// 1: Radius in meters <NUMBER>
// 2: Callsign/title <STRING> (default: "ArmaTAK Circle")
// 3: Stale time in seconds <NUMBER> (default: 86400)
// 4: Stroke color as signed ARGB int <NUMBER> (default: -1)
// 5: Fill color as signed ARGB int <NUMBER> (default: -1761607681)
// 6: Stroke weight <NUMBER> (default: 3)
// 7: Optional registration scope <STRING> (default: "")
//
// Example:
// [player, 300, "Mortar Risk Area"] call armatak_fnc_draw_circle;
//
// Public: Yes

params [
	["_center", objNull, [objNull, []]],
	["_radius", 100, [0]],
	["_callsign", "ArmaTAK Circle", [""]],
	["_staleSeconds", 86400, [0]],
	["_strokeColor", -1, [0]],
	["_fillColor", -1761607681, [0]],
	["_strokeWeight", 3, [0]],
	["_scope", "", [""]]
];

[
	_center,
	_radius,
	_radius,
	360,
	_callsign,
	_staleSeconds,
	_strokeColor,
	_fillColor,
	_strokeWeight,
	"",
	"u-d-c-c",
	_scope
] call armatak_fnc_draw_ellipse;
