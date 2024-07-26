if ((!isServer) && (player != player)) then {
	waitUntil {
		player == player
	};
};

if (isDedicated) exitWith {};

if (player != player) then {
	waitUntil {
		player == player
	};
};
 
{
	[{if (_x getVariable "_atak_uid" == "") then {
		[_x] call armatak_fnc_postGeoObject;
	} else {
		[_x] call armatak_fnc_putGeoObject;
	}},0,[]] call CBA_fnc_addPerFrameHandler;
} forEach playableUnits;
