params["_latitude", "_longitude", "_altitude"];

_playerPosition = [_latitude, _longitude, _altitude];

_playerLatitude = _playerPosition select 0;
_playerLongitude = _playerPosition select 1;

_playerMaxLongitude = 8190;
_playerMaxLatitude = 8190;

_MapMaxLatitude = 25.080683;
_MapMinLatitude = 24.940569;

_MapMaxLongitude = 39.569794;
_MapMinLongitude = 39.456881;

_LongitudeDifference = _MapMaxLongitude - _MapMinLongitude;
_LatitudeDifference = _MapMaxLatitude - _MapMinLatitude;

_RealLongitude = (_playerLongitude / _playerMaxLongitude) * _LongitudeDifference + _MapMinLongitude;
_RealLatitude = (_playerLatitude / _playerMaxLatitude) * _LatitudeDifference + _MapMinLatitude;

[_RealLongitude, _RealLatitude, _playerPosition select 2]