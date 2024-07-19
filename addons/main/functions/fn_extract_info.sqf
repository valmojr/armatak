params["_unit"];

// ATAK UTIL INFOS
private _atak_uid = getPlayerUID _unit;			// ATAK Get Player UID
private _atak_callsign = name _unit;			// ATAK Display Name
private _atak_team_color = "team_color"; 		// ATAK Team Colors -> White, Yellow, Orange, Magenta, Red, Maroon, Purple, Dark Blue, Blue, Cyan, Teal, Green, Dark Green, Brown || Change it on every squad? make it customizable?
private _atak_role = "role";					// ATAK Roles -> Team Member, Team Lead, HQ, Sniper, Medic, Forward Observer, RTO, K9
private _atak_display_type = "display_type";	// ATAK Display Types -> Ground Troop, Armored Vehicle, Civilian Vehicle, Generic Air Unit, Generic Ground Unit, Generic Sea Surface Unit
private _atak_location = worldName;				// ATAK Location in Real World -> Already stored in a database or array
private _atak_position = getPos _unit;			// ATAK Position -> Converted Arma 3 XYZ Coords to Geographic Coords

_unit_info = [_atak_uid, _atak_callsign, _atak_team_color, _atak_role, _atak_display_type, _atak_location, _atak_position];

_unit_info