# Maximal Music CLI Verb Set

## CLI Styling & Color Conventions

```shww
Color Scheme (typical console-compatible):
  Commands:        bold cyan — visually separates top-level actions clearly
  Things:          bold yellow — objects being manipulated stand out distinctly
  Options/Flags:   green — consistent with Unix CLI tradition for parameters
  Placeholders:    italic white (or dim white) — indicates user-provided input
  Errors:          bold red — immediate visibility for critical issues
  Warnings:        yellow — non-critical but cautionary
  Section Titles:  bold magenta — anchors large help texts with clarity
  JSON Output:     dim white — emphasizes “machine-friendly” output

Extended Markup Conventions:
  <placeholder>    user-supplied value (e.g. <Key>, <Chord>, <File>)
  [optional]       optional argument or flag group
  A | B            mutually exclusive options
  ...              indicates multiple values allowed
  --               long flag prefix
  -                short flag prefix

Examples with Color Semantics:
  music list chords --in Cmaj --json

    list       = bold cyan (verb)
    chords     = bold yellow (thing)
    --in       = green (flag)
    Cmaj       = italic white (placeholder)
    --json     = green (flag with machine-output semantics)
```

This document provides the complete maximal set of top-level verbs for the `music` CLI, along with detailed help-style sections, expanded stylistic conventions, and descriptive context aimed at building a professional-grade, discoverable command suite.

---

# Command Reference (Help-Style)

```sh
Usage: music <command> [options]

Commands:
  list           Enumerate static musical theory objects
  inspect        Detailed multi-section report for a musical entity
  analyze        Analyze input: key, scale, mode, tensions, function
  suggest        Theory-aware suggestions (reharm, modulations, voicings)
  explain        Explain reasoning behind analysis or musical choices
  convert        Transform musical representations (transpose, spell, roman→chord)
  validate       Check correctness of structures (chords, scales, intervals)
  render         Generate diagrams/maps (key graph, chord graph)
  expose         Show internal registries, tunings, pitch systems, modes
  generate       Create motif/arpeggio/rhythm seeds parameterized by density
  score          Heuristic scoring for progressions, melodies, and chords
  extrapolate    Extend or predict continuation of a musical pattern
  explain-diff   Compare two objects and highlight differences
  map            Produce relational maps (keys, chords, mixture sets)
  profile        Style/genre profiling for a key or progression
  interpolate    Blend between two musical entities
  search         Find objects matching constraints (notes, chords, scales)
  estimate       Produce heuristic estimates (brightness, instability)
  resolve        Suggest resolution paths for tensions or non-chord tones

Use "music <command> --help" for detailed usage of each command.
```

---

# Per-Command Help

Moved to separate document: see **Music CLI Command Help**.
