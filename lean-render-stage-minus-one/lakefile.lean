import Lake
open Lake DSL

package «stage-minus-one» where

lean_lib StageMinusOne where
  roots := #[`StageMinusOne]

@[default_target]
lean_exe stage_minus_one where
  root := `Main
