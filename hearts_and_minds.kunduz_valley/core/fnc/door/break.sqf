
/* ----------------------------------------------------------------------------
Function: btc_door_fnc_break

Description:
    Break locked door action.

Parameters:

Returns:

Examples:
    (begin example)
        [] call btc_door_fnc_break;
    (end)

Author:
    Vdauphin

---------------------------------------------------------------------------- */

([2] call ace_interaction_fnc_getDoor) params ["_house", "_door"];
if (_door isEqualTo "") exitWith {
    (localize "STR_BTC_HAM_O_DOOR_NO") call CBA_fnc_notify;
};

[btc_door_breaking_time, [_house, _door, player, 0.2], {
    params ["_args"];
    playSound3D ["\z\ace\addons\logistics_wirecutter\sound\wirecut.ogg", player];
    _args call btc_door_fnc_broke;
}, {}, localize "STR_BTC_HAM_O_DOOR_BREAKING", {true}, ["isNotInside"]] call ace_common_fnc_progressBar;
