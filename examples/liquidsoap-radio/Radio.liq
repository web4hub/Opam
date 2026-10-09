# OPAML example: local playlist -> fallback silence -> Icecast stream.
# Prerequisites:
#   - Liquidsoap 2.x installed separately
#   - music.m3u containing paths/URLs to audio tracks
#   - An Icecast server reachable at the settings below
#
# Keep real credentials out of version control. Replace these placeholders
# using your deployment's secret/configuration mechanism before publishing.

settings.log.level := 3

radio_playlist = playlist(mode="random", "music.m3u")
radio = fallback(track_sensitive=false, [radio_playlist, blank()])

icecast_host = "127.0.0.1"
icecast_port = 8000
icecast_password = "CHANGE_ME"
icecast_mount = "/radio.mp3"

output.icecast(
  %mp3,
  host=icecast_host,
  port=icecast_port,
  password=icecast_password,
  mount=icecast_mount,
  name="OPAML Example Radio",
  description="A sample Liquidsoap stream managed as an OPAML project",
  radio
)
