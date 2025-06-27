const bytes = [
    "e445a52e51cb9a1d40c6cde8260871e20c14defc825ec67694250818bb654065f4298d3156d571b4d4f8090c18e9a863fd51dc4c913e7ca8e73cd3cbb45769ac41aaade3696540dcf482a5fe57383a8f28cac2eb06000000069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f000000000019bdc520300000000",
    "e445a52e51cb9a1d494f4e7fb8d50ddc71337d91df2be75ff1c1ce88c1f5f0293ce7ee4562f6ee3e045fa07faad311dd069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f0000000000108b5000000000000"
]

for (const b of bytes) {
    const buf = Buffer.from(b, 'hex');
    const disc = buf.slice(0, 8);
    console.log(`disc=${Array.from(disc)} (${disc.toString('hex')})`, buf.length - 16);
}
