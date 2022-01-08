-- step.fut
-- Functions to advance an FDTD field.

-- Advance the 1D magnetic field.
entry hy_step_1d [n] (hy: [n]f64) (chyh: [n]f64) (chye: [n]f64) (ez: [n]f64): [n]f64 = 
  concat_to n
    (map (\i -> chyh[i] * hy[i] + chye[i] * (ez[i + 1] - ez[i])) (0..<n-1)) 
    [hy[n-1]]

-- Advance the 1D electric field.
entry ez_step_1d [n] (ez: [n]f64) (cezh: [n]f64) (ceze: [n]f64) (hy: [n]f64): [n]f64 =
  concat_to n 
    [ez[0]] 
    (map (\i -> ceze[i] * ez[i] + cezh[i] * (hy[i] - hy[i - 1])) (1..<n))

-- Step the simulation forward, without post-{magnetic/electric} functions.
entry step_1d [n] (hy: [n]f64) (chyh: [n]f64) (chye: [n]f64) (ez: [n]f64) (cezh: [n]f64) (ceze: [n]f64): ([n]f64, [n]f64) =
    let hy = hy_step_1d hy chyh chye ez in
    let ez = ez_step_1d ez cezh ceze hy in
    (hy, ez)

-- Step the simulation forward 'steps' times.
entry step_multiple_1d [n] (steps: i64) (hy: [n]f64) (chyh: [n]f64) (chye: [n]f64) (ez: [n]f64) (cezh: [n]f64) (ceze: [n]f64): ([n]f64, [n]f64) =
  loop (hy, ez) for i < steps do
    step_1d hy chyh chye ez cezh ceze

--
-- 2D TM^Z
--

-- Advance the Hx portion of the 2d magnetic field.
-- hx(mm, nn) = chxh(mm, nn) * hx(mm, nn) - chxe(mm, nn) * (ez(mm, nn + 1) - ez(mm, nn))
def hx_step_2d [x][y] (hx: [x][y]f64) (chxh: [x][y]f64) (chxe: [x][y]f64) (ez: [x][y]f64): [x][y]f64 =
  -- Concat within the inner array: 
  --  [[a, b, c], [1, 2, 3]] -> [[a, b, c, d], [1, 2, 3, 4]].
  map (\m -> 
    let tmp = map (\n -> chxh[m, n] * hx[m, n] - chxe[m, n] 
      * (ez[m, n + 1] - ez[m, n]))
    (0..<y-1) in
    concat_to y tmp [hx[m, y-1]]) 
  (0..<x)
 
-- Advance the Hy portion of the 2d magnetic field.
-- hy(mm, nn) = chyh(mm, nn) * hy(mm, nn) + chye(mm, nn) * (ez(mm + 1, nn) - ez(mm, nn))
def hy_step_2d [x][y] (hy: [x][y]f64) (chyh: [x][y]f64) (chye: [x][y]f64) (ez: [x][y]f64): [x][y]f64 =
  -- Concat the outer array:
  --  [[a, b, c, d]] -> [[a, b, c, d], [1, 2, 3, 4]].
  let tmp = map (\m -> 
    map (\n -> 
      chyh[m, n] * hy[m, n] + chye[m, n] * (ez[m + 1, n] - ez[m, n]))
    (0..<y)) 
  (0..<x-1) in
  concat_to x tmp
    (take ((x-1) * y) hy)

-- Advance the Ez portion of the 2d magnetic field.
-- ez(mm, nn) = ceze(mm, nn) * ez(mm, nn) + cezh(mm, nn) * ((hy(mm, nn) 
--    - hy((mm - 1), nn)) - (hx(mm, nn) - hx(mm, nn - 1)))
entry ez_step_2d [x][y] (ez: [x][y]f64) (cezh: [x][y]f64) (ceze: [x][y]f64) (hy: [x][y]f64) (hx: [x][y]f64): [x][y]f64 =
  -- Concat the beginning of ez within both the inner and outer arrays:
  --  [[1, 2, 3]] -> [[a, b, c, d], [1, 2, 3, 4]]
  let tmp = map (\m ->
    let a = map (\n -> ceze[m, n] * ez[m, n] + cezh[m, n] * ((hy[m, n] - hy[m-1, n]) - (hx[m, n] - hx[m, n-1]))) (1..<y) in
    concat_to y [ez[m, 0]] a)
    (1..<x) in
  concat_to x 
    (take y ez)
    tmp

-- Advance a full magnetic step in the 2d field.
entry magnetic_step_2d [x][y] (hx: [x][y]f64) (chxh: [x][y]f64) (chxe: [x][y]f64) (hy: [x][y]f64) (chyh: [x][y]f64) (chye: [x][y]f64) (ez: [x][y]f64):
  ([x][y]f64, [x][y]f64) =
    let hx = hx_step_2d hx chxh chxe ez in
    let hy = hy_step_2d hy chyh chye ez in
    (hx, hy)

-- Step the simulation forward, without post-{magnetic/electric} functions.
entry step_2d [x][y] (hx: [x][y]f64) (chxh: [x][y]f64) (chxe: [x][y]f64) (hy: [x][y]f64) (chyh: [x][y]f64)
                     (chye: [x][y]f64) (ez: [x][y]f64) (cezh: [x][y]f64) (ceze: [x][y]f64): ([x][y]f64, [x][y]f64, [x][y]f64) = 
  let hx = hx_step_2d hx chxh chxe ez in
  let hy = hy_step_2d hy chyh chye ez in
  let ez = ez_step_2d ez ceze cezh hy hx in
  (hx, hy, ez)

-- Step the simulation forward 'steps' times.
entry step_multiple_2d [x][y] (steps: i64) (hx: [x][y]f64) (chxh: [x][y]f64) (chxe: [x][y]f64) (hy: [x][y]f64) (chyh: [x][y]f64)
                              (chye: [x][y]f64) (ez: [x][y]f64) (ceze: [x][y]f64) (cezh: [x][y]f64): ([x][y]f64, [x][y]f64, [x][y]f64) =
  loop (hx, hy, ez) for i < steps do
    step_2d hx chxh chxe hy chyh chye ez ceze cezh

--
-- 3D
--

-- Advance the Hx portion of the 3D magnetic field.
-- hx(m, n, p) = chxh(m, n, p) * hx(m, n, p) +
--  chxe(m, n, p) * ((ey(m, n, p + 1) - ey(m, n, p)) -
--                   (ez(m, n + 1, p) - ez(m, n, p)))
def hx_step_3d [x][y][z] (hx: [x][y][z]f64) (chxh: [x][y][z]f64) (chxe: [x][y][z]f64) (ey: [x][y][z]f64) (ez: [x][y][z]f64): [x][y][z]f64 =
-- TODO: Concatenation!
  map (\m -> 
    map (\n -> 
      map (\p -> chxh[m, n, p] * hx[m, n, p] + chxe[m, n, p]
        * ((ey[m, n, p + 1] - ey[m, n, p]) - (ez[m, n + 1, p] - ez[m, n, p))) 
        (0..<p-1))
      (0..<n-1))
    (0..<m)

-- Advance the Hy portion of the 3D magnetic field.
-- hy(m, n, p) = chyh(m, n, p) * hy(m, n, p) +
--  chye(m, n, p) * ((ez(m + 1, n, p) - ez(m, n, p)) -
--                   (ex(m, n, p + 1) - ex(m, n, p)))
def hy_step_3d [x][y][z] (hy: [x][y][z]f64) (chyh: [x][y][z]f64) (chye: [x][y][z]f64) (ez: [x][y][z]f64) (ex: [x][y][z]f64): [x][y][z]f64 =
-- TODO: Concatenation!
  map (\m ->
    map (\n ->
      map (\p -> chyh[m, n, p] * hy[m, n, p] + chye[m, n, p]
        * ((ez[m + 1, n, p] - ez[m, n, p]) - (ex[m, n, p + 1] - ez[m, n, p])))
        (0..<p-1))
      (0..<n))
    (0..<m-1)

-- Advance the Hz portion of the 3D magnetic field.
-- hz(m, n, p) = chzh(m, n, p) * hz(m, n, p) +
--  chze(m, n, p) * ((ex(m, n + 1, p) - ex(m, n, p)) -
--                   (ey(m + 1, n, p) - ey(m, n, p)))
def hz_step_3d [x][y][z] (hz: [x][y][z]f64) (chzh: [x][y][z]f64) (chze: [x][y][z]f64) (ex: [x][y][z]f64) (ey: [x][y][z]f64): [x[y][z]]f64 =
  -- TODO: Concatenation!
  map (\m ->
    map (\n ->
      map (\p -> chzh[m, n, p] * hz[m, n, p] + chze[m, n, p]
        * ((ex[m, n + 1, p] - ex[m, n, p]) - (ey[m + 1, n, p] - ey[m, n, p])))
        (0..<p))
      (0..<n-1))
    (0..<m-1)


-- Advance a full magnetic step in the 3d field.
entry magnetic_step_3d [x][y][z] (hx: [x][y][z]f64) (chxh: [x][y][z]f64) (chxe: [x][y][z]f64)
                                 (hy: [x][y][z]f64) (chyh: [x][y][z]f64) (chye: [x][y][z]f64)
                                 (hz: [x][y][z]f64) (chzh: [x][y][z]f64) (chze: [z][y][z]f64)
                                 (ex: [x][y][z]f64) (ey: [x][y][z]f64) (ez: [x][y][z]: f64):
                                 ([x][y][z]f64, [x][y][z]f64, [x][y][z]f64) =
  let hx = hx_step_3d hx chxh chxe ey ez in
  let hy = hy_step_3d hy chyh chye ez ex in
  let hz = hz_step_3d hz chzh chze ex ey in
  (hx, hy, hz)

-- Advance the Ex portion of the 3D electric field.
-- ex(m, n, p) = cexe(m, n, p) * ex(m, n, p) +
--  cexh(m, n, p) * ((hz(m, n, p) - hz(m, n - 1, p)) -
--                   (hy(m, n, p) - hy(m, n, p - 1)))
def ex_step_3d [x][y][z] (ex: [x][y][z]f64) (cexe: [x][y][z]f64) (cexh: [x][y][z]f64) (hz: [x][y][z]f64, (hy: [x][y][z]f64): [x][y][z]f64 =
  -- TODO: Concatenation!
  map (\m ->
    map (\n ->
      map (\p -> cexe[m, n, p] * ex[m, n, p] + cexh[m, n, p]
        * ((hz[m, n, p] - hz[m, n - 1, p]) - (hy[m, n, p] - hy[m, n, p - 1])))
        (1..<p))
      (1..<n))
    (0..<m)

-- Advance the Ey portion of the 3D electric field.
-- ey(m, n, p) = ceye(m, n, p) * ey(m, n, p) +
--  ceyh(m, n, p) * ((hx(m, n, p) - hx(m, n, p - 1)) -
--                   (hz(m, n, p) - hz(m - 1, n, p)))
def ey_step_3d [x][y][z] (ey: [x][y][z]f64) (ceye: [x][y][z]f64) (ceyh: [x][y][z]f64) (hx: [x][y][z]f64) (hz: [x][y][z]f64): [x][y][z]f64 =
  -- TODO: Concatenation!
  map (\m ->
    map (\n ->
      map (\p -> ceye[m, n, p] * ey[m, n, p] + ceyh[m, n, p]
        * ((hx[m, n, p] - hx[m, n, p - 1]) - (hz[m, n, p] - hz[m - 1, n, p])))
        (1..<p))
      (0..<n))
    (1..<m)

-- Advance the Ez portion of the 3D electric field.
-- ez(m, n, p) = ceze(m, n, p) * ez(m, n, p) +
--  cezh(m, n, p) * ((hy(m, n, p) - hy(m - 1, n, p)) -
--                   (hx(m, n, p) - hx(m, n - 1, p)))
def ez_step_3d [x][y][z] (ez: [x][y][z]f64) (ceze: [x][y][z]f64) (cezh: [x][y][z]f64) (hy: [x][y][z]f64) (hx: [x][y][z]f64): [x][y][z]f64 =
  -- TODO: Concatenation!
  map (\m ->
    map (\n ->
      map (\p -> ceze[m, n, p] * ez[m, n, p] + cezh[m, n, p]
        * ((hy[m, n, p] - hy[m - 1, n, p]) - (hx[m, n, p] - hx[m, n - 1, p])))
        (0..<p))
      (1..<n))
    (1..<m)

-- Advance a full electric step in the 3d field.
entry electric_step_3d [x][y][z] (ex: [x][y][z]f64) (cexe: [x][y][z]f64) (cexh: [x][y][z]f64)
                                 (ey: [x][y][z]f64) (ceye: [x][y][z]f64) (ceyh: [x][y][z]f64)
                                 (ez: [x][y][z]f64) (ceze: [x][y][z]f64) (cezh: [z][y][z]f64)
                                 (hx: [x][y][z]f64) (hy: [x][y][z]f64) (hz: [x][y][z]f64):
                                 ([x][y][z]f64, [x][y][z]f64, [x][y][z]f64) =
  let ex = ex_step_3d hx chxh chxe ey ez in
  let ey = ey_step_3d hy chyh chye ez ex in
  let ez = ez_step_3d hz chzh chze ex ey in
  (ex, ey, ez)

-- Step the simulation foward, without post-{magnetic/electric} functions.
entry step_3d [x][y][z] (hx: [x][y][z]f64) (chxh: [x][y][z]f64) (chxe: [x][y][z]f64)
                        (hy: [x][y][z]f64) (chyh: [x][y][z]f64) (chye: [x][y][z]f64)
                        (hz: [x][y][z]f64) (chzh: [x][y][z]f64) (chze: [z][y][z]f64)
                        (ex: [x][y][z]f64) (cexe: [x][y][z]f64) (cexh: [x][y][z]f64)
                        (ey: [x][y][z]f64) (ceye: [x][y][z]f64) (ceyh: [x][y][z]f64)
                        (ez: [x][y][z]f64) (ceze: [x][y][z]f64) (cezh: [x][y][z]f64):
                        ([x][y][z]f64, [x][y][z]f64, [x][y][z]f64) =
  let (hx, hy, hz) = magnetic_step_3d hx chxh chxe hy chyh chye hz chzh chze ex ey ez in
  let (ex, ey, ez) = magnetic_step_3d ex cexe cexh ey ceye ceyh ez ceze cezh hx hy hz in
  (hx, hy, hz, ex, ey, ez)

-- Step the simulation foward 'steps' times.
entry step_multiple_3d [x][y][z] (steps: i64) 
                        (hx: [x][y][z]f64) (chxh: [x][y][z]f64) (chxe: [x][y][z]f64)
                        (hy: [x][y][z]f64) (chyh: [x][y][z]f64) (chye: [x][y][z]f64)
                        (hz: [x][y][z]f64) (chzh: [x][y][z]f64) (chze: [z][y][z]f64)
                        (ex: [x][y][z]f64) (cexe: [x][y][z]f64) (cexh: [x][y][z]f64)
                        (ey: [x][y][z]f64) (ceye: [x][y][z]f64) (ceyh: [x][y][z]f64)
                        (ez: [x][y][z]f64) (ceze: [x][y][z]f64) (cezh: [x][y][z]f64):
                        ([x][y][z]f64, [x][y][z]f64, [x][y][z]f64) =
  loop (hx, hy, hz, ex, ey, ez) for i < steps do
    step_3d hx chxh chxe hy chyh chye hz chzh chze hz chzh chze
      ex cexe cexh ey ceye ceyh ez ceze cezh
