import StageMinusOne

open StageMinusOne

private def emit (doc : Node) : IO UInt32 := do
  IO.println (renderText {} doc)
  pure 0

def main (args : List String) : IO UInt32 := do
  match args with
  | "gen" :: n :: _ =>
      match n.toNat? with
      | some i => emit (generatedFixture i)
      | none =>
          IO.eprintln s!"invalid generated fixture index: {n}"
          pure 2
  | name :: _ =>
      match fixture name with
      | none =>
          IO.eprintln s!"unknown fixture: {name}"
          pure 2
      | some doc => emit doc
  | [] =>
      match fixture "nested" with
      | some doc => emit doc
      | none => pure 2
