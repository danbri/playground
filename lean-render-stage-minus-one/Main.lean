import StageMinusOne

open StageMinusOne

def main (args : List String) : IO UInt32 := do
  let name := args.head?.getD "nested"
  match fixture name with
  | none =>
      IO.eprintln s!"unknown fixture: {name}"
      pure 2
  | some doc =>
      IO.println (renderText {} doc)
      pure 0
