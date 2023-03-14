#!/usr/bin/env bash
set -ex

export YT_CHANNEL_ID="UCyWKvHqZ4UcW7vXUEYZJFUw"
export YT_KEY="AIzaSyDY65gjsGRDaXCMln8niXXYVBzdQsa2OWs"
# curl -s "https://youtube.googleapis.com/youtube/v3/search?part=id&q=ogrodje&key=${YT_KEY}&type=channel"


#curl -s "https://www.googleapis.com/youtube/v3/search?key=${YT_KEY}&channelId=${YT_CHANNEL_ID}&part=snippet,id&order=date&maxResults=50&type=video" \
#  | jq .

# works
#curl -s "https://www.googleapis.com/youtube/v3/search?key=${YT_KEY}&channelId=${YT_CHANNEL_ID}&part=id&order=date&maxResults=50&type=video" \
#  | jq .

curl -s "https://www.googleapis.com/youtube/v3/videos?key=${YT_KEY}&id=hMquZ55IKeI&part=statistics,contentDetails,snippet" \
  | jq . # "{statistics: .items[0].statistics}"
