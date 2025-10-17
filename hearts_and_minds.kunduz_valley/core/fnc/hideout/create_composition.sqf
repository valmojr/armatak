
/* ----------------------------------------------------------------------------
Function: btc_hideout_fnc_create_composition

Description:
    Create a random hideout composition.

Parameters:
    _pos - Position. [Array]

Returns:
    _result - The flag. [Object]

Examples:
    (begin example)
        _result = [getPos (allPlayers#0)] call btc_hideout_fnc_create_composition;
    (end)

Author:
    Giallustio

---------------------------------------------------------------------------- */

params [
    ["_pos", [0, 0, 0], [[]]]
];

private _type_bigbox = selectRandom ["Box_FIA_Ammo_F", "C_supplyCrate_F", "Box_East_AmmoVeh_F"];
private _power = selectRandom btc_type_power;

private _composition_hideout = [
    [selectRandom btc_type_campfire,0,[-2.30957,-1.02979,0]],
    [selectRandom btc_type_sleepingbag,135.477,[0.758789,-3.91309,0]],
    [selectRandom btc_type_seat,279.689,[-4.52783,-0.76416,0]]
];

private _composition = [_pos, random 360, _composition_hideout] call btc_fnc_create_composition;

_composition select ((_composition apply {typeOf _x}) find _type_bigbox);
