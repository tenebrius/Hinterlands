"""Repetition check for the viewer's screenshots (real-scale overhaul, rule 6).

Measures how strongly a screenshot repeats itself at regular intervals: the
sharpest peak of its power spectrum against the spectrum's level at that
frequency, and the highest peak of its autocorrelation away from zero shift.
A stamped pattern, a hidden lattice or a texture tiled at one size shows as a
high score; natural detail (irregular sizes, angles that change from parcel to
parcel) does not, and neither do straight lines and edges, whose spectrum
spreads along a ray instead of peaking. The autocorrelation peak is reported
for diagnosis only: long straight roads and edges raise it too.

Usage:
  python3 -I tools/rhythm.py SHOT.png [SHOT.png ...]
  python3 -I tools/rhythm.py --before BEFORE.png --after AFTER.png
Options:
  --crop X0,Y0,X1,Y1  the part of the screenshot to measure (default: the map
                      area of the 1400 x 900 screenshots, 320,20,1380,840).

With --before and --after the check fails (exit status 1) if the after shot
brings in a rhythm: its spectral peak passes the limit and is half again the
before shot's.
"""

import sys

import numpy as np
from PIL import Image

SPECTRAL_LIMIT = 30.0
# Periods measured, in pixels: from fine detail up to a quarter of the crop.
MIN_PERIOD = 4.0


def load(path, crop):
    im = Image.open(path).convert("RGB").crop(crop)
    a = np.asarray(im, dtype=np.float64) / 255.0
    return 0.2126 * a[..., 0] + 0.7152 * a[..., 1] + 0.0722 * a[..., 2]


def box_blur(a, r):
    """Mean over a (2r+1) square, by an integral image (edges clamped)."""
    p = np.pad(a, r + 1, mode="edge")
    s = p.cumsum(0).cumsum(1)
    k = 2 * r + 1
    h, w = a.shape
    out = (s[k:k + h, k:k + w] - s[0:h, k:k + w] - s[k:k + h, 0:w] + s[0:h, 0:w])
    return out / (k * k)


def measure(gray):
    h, w = gray.shape
    # Keep detail below about 64 px: the layout of the map is not a rhythm.
    hp = gray - box_blur(gray, 32)
    win = np.outer(np.hanning(h), np.hanning(w))
    f = np.fft.fft2(hp * win)
    power = np.abs(f) ** 2
    ps = np.fft.fftshift(power)
    fy = np.fft.fftshift(np.fft.fftfreq(h))[:, None]
    fx = np.fft.fftshift(np.fft.fftfreq(w))[None, :]
    rad = np.sqrt(fx ** 2 + fy ** 2)
    lo = 4.0 / min(h, w)
    hi = 1.0 / MIN_PERIOD
    # The spectrum's level at each radius: the median of its annulus.
    nbins = 64
    edges = np.linspace(lo, hi, nbins + 1)
    idx = np.digitize(rad, edges) - 1
    ok = (idx >= 0) & (idx < nbins)
    level = np.zeros(nbins)
    for b in range(nbins):
        sel = ps[ok & (idx == b)]
        level[b] = np.median(sel) if sel.size else np.inf
    # A peak is a 3 x 3 patch of the spectrum, so one stray bin does not count.
    patch = sum(np.roll(np.roll(ps, dy, 0), dx, 1) for dy in (-1, 0, 1) for dx in (-1, 0, 1)) / 9.0
    # Against the spectrum's level at that radius (its annulus) and against its
    # neighbours along the same ray from the origin: a straight line or an
    # edge spreads along a ray and is no peak; a repeating pattern spikes.
    cy, cx = h // 2, w // 2
    yy, xx = np.mgrid[0:h, 0:w]
    dy_, dx_ = yy - cy, xx - cx
    r_bins = np.hypot(dy_, dx_)
    step = np.maximum(3.0, 0.25 * r_bins)
    scale_in = np.where(r_bins > 0, (r_bins - step) / np.maximum(r_bins, 1e-9), 0.0)
    scale_out = np.where(r_bins > 0, (r_bins + step) / np.maximum(r_bins, 1e-9), 0.0)

    def along(scale):
        y = np.clip(np.rint(cy + dy_ * scale).astype(int), 0, h - 1)
        x = np.clip(np.rint(cx + dx_ * scale).astype(int), 0, w - 1)
        return patch[y, x]

    ray = 0.5 * (along(scale_in) + along(scale_out))
    ratio = np.zeros_like(ps)
    ratio[ok] = np.minimum(patch[ok] / level[idx[ok]], patch[ok] / np.maximum(ray[ok], 1e-30))
    k = np.unravel_index(np.argmax(ratio), ratio.shape)
    peak = float(ratio[k])
    period = 1.0 / max(float(rad[k]), 1e-9)
    angle = float(np.degrees(np.arctan2(fy[k[0], 0], fx[0, k[1]])))
    # Autocorrelation away from zero shift (beyond the size of a single feature).
    ac = np.fft.fftshift(np.real(np.fft.ifft2(power)))
    ac /= ac[h // 2, w // 2]
    yy, xx = np.mgrid[0:h, 0:w]
    dist = np.hypot(yy - h // 2, xx - w // 2)
    far = (dist > 12) & (dist < min(h, w) / 4)
    acp = float(ac[far].max())
    return peak, period, angle, acp


def report(path, crop):
    peak, period, angle, acp = measure(load(path, crop))
    flag = "RHYTHM" if peak > SPECTRAL_LIMIT else "ok"
    print(f"{path}: spectral peak {peak:.1f} (period {period:.1f} px at {angle:.0f} deg), "
          f"autocorrelation peak {acp:.2f}  {flag}")
    return peak, acp


def main(argv):
    crop = (320, 20, 1380, 840)
    before = after = None
    shots = []
    i = 0
    while i < len(argv):
        a = argv[i]
        if a == "--crop":
            crop = tuple(int(x) for x in argv[i + 1].split(","))
            i += 2
        elif a == "--before":
            before = argv[i + 1]
            i += 2
        elif a == "--after":
            after = argv[i + 1]
            i += 2
        else:
            shots.append(a)
            i += 1
    for s in shots:
        report(s, crop)
    if before and after:
        bp, _ = report(before, crop)
        ap, _ = report(after, crop)
        brings = ap > SPECTRAL_LIMIT and ap > 1.5 * bp
        print("FAIL: the after shot brings in a rhythm" if brings else "pass")
        return 1 if brings else 0
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
