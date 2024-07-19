# ARMATAK

![ARMATAK FRAME LOGO](./files/picture.png)

## Project Concept

ARMATAK is a server side Arma 3 addons for streaming unit positions to TAK Clients in sessions on real locations maps. It basically setup a FreeTAKServer instance on your Server (Must be a Linux Arma 3 Dedicated Server), convert the useful information in your game sessions and send it as HTTP Post Requests right into the FreeTAKServer REST API as TAK entities soo the players can connect their WinTAK/ATAK/iTAK and game using the TAK features for situation awareness/planning right on the map.

## Disclaimer

If you read the Project concept, there is a obvious limitation on using ARMATAK: it converts the Arma 3 Map location to a real world location, so let's use the Arma 3 Vanilla Maps (Altis...) as an examples, whatever the map developer had done different for real life, won't be showed by your TAK client map imagery, and there are MANY changes to be done in Arma 3 maps just for gameplay proposes, even when simulating real locations.
Another thing to consider is that each dedicated server also generates a FreeTAKServer instance, and the whole TAK environment is developed to give information for the client, not remove information, so by using ARMATAK, that points to a single FTS instance, you have to assume that every user will be displayed on the TAK platform, even the ones on the other side, and the other side will can connect just by using the same address as the dedicated server, i wish i can have a different approach that could make possible using ARMATAK on PvP sessions, but that is what we have for now.

## Getting Started

1. Add ARMATAK to the Mod Preset of your mission.
2. Launch Your Server with ARMATAK.
3. Wait for the Arma 3 Dedicated log that the ARMATAK is connected to the FTS instance, you can check this by the Dedicated Server console, or the Dedicated Server logs if you are running the instance as a background process.
4. Check your Mission Briefing Diary for the ARMATAK Connection information.
5. Grab your android phone, tablet, iPhone, second screen or whatever you want to use as TAK client and connect it using the connection information. The FTS instance should send you a welcome message to the ARMATAK Instance.
6. Start the Mission.
7. Check your buddies on the real world location.
8. Send your planning information to your buddies using the real world location.

### Get in Touch

[Join the Discord Server for ARMATAK!](https://discord.gg/aZK9VSJ8)

## License

The whole Project is licensed under MIT License.
