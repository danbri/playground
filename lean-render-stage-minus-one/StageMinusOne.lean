namespace StageMinusOne

structure Point where
  x : Nat
  y : Nat
  deriving Repr, DecidableEq

structure Rect where
  x : Nat
  y : Nat
  w : Nat
  h : Nat
  deriving Repr, DecidableEq

inductive Color where
  | black | white | red | blue | green
  deriving Repr, DecidableEq

structure BoxStyle where
  width : Option Nat := none
  height : Option Nat := none
  padding : Nat := 0
  background : Color := .white
  color : Color := .black
  deriving Repr, DecidableEq

inductive Node where
  | box : BoxStyle → List Node → Node
  | text : String → Node
  deriving Repr, DecidableEq

structure Environment where
  viewportWidth : Nat := 120
  glyphWidth : Nat := 8
  lineHeight : Nat := 16
  deriving Repr, DecidableEq

inductive DisplayItem where
  | rect : Rect → Color → DisplayItem
  | text : Point → String → Color → DisplayItem
  deriving Repr, DecidableEq

structure LaidOut where
  width : Nat
  height : Nat
  items : List DisplayItem
  deriving Repr, DecidableEq

private def colorName : Color → String
  | .black => "black"
  | .white => "white"
  | .red => "red"
  | .blue => "blue"
  | .green => "green"

private def pushWord (limit : Nat) (lines : List String) (word : String) : List String :=
  match lines.reverse with
  | [] => [word]
  | last :: revRest =>
      let candidate := if last.isEmpty then word else last ++ " " ++ word
      if candidate.length ≤ limit then
        (candidate :: revRest).reverse
      else
        lines ++ [word]

/-- Fixed-advance wrapping. Long words are intentionally not split in R1. -/
def wrapText (env : Environment) (width : Nat) (s : String) : List String :=
  let charsPerLine := max 1 (width / max 1 env.glyphWidth)
  let words := s.splitOn " " |>.filter (fun w => !w.isEmpty)
  match words with
  | [] => [""]
  | _ => words.foldl (pushWord charsPerLine) []

private def textItems (lineHeight x y : Nat) (color : Color) : List String → Nat → List DisplayItem
  | [], _ => []
  | line :: rest, i =>
      DisplayItem.text { x := x, y := y + i * lineHeight } line color ::
        textItems lineHeight x y color rest (i + 1)

mutual
  private def layoutChildren
      (env : Environment) (children : List Node) (x y width : Nat) (color : Color) : Nat × List DisplayItem :=
    children.foldl (fun (acc : Nat × List DisplayItem) child =>
      let (dy, items) := acc
      let laid := layoutNode env child x (y + dy) width color
      (dy + laid.height, items ++ laid.items)
    ) (0, [])

  /-- Batch layout semantics for the tiny renderer. -/
  def layoutNode (env : Environment) (node : Node) (x y availableWidth : Nat) (inheritedColor : Color) : LaidOut :=
    match node with
    | .text s =>
        let lines := wrapText env availableWidth s
        let items := textItems env.lineHeight x y inheritedColor lines 0
        { width := availableWidth, height := lines.length * env.lineHeight, items := items }
    | .box style children =>
        let outerWidth := style.width.getD availableWidth
        let innerWidth := outerWidth - (2 * style.padding)
        let contentX := x + style.padding
        let contentY := y + style.padding
        let (contentHeight, childItems) := layoutChildren env children contentX contentY innerWidth style.color
        let naturalHeight := contentHeight + 2 * style.padding
        let outerHeight := style.height.getD naturalHeight
        let bg := DisplayItem.rect { x := x, y := y, w := outerWidth, h := outerHeight } style.background
        { width := outerWidth, height := outerHeight, items := bg :: childItems }
end

def render (env : Environment) (doc : Node) : List DisplayItem :=
  (layoutNode env doc 0 0 env.viewportWidth .black).items

def fixture : String → Option Node
  | "nested" => some <|
      .box { width := some 120, padding := 10, background := .red, color := .black } [
        .text "hello world hello world",
        .box { width := some 80, height := some 20, padding := 0, background := .blue, color := .white } [],
        .text "tail"
      ]
  | "inherit" => some <|
      .box { width := some 96, padding := 8, background := .green, color := .white } [
        .text "alpha beta gamma delta"
      ]
  | _ => none

private def showItem : DisplayItem → String
  | .rect r c => s!"rect {r.x} {r.y} {r.w} {r.h} {colorName c}"
  | .text p s c => s!"text {p.x} {p.y} {colorName c} |{s}|"

def renderText (env : Environment) (doc : Node) : String :=
  String.intercalate "\n" ((render env doc).map showItem)

end StageMinusOne
