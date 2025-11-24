# SH-Potify. Rust Self Hosted .mp3 streaming with Spotify synchronization

## A Restful API to stream .mp3 files that synchronizes with your Spotify account to download your playlists into the server. It's also my first Rust project.

This project was built to automatically download your Copy-Right free playlists from Spotify and self stream them, avoiding the Spotify free account restrictions.
At the moment I'm working on an example client for Potify, but you can still launch  it and access it from the browser or Postman.

## The endpoints at the moment:
<details>
  <summary> GET /get_user_playlists</summary>
  Queries the Spotify API to retrieve the user's <strong>PUBLIC</strong> playlists and returns them as JSON so the client can display the information.

  ### Parameters:
  - **user-id** (required):  
    The Spotify user ID whose playlists will be retrieved.  
    You can find it in Spotify → Account → Edit personal information → Username.
    
  ### Example:
  ```bash
  curl "http://IP:PORT/get_user_playlists?user-id=SpotifyID"
  ```
  
</details>
<details>
  <summary> GET /select_db_playlists</summary>
  Returns <strong>all</strong> playlists that have been downloaded and stored on the server.

  ### Example:
  ```bash
  curl "http://IP:PORT/select_db_playlists"
  ```
  
</details>
<details>
  <summary> POST /download_playlists</summary>
  Sends a JSON object to the server containing a list of playlist IDs. These IDs are the official Spotify playlist identifiers.

  ### Body (JSON):
  An array containing the Spotify playlist IDs to download.

  ```bash
  curl -X POST "http://IP:PORT/download_playlists" \
     -H "Content-Type: application/json" \
     -d '["playlistId1", "playlistId2", "playlistId3"]'
  ```
  
</details>
<details>
  <summary>GET /stream/{songId}</summary>
  Starts streaming the `.mp3` file previously downloaded and stored on the server.

  ### Path parameter:
  **songId** — The original Spotify track ID.  
  This ID must correspond to a track already downloaded to the server; otherwise streaming will fail.

  ```bash
  curl "http://IP:PORT/stream/trackId"
  ```

</details>

### Future Improvements
- Endpoint names and parameter usage will be standardized for better consistency.
- New endpoints will be added, such as `/albums`, to expand the app's functionality and provide more options for users.

## The database:

It stores two tables, Track and Playlist (N:M)

### Track struct:
- file_path: Option<String>
- spotify_id: String
- album_name: String (Defaults to unknown)
- artist: String (Defaults to unknown)
- name: String (Defaults to unknown)
- url: String

Url is not stored on the database.

### Playlist struct:
- spotify_id: String
- name: String
- tracks: Option<Vec<Track>>

## Installation:

1. Clone this project
2. Build the docker image.
3. (Currently hardcoded; future updates will allow configuring this) Edit the config file with your developer Spotify account, setting Client-ID, Secret, and the port.
4. Launch the image with docker:
   ```bash
   docker run -p 6580:6580 -v /persistentVolume/tracks:/app/tracks -v /persistentVolume/sqlite:/app/sqlite
   ```
   The image uses /tracks and /sqlite to persist the data, persistentVolume is an example directory on the host.
5. If you want to access the server outside your local network you will need to open the port using a NAT rule.
   <strong>The server uses http, all the traffic will be unencrypted and readable.</strong>

The docker image will install ffmpeg, yt-dlp and spotdl as dependencies.


