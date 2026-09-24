local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/01 - blocksize 4096.flac")

print(audio.sampleRate)
print(audio.frames)

print(`duration: {audio.frames/audio.sampleRate}`)