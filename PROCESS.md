# Design process

This document is intended to record the design process in scoping and
implementing this application. It is written in the first person from the
perspective of the implementor / maintainer.

## Initial impressions

Approaching this problem, I did not have any experience with HLS or the m3u
format. As an initial exploration, I fetched the linked file with `curl` just
to visually inspect the structure of the data I was going to be working with.
It is a touch daunting at a first glance, but a cursory google yielded an
*incredibly* useful resource in [this long-form
article](https://www.toptal.com/apple/introduction-to-http-live-streaming-hls).

Now that this domain is somewhat demystified for me, I feel prepared to
estimate the complexity of this undertaking - I expect this will take
approximately 3 hours, with a possibility of finishing in less time if things
go right and I am able to set up CI / CD without too many YAML issues.

As such, I expect to complete this project by COB on Monday, 2/1.

## Implementation questions

The following questions were sent to the team on Friday, 1/29:

- On what basis (or by which key) should the variants in that master playlist
  be compared in sorting?
- Are there any prior assumptions (such as target screen size, hardware support
  for audio codecs or network bandwidth) that I should build into the sorting
  function?
- Should the variants remain grouped by audio codec? (probably yes but just
  want to confirm)
- What should the output format be? The same as the input? Human readable in
  some sense? Switchable between both options?

## Some notes from development

One rather important question was "is there an existing library to parse m3u8
data?". There are several on crates.io, and the first one I found (`hls_m3u8`)
seemed promising at first. The API seems simple enough, but as soon as I tried
to run it, I found that it cannot handle the slash-delimited list format in the
`CHANNELS` attribute, only plain old single integers. Since development
progress seemed pretty stale (no commits in the last few months), I figured it
was a lost cause for a quick project like this.

Passing over a handful of very stale and/or incomplete options, I settled on
`m3u8-rs`. It is not perfect (completely discards the `CHANNELS` attribute) but
at least it doesn't crash and burn. Given more time for implementation, my
preferred strategy would be to fork the other library (`hls_m3u8`), fix the
bug, and make a PR to their repository - I think that their API is nicer to
work with, all else being equal.

