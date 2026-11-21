# Audio programming journal

## 21 Nov
Basic waveforms (sine, square, sawtooth);

s: sample
sf: sampling frequency

Major gotchas:
* increments were in samples not seconds
* for non-whole frequencies (123.45 as opposed to 440.00), using sample increments with modulo on 
  the sample rate (`s = (s+1)%sr`) would cause audible glitch every second. That is because the remaining
  decimal part of the frequency was snapped to zero at the 1s mark
* It is helpful to think in wave phases rather than cycles. That way the problem above goes away as
  we increment by the next phase step (next = curr + note/sf). In addition we can perform nifty tricks
  to optimise away modulo operations with IF statements, since the phase is a non-dimensional concept
  ranging from 0. to 1.
* Phase here is not the thing added to the `wt` part of the traditional trigonometric expression.
  It is `wt` itself where `w` in this case is our note. So the various waveforms can be expressed as:
  SINE - `sin(2 * PI * phase)`
  SQUARE - `phase < 0.5 ? -1 : 1`
  SAWTOOTH - `((2 * phase) - 1)`
  To wrap around the phase we just need to check if it is >= 1. if it is, subtract 1.
