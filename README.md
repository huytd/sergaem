# Sergaem: WebSocket Multiplayer Game Server

This is a base code for building WebSocket-based multiplayer online game server.

The main components in this architecture:

- **ClientHandler:** Handler which will be allocated for each connected client, to procesisng the incoming/outgoing messages.
- **NetworkManager:** All of the connected client and their related jobs will be managed in this module.
- **GameManager:** The bridge to connect between networking layer and game logic layer, this module also manage the creation of gamme rooms - the smallest logic unit of the game where you will be working on. Think of this as a lobby of the game.
- **Game:** This is where you will be working on, write all your game logic in the update function of this module, you can write as many game room logic as you want for your server.

_(More details coming soon)_
