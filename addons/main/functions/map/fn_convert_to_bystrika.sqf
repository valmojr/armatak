params ["_longitudeInGame", "_latitudeInGame", "_altitude"];

private _mapWidth = 7680;
private _mapHeight = 7680;

//    SW corner (used as origin)
private _SW_lat = 50.652803;
private _SW_lon = 14.092823;

//    SE corner
private _SE_lat = 50.662156;
private _SE_lon = 14.200170;

//    NW corner
private _NW_lat = 50.721832;
private _NW_lon = 14.077843;

private _edgeSE_lat = _SE_lat - _SW_lat;
private _edgeSE_lon = _SE_lon - _SW_lon;

private _edgeNW_lat = _NW_lat - _SW_lat;
private _edgeNW_lon = _NW_lon - _SW_lon;

private _fx = _longitudeInGame / _mapWidth;
private _fy = _latitudeInGame / _mapHeight;

private _realLat = _SW_lat + (_fx * _edgeSE_lat) + (_fy * _edgeNW_lat);
private _realLon = _SW_lon + (_fx * _edgeSE_lon) + (_fy * _edgeNW_lon);

[_realLat, _realLon, _altitude]
