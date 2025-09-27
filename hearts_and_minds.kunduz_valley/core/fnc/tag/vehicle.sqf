
/* ----------------------------------------------------------------------------
Function: btc_tag_fnc_vehicle

Description:
    Create random tag with stencil on a vehicle.

Parameters:
    _vehicle - Vehicle to apply the tag to. [Object]

Returns:

Examples:
    (begin example)
        [cursorObject] call btc_tag_fnc_vehicle;
    (end)

Author:
    Vdauphin

---------------------------------------------------------------------------- */

params [
    ["_vehicle", objNull, [objNull]]
];

private _sentences = localize selectRandom btc_type_tags_sentences;
private _count =  selectMax ((_sentences splitString " ") apply {count _x});  
private _size = 0.35;   
if (_count > 8) then {  
    _size = 0.25;  
};  
if (_count < 5) then {  
    _size = 0.45;  
}; 
[_vehicle, _sentences, _size, selectRandom ["f7e9e1f8", "ba2619", "8fce00", "25310a", "87CEEB"]] call ace_tagging_fnc_stencilVehicle; 
