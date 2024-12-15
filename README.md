# ARMATAK

![ARMATAK FRAME LOGO](./picture.png)

## Project Concept

ARMATAK is a Arma 3 addons for sending the player's location to WebSocket connection for SIMTAK, a self made application for mocking the EUD's location with provided data from the WebSocket. ARMATAK is also able to stream selected unit positions to TAK Clients in sessions on real locations maps. It basically sends HTTP requests to a OpenTAKServer API instance or FreeTAKServer from your Game Server, converting the useful information as HTTP Requests right into the OpenTAKServer/FreeTAKServer REST API as TAK entities soo the players can connect their WinTAK/ATAK/iTAK and game using the TAK features as a Blue Force Tracker or mission planning tool for Arma 3.

## Disclaimers

1. ARMATAK converts the Arma 3 Map location to a real world location, so let's use the Arma 3 Vanilla Maps (Altis...) as an examples, whatever the map developer had done different for real life, won't be showed by your TAK client map imagery, and there are MANY changes to be done in Arma 3 maps just for gameplay proposes, even when simulating real locations, precised map imagery based on the game map is available on mod discord server.  
2. Custom markers uses the OpenTAKServer REST API, check the [OTS Docs](https://docs.opentakserver.io/) for more information. Both ARMATAK and OpenTAKServer are on development projects, don't expect a stable experience using it, after some tests i made, the REST API is limited to 3 simultaneous markers due to NGINX timeout protection on traffic jams.  
3. The TAK ecosystem have a slow learning curve (so does Arma), some YouTube channels i recommend is The [TAK Syndicate](https://www.youtube.com/@thetaksyndicate6234), [ATAK Maps](https://www.youtube.com/@ATAKMap) and [T Rex Labs](https://www.youtube.com/playlist?list=PLF9F26zKtAJ3d0jPgi80seK8-bSzlE2L9) to start learning, i don't think it's the perfect tool for situation awareness, but is the best tool we have publicly available right now, learn how to use data feeds, packages, planning and map editing tools, that is what you will use on ARMATAK addon.  
4. The custom markers will be displayed to anyone connected to the TAK Server, no matter what is their side in the game session.
5. It is strongly recommended to use a dedicated server;

### Get in Touch

[Join the Discord Server for ARMATAK!](https://discord.gg/svK64PCycU)

## License

The whole Project is licensed under GPL License.

## Acknowledgment

* Ind3goFox - for releasing a3go, used to build the first versions of this project's extensions and for teaching me how extensions works.
* The Folks on the ACE Dev Team - for giving me many tips about arma 3 engine, project's architecture and software stuff, especially for
* BrettMayson - for building HEMTT and ARMA-RS, the tools i used to get this project to life, also for taking some time to teach a non-rust programmer how to make things work.
