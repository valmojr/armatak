#include "..\script_component.hpp"

/*
	 * Author: Valmo Trindade
	 * This function is used to convert the position of a unit to the world world location.
	 *
	 * Argument:
	 * 0: in game latitude <NUMBER> is the latitude of the unit.
	 * 1: in game longitude <NUMBER> is the longitude of the unit.
	 * 2: in game altitude <NUMBER> is the altitude of the unit.
	 * 3: in game bearing <NUMBER> is the bearing of the unit.
	 *
	 * Return Value:
	 * ARRAY -> [latitude, longitude, altitude, bearing]
	 *
	 * Example:
	 * [player] call armatak_client_fnc_convertClientLocation;
	 *
	 * Public: Yes
*/

params["_latitude", "_longitude", "_altitude"];

_position = [_latitude, _longitude, _altitude];

_realLocation = null;

switch (toLower worldName) do {
	case "altis": {
		_realLocation = _position call armatak_fnc_convert_to_altis;
	};
	case "stratis": {
		_realLocation = _position call armatak_fnc_convert_to_stratis;
	};
	case "malden": {
		_realLocation = _position call armatak_fnc_convert_to_malden;
	};
	case "vr": {
		_realLocation = _position call armatak_fnc_convert_to_vr;
	};
	case "lawn": {
		_realLocation = _position call armatak_fnc_convert_to_lawn;
	};
	case "cucui": {
		_realLocation = _position call armatak_fnc_convert_to_cucui;
	};
	case "chernarus": {
		_realLocation = _position call armatak_fnc_convert_to_chernarus;
	};
	case "chernarus_summer": {
		_realLocation = _position call armatak_fnc_convert_to_chernarus;
	};
	case "chernarus_winter": {
		_realLocation = _position call armatak_fnc_convert_to_chernarus;
	};
	case "bootcamp_acr": {
		_realLocation = _position call armatak_fnc_convert_to_bukovina;
	};
	case "woodland_acr": {
		_realLocation = _position call armatak_fnc_convert_to_bystrika;
	};
	case "mountains_acr": {
		_realLocation = _position call armatak_fnc_convert_to_takistan_montains;
	};
	case "sara_dbe1": {
		_realLocation = _position call armatak_fnc_convert_to_united_sahrani;
	};
	case "saralite": {
		_realLocation = _position call armatak_fnc_convert_to_southen_sahrani;
	};
	case "enoch": {
		_realLocation = _position call armatak_fnc_convert_to_livonia;
	};
	case "kunduz": {
		_realLocation = _position call armatak_fnc_convert_to_kunduz;
	};
	case "kunduz_valley": {
		_realLocation = _position call armatak_fnc_convert_to_kunduz_valley;
	};
	case "malvinasfalkands": {
		_realLocation = _position call armatak_fnc_convert_to_malvinas_malvinasfalkands;
	};
	case "pebble_island_airfield": {
		_realLocation = _position call armatak_fnc_convert_to_malvinas_pebble_island_airfield;
	};
	case "p_argentino_stanley": {
		_realLocation = _position call armatak_fnc_convert_to_malvinas_p_argentino_stanley;
	};
	case "top_malo_house": {
		_realLocation = _position call armatak_fnc_convert_to_malvinas_top_malo_house;
	};
	case "pradera_ganso": {
		_realLocation = _position call armatak_fnc_convert_to_malvinas_pradera_ganso;
	};
	case "tanoa": {
		_realLocation = _position call armatak_fnc_convert_to_tanoa;
	};
	case "zagor_zagorsk_reserved_forest": {
		_realLocation = _position call armatak_fnc_convert_to_zagor_zagorsk_reserved_forest;
	};
	case "umb_colombia": {
		_realLocation = _position call armatak_fnc_convert_to_colombia;
	};
	case "clafghan": {
		_realLocation = _position call armatak_fnc_convert_to_clafghan;
	};
	default {
		_warning = format ["<t color='#FF8021'>ARMATAK</t><br/> %1", "Unsupported Map"];
		[[_warning, 1.5]] call CBA_fnc_notify;

		_realLocation = [0, 0, 0];
	};
};

_realLocation
