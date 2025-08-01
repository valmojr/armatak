params["_latitude", "_longitude", "_altitude"];

_playerPosition = [_latitude, _longitude, _altitude];

_playerLatitude = _playerPosition select 0;
_playerLongitude = _playerPosition select 1;

_playerMaxLongitude = 6340;
_playerMaxLatitude = 6340;

_MapMaxLongitude = 35.042822;
_MapMinLongitude = 34.914006;

_MapMaxLatitude = 36.268269;
_MapMinLatitude = 36.111253;

_LongitudeDifference = _MapMaxLongitude - _MapMinLongitude;
_LatitudeDifference = _MapMaxLatitude - _MapMinLatitude;

_RealLongitude = (_playerLongitude / _playerMaxLongitude) * _LongitudeDifference + _MapMinLongitude;
_RealLatitude = (_playerLatitude / _playerMaxLatitude) * _LatitudeDifference + _MapMinLatitude;

[_RealLongitude, _RealLatitude, _playerPosition select 2]
