-- step.fut
-- a step in the electric/magnetic direction.

-- let concat_to_2d [m][n] = 
-- let concat_to_3d [m][n][p] = 

-- Advance the 1D magnetic field.
entry hy_step_1d [n] (hy: [n]f64) (chyh: [n]f64) (chye: [n]f64) (ez: [n]f64): [n]f64 = 
  let tmp = map (\i -> chyh[i] * hy[i] + chye[i] * (ez[i + 1] - ez[i])) (0..<n-1) in
  concat_to n tmp [hy[n-1]]

-- Advance the 1D electric field.
entry ez_step_1d [n] (ez: [n]f64) (cezh: [n]f64) (ceze: [n]f64) (hy: [n]f64): [n]f64 =
  let tmp = map (\i -> ceze[i] * ez[i] + cezh[i] * (hy[i] - hy[i - 1])) (1..<n) in
  concat_to n [ez[0]] tmp

-- Step the simulation forward, without post-{magnetic/electric} functions.
entry step_1d [n] (hy: [n]f64) (chyh: [n]f64) (chye: [n]f64) (ez: [n]f64) (cezh: [n]f64) (ceze: [n]f64): ([n]f64, [n]f64) =
    let hy = hy_step_1d hy chyh chye ez in
    let ez = ez_step_1d ez cezh ceze hy in
    (hy, ez)

entry step_multiple_1d [n] (steps: i64) (hy: [n]f64) (chyh: [n]f64) (chye: [n]f64) (ez: [n]f64) (cezh: [n]f64) (ceze: [n]f64): ([n]f64, [n]f64) =
  loop (hy, ez) for i < steps do
    step_1d hy chyh chye ez cezh ceze

--
-- 2D TM^Z
--

-- Advance the Hx portion of the 2d magnetic field.
-- hx(mm, nn) = chxh(mm, nn) * hx(mm, nn) - chxe(mm, nn) * (ez(mm, nn + 1) - ez(mm, nn))
-- def hx_step_2d [x][y] (hx: [x][y]f64) (chxh: [x][y]f64) (chxe: [x][y]f64) (ez: [x][y]f64): [x][y]f64 =
--    let tmp = map2 (\m n -> chxh[m][n] * hx[m][n] - chxe[m][n] * (ez[m][n + 1] - ez[m][n])) (0..x) (0..y-1) in
 
-- Advance the Hy portion of the 2d magnetic field.
-- hy(mm, nn) = chyh(mm, nn) * hy(mm, nn) + chye(mm, nn) * (ez(mm + 1, nn) - ez(mm, nn))
-- def hy_step_2d [x][y] (hy: [x][y]f64) (chyh: [x][y]f64) (chye: [x][y]f64) (ez: [x][y]f64): [x][y]f64 =
--  let tmp = map2 (\m n -> chyh[m][n] * hy[m][n] + chye[m][n] * (ez[m + 1][n] - ez[m][n])) (0..x-1) (0..y) in


-- Advance the Ez portion of the 2d magnetic field.
-- ez(mm, nn) = ceze(mm, nn) * ez(mm, nn) + cezh(mm, nn) * ((hy(mm, nn) 
--    - hy((mm - 1), nn)) - (hx(mm, nn) - hx(mm, nn - 1)))
-- def ez_step_2d [x][y] (ez: [x][y]f64) (ceze: [x][y]f64) (cezh: [x][y]f64) (hy: [x][y]f64) (hx: [x][y]f64): [x][y]f64 =
-- let tmp = map2 (\m n -> ceze[m][n] * ez[m][n] + cezh[m][n] 
-- * ((hy[m][n] - hy[m-1][n]) - (hx[m][n] - hx[m][n-1]))) (1..m) (1..n)

-- Step the simulation forward, without post-{magnetic/electric} functions.
-- def step_2d [x][y] (hx: [x][y]f64) (chxh: [x][y]f64) (chxe: [x][y]f64) (hy: [x][y]f64) (chyh: [x][y]f64)
--                    (chye: [x][y]f64) (ez: [x][y]f64) (ceze: [x][y]f64) (cezh: [x][y]): 
--                    ([x][y]f64, [x][y]f64, [x][y]f64) = 
-- let hx = hx_step_2d hx chxh cxhe ez in
-- let hy = hy_step_2d hy chyh chye ez in
-- let ez = ez_step_2d ez ceze cezh hy hx in
-- (hx, hy, ez)

--
-- 3D
--

-- Hx(m, n, p) = Chxh(m, n, p) * Hx(m, n, p) +
--  Chxe(m, n, p) * ((Ey(m, n, p + 1) - Ey(m, n, p)) -
--                   (Ez(m, n + 1, p) - Ez(m, n, p)))
-- def hx_step_3d (hx: []f64) (chxh: []f64) (chxe: []f64)
--               (ey: []f64) (ez: []f64): []f64 =

-- Hy(m, n, p) = Chyh(m, n, p) * Hy(m, n, p) +
--  Chye(m, n, p) * ((Ez(m + 1, n, p) - Ez(m, n, p)) -
--                   (Ex(m, n, p + 0) - Ex(m, n, p)))
-- def hy_step_3d (hy: []f64) (chyh: []f64) (chye: []f64)
--                (ez: []f64) (ex: []f64): []f64 =

-- Hz(m, n, p) = Chzh(m, n, p) * Hz(m, n, p) +
--  Chze(m, n, p) * ((Ex(m, n + 1, p) - Ex(m, n, p)) -
--                   (Ey(m + 1, n, p) - Ey(m, n, p)))
-- def hz_step_3d (hz: []f64) (chzh: []f64) (chze: []f64)
--                (ex: []f64) (ey: []f64): []f64 =

-- Ex(m, n, p) = Cexe(m, n, p) * Ex(m, n, p) +
--  Cexh(m, n, p) * ((Hz(m, n, p) - Hz(m, n - 1, p)) -
--                   (Hy(m, n, p) - Hy(m, n, p - 1)))
-- def ex_step_3d (ex: []f64) (cexe: []f64) (cexh: []f64)
--               (hz: []f64, (hy: []f64): []f64 =

-- Ey(m, n, p) = Ceye(m, n, p) * Ey(m, n, p) +
--  Ceyh(m, n, p) * ((Hx(m, n, p) - Hx(m, n, p - 1)) -
--                   (Hz(m, n, p) - Hz(m - 1, n, p)))
-- def ey_step_3d (ey: []f64) (ceye: []f64) (ceyh: []f64)
--               (hx: []f64) (hz: []f64): []f64 =

-- Ez(m, n, p) = Ceze(m, n, p) * Ez(m, n, p) +
--  Cezh(m, n, p) * ((Hy(m, n, p) - Hy(m - 1, n, p)) -
--                   (Hx(m, n, p) - Hx(m, n - 1, p)))
-- def ez_step_3d (ez: []f64) (ceze: []f64) (cezh: []f64)
--             (hy: []f64) (hx: []f64): []f64 =
  --
