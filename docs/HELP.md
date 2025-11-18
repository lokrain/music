# Music CLI Command Help

This document contains all per-command `--help` sections for the `music` CLI.

---

## list — help

```sh
Usage: music list <thing> [options]

Things:
  notes          List notes for a key or scale, including spellings and enharmonics
  degrees        List degree info: functional names, tendencies, resolutions
  chords         List diatonic/borrowed chords with qualities and symbols
  cadences       List cadence patterns (authentic, plagal, half, deceptive)
  progressions   List common progressions by style
  relations      List related keys, mixture keys, and pivot chords

Options:
  --in <Key>         Specify key (Cmaj, Amin, Gdorian, etc.)
  --style <profile>  Style profile (pop, jazz, rock, funk)
  --system <id>      Pitch/tuning system
  --json             JSON output
```

---

## inspect — help

```sh
Usage: music inspect <entity> [options]

Entities:
  key            Inspect a key with all sections in order
  chord          Inspect chord intervals, components, tensions
  scale          Inspect scale structure, intervals, step-patterns

Options:
  --sections <list>  Comma-separated list: identity, degrees, chords, cadences...
  --system <id>      System override
  --json             JSON output
```

---

## analyze — help

```sh
Usage: music analyze <input> [options]

Inputs:
  melody         Detect key/scale/mode and tension profile
  chords         Analyze progression for function and patterns
  midi           Analyze MIDI file (notes, chords, timing)

Options:
  --in <Key>         Force context key
  --system <id>      Tuning/pitch system
  --json             JSON output
```

---

## suggest — help

```sh
Usage: music suggest <thing> [options]

Things:
  chords         Reharmonization options ranked by fitness
  modulations    Possible target keys with pivot suggestions
  voicings       Chord voicings based on style and register

Options:
  --in <Key>         Context key
  --style <id>       Style profile
  --density <num>    Adjust harmonic density
  --json             JSON output
```

---

## explain — help

```sh
Usage: music explain <thing> [options]

Things:
  progression    Explain harmonic function and motion
  modulation     Break down pivot, target, functional shifts
  tension        Explain tension usage and suggested resolutions

Options:
  --json             JSON output
```

---

## convert — help

```sh
Usage: music convert <task> [options]

Tasks:
  transpose      Transpose notes or chord sequences
  spell          Convert pitch classes to correct enharmonic spellings
  roman          Convert Roman numerals to chord symbols in key

Options:
  --in <Key>         Context key
  --json             JSON output
```

---

## validate — help

```sh
Usage: music validate <thing>

Things:
  chord          Check chord spelling, interval correctness
  scale          Validate scale-step pattern
  progression    Check diatonicity and legality under style
```

---

## render — help

```sh
Usage: music render <diagram> [options]

Diagrams:
  key-map        Key relation graph
  chord-map      Graph of chord connections
  scale-chart    Interval and step-pattern chart

Options:
  --json             JSON graph
```

---

## expose — help

```sh
Usage: music expose <registry>

Registries:
  tunings        All known tunings
  systems        Pitch systems
  modes          Mode definitions
  scales         Named scale presets
```

---

## generate — help

```sh
Usage: music generate <command> [options]

Commands:
  motif        Generate a short motif within the selected scale
  arpeggio     Generate an arpeggio pattern anchored to the scale
  rhythm       Generate a rhythm cell sized for the density
  help         Print command-specific help

Options:
  -r, --root <int>        Root index anchoring the generated material (default 60)
  -s, --system <id>       Pitch system identifier (default 12tet)
      --scale <Scale>     Scale used to derive pitch material (major, dorian, ...)
      --density <mode>    sparse | balanced | dense (controls length + activity)
      --format <fmt>      text (default) or json output
```

---

## score — help

```sh
Usage: music score <command> [options]

Commands:
  progression  Score functional strength and cadence weight
  melody       Score melodic tension density and resolution
  chord        Score chord color/tension profile
  help         Print command-specific help

Shared options:
  --format <fmt>     text (default) or json

Progression options:
  --progression <I,ii,V>    Roman numerals separated by commas
  --in <Key>                Optional key hint surfaced in commentary

Melody options:
  --notes <60,62,...>       Comma-separated MIDI-like indices
  --in <Key>                Optional key/context label for the report
  --system <id>             Pitch system for note labels (default 12tet)

Chord options:
  --notes <60,64,67>        Comma-separated MIDI-like indices (root→top)
  --system <id>             Pitch system for labeling (default 12tet)
```

---

## extrapolate — help

```sh
Usage: music extrapolate <input> [options]

Inputs:
  melody         Predict continuation
  progression    Extend progression logically

Options:
  --in <Key>         Context key
  --json             JSON output
```

---

## explain-diff — help

```sh
Usage: music explain-diff <A> <B> [options]

Compare:
  keys, scales, chords, modes, pitch sets

Options:
  --json             JSON output
```

---

## map — help

```sh
Usage: music map <thing> [options]

Things:
  keys           Key adjacency map
  chords         Chord-type connectivity
  mixtures       Modal mixture mapping

Options:
  --json             JSON graph
```

---

## profile — help

```sh
Usage: music profile <thing> [options]

Things:
  key            Style interpretation of a key
  progression    Style usage profile and likelihood

Options:
  --style <id>       Style profile
  --json             JSON output
```

---

## interpolate — help

```sh
Usage: music interpolate <A> <B> [options]

Blend:
  keys, scales, chords, progressions

Options:
  --steps <n>        Number of intermediate states
  --json             JSON output
```

---

## search — help

```sh
Usage: music search <query> [options]

Queries:
  scale          Find scales containing specific notes
  chord          Find chords with interval criteria
  progression    Find matching progression patterns

Options:
  --json             JSON output
```

---

## estimate — help

```sh
Usage: music estimate <thing> [options]

Things:
  brightness     Estimate tonal brightness
  instability    Estimate instability/tension

Options:
  --json             JSON output
```

---

## resolve — help

```sh
Usage: music resolve <thing> [options]

Things:
  tension        Suggest resolutions for tensions
  degree         Suggest resolutions for unstable scale degrees

Options:
  --in <Key>         Context key
  --json             JSON output
```
