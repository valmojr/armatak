// function name: armatak_fnc_report_marker
// function author: Valmo
// function description: Sends a one-shot TAK report marker with an independent stale time.
//
// Arguments:
// 0: Source position or object <ARRAY|OBJECT>
// 1: Affiliation or raw CoT type <STRING> (default: "unknown")
//    Supported affiliations: "friendly", "enemy", "hostile", "neutral", "unknown"
// 2: Marker kind <STRING> (default: "infantry")
//    Supported kinds: "infantry", "tank", "car", "apc", "helicopter", "plane", "ship", "static"
// 3: Callsign <STRING> (default: "Report")
// 4: Remarks <STRING> (default: "")
// 5: Stale time in seconds <NUMBER> (default: 3600)
//
// Example:
// [cursorObject, "enemy", "tank", "Enemy Tank", "Reported enemy tank"] call armatak_fnc_report_marker;
// [screenToWorld [0.5, 0.5], "unknown", "infantry", "Unknown Contact", "Unknown contact reported", 7200] call armatak_fnc_report_marker;
// [cursorObject, "a-h-G-U-C-A-T", "Enemy Tank", "Reported enemy tank", 7200] call armatak_fnc_report_marker;
//
// Public: Yes

params [
	["_source", objNull, [objNull, []]],
	["_affiliationOrType", "unknown", [""]],
	["_kindOrCallsign", "infantry", [""]],
	["_callsignOrRemarks", "Report", [""]],
	["_remarksOrStaleSeconds", "", ["", 0]],
	["_staleSeconds", 3600, [0]]
];

private _type = "";
private _callsign = _callsignOrRemarks;
private _remarks = _remarksOrStaleSeconds;

if ((_affiliationOrType select [0, 2]) isEqualTo "a-") then {
	_type = _affiliationOrType;
	_callsign = _kindOrCallsign;
	_remarks = _callsignOrRemarks;

	if (_remarksOrStaleSeconds isEqualType 0) then {
		_staleSeconds = _remarksOrStaleSeconds;
	};
} else {
	private _affiliation = switch (toLower _affiliationOrType) do {
		case "friendly": {"f"};
		case "enemy": {"h"};
		case "hostile": {"h"};
		case "neutral": {"n"};
		default {"u"};
	};

	private _kind = switch (toLower _kindOrCallsign) do {
		case "tank": {"G-U-C-A-T"};
		case "car": {"G-U-C-I-M"};
		case "apc": {"G-U-C-I-I"};
		case "helicopter": {"A-M-H"};
		case "plane": {"A-M-F"};
		case "ship": {"S"};
		case "static": {"G-U-C-F-M"};
		default {"G-U-C-I"};
	};

	_type = "a-" + _affiliation + "-" + _kind;

	if (_remarksOrStaleSeconds isEqualType 0) then {
		_staleSeconds = _remarksOrStaleSeconds;
		_remarks = "";
	};
};

private _position = if (_source isEqualType objNull) then {
	getPos _source
} else {
	_source
};

if ((count _position) < 2) exitWith {
	""
};

private _altitude = _position param [2, 0, [0]];
private _realLocation = [_position select 0, _position select 1, _altitude] call armatak_client_fnc_convertClientLocation;
private _uuid = "armatak" callExtension ["uuid", []] select 0;
private _payload = [
	_uuid,
	_type,
	_realLocation select 0,
	_realLocation select 1,
	_realLocation select 2,
	_callsign,
	_staleSeconds max 1,
	_remarks
];

"armatak" callExtension ["tcp_socket:cot:report_marker", [_payload]];
