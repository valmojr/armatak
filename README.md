# ARMATAK

![ARMATAK FRAME LOGO](./picture.png)

## Project Concept

ARMATAK is a Arma 3 addons for sending the player's location to WebSocket connection for SIMTAK, a self made application for mocking the EUD's location with provided data from the WebSocket. ARMATAK is also able to stream selected unit positions to TAK Clients in sessions on real locations maps. It basically sends HTTP requests to a OpenTAKServer API instance or FreeTAKServer from your Game Server, converting the useful information as HTTP Requests right into the OpenTAKServer/FreeTAKServer REST API as TAK entities soo the players can connect their WinTAK/ATAK/iTAK and game using the TAK features as a Blue Force Tracker or mission planning tool for Arma 3.

## Disclaimers

1. If you read the Project concept, there is a obvious limitation on using ARMATAK: it converts the Arma 3 Map location to a real world location, so let's use the Arma 3 Vanilla Maps (Altis...) as an examples, whatever the map developer had done different for real life, won't be showed by your TAK client map imagery, and there are MANY changes to be done in Arma 3 maps just for gameplay proposes, even when simulating real locations.  
2. Another thing to consider is that this addon is running using the OpenTAKServer API, check the [OTS Docs](https://docs.opentakserver.io/) for more information. Both ARMATAK and OpenTAKServer are on development projects, don't expect a stable experience using it.  
3. The TAK ecosystem have a slow learning curve (so does Arma), some YouTube channels i recommend is The [TAK Syndicate](https://www.youtube.com/@thetaksyndicate6234), [ATAK Maps](https://www.youtube.com/@ATAKMap) and [T Rex Labs](https://www.youtube.com/playlist?list=PLF9F26zKtAJ3d0jPgi80seK8-bSzlE2L9) to start learning, i don't think it's the perfect tool for situation awareness, but is the best tool we have publicly available right now, just remember that you must focus on data feeds, packages, planning and map editing tools, that is what you will use on ARMATAK addon.  
4. Also, the whole TAK environment is developed to give information for the client, not remove information, so by using ARMATAK, that points to a single FTS instance, you have to assume that every user will be displayed on the TAK platform, even the ones on the other side, and the other side will can connect just by using the same address as the dedicated server, i wish i can have a different approach that could make possible using ARMATAK on PvP sessions, but that is what we have for now.  
5. It is strongly recommended to use a dedicated server;

## How to use

1. Ensure you have a working OpenTAKServer instance running and reachable by the Arma 3 server.
2. Grab your phone add connect the TAK client to the OTS instance.
3. Build your A3 mission as you wish.
4. Insert the "ARMATAK CORE" module on the 3den editor.
5. Fill your OTS info on the module properties, i recommend to connect to OTS with a TCP connection, its not a safe connection but is easy for the newer TAK users, if there is no iTAK users.
6. The module itself can sync all player slots as TAK entities.
7. Start the game.
8. Other players can get the connection information on the briefing screen diary, with a connection QR Code.
9. When the game starts, you should see a "ARMATAK Connected" message both on the Game and on the tak server chat.
10. Enjoy your game session with enhanced situation awareness.

### Get in Touch

[Join the Discord Server for ARMATAK!](https://discord.gg/svK64PCycU)

## License

The whole Project is licensed under GPL License.

## Acknowledgment

* Ind3goFox - for releasing a3go, used to build the first versions of this project's extensions and for teaching me how extensions works.
* The Folks on the ACE Dev Team - for giving me many tips about arma 3 engine, project's architecture and software stuff, especially for
* BrettMayson - for building HEMTT and ARMA-RS, the tools i used to get this project to life, also for taking some time to teach a non-rust programmer how to make things work.
