params["_unit"];
private _callsign = "";
if (roleDescription _unit != "") then {
	_callsign = name _unit + " | " + roleDescription _unit;
} else {
	_callsign = name _unit;
};
_callsign