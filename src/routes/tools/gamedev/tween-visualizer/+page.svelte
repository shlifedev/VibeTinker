<script lang="ts">
  type EasingFunction = (t: number) => number

  // Configurable parameters with defaults
  let backOvershoot = $state(1.70158)
  let elasticAmplitude = $state(1)
  let elasticPeriod = $state(0.3)

  // Which easings use which params
  const backEasings = new Set(["easeInBack", "easeOutBack", "easeInOutBack"])
  const elasticEasings = new Set(["easeInElastic", "easeOutElastic"])

  function getEasingFn(id: string): EasingFunction {
    switch (id) {
      case "linear": return (t) => t
      case "easeInQuad": return (t) => t * t
      case "easeOutQuad": return (t) => t * (2 - t)
      case "easeInOutQuad": return (t) => t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t
      case "easeInCubic": return (t) => t * t * t
      case "easeOutCubic": return (t) => { const t1 = t - 1; return t1 * t1 * t1 + 1 }
      case "easeInOutCubic": return (t) => t < 0.5 ? 4 * t * t * t : (t - 1) * (2 * t - 2) * (2 * t - 2) + 1
      case "easeInQuart": return (t) => t * t * t * t
      case "easeOutQuart": return (t) => { const t1 = t - 1; return 1 - t1 * t1 * t1 * t1 }
      case "easeInOutQuart": return (t) => { const t1 = t - 1; return t < 0.5 ? 8 * t * t * t * t : 1 - 8 * t1 * t1 * t1 * t1 }
      case "easeInBack": return (t) => {
        const s = backOvershoot
        return (s + 1) * t * t * t - s * t * t
      }
      case "easeOutBack": return (t) => {
        const s = backOvershoot
        return 1 + (s + 1) * Math.pow(t - 1, 3) + s * Math.pow(t - 1, 2)
      }
      case "easeInOutBack": return (t) => {
        const s = backOvershoot * 1.525
        return t < 0.5
          ? (Math.pow(2 * t, 2) * ((s + 1) * 2 * t - s)) / 2
          : (Math.pow(2 * t - 2, 2) * ((s + 1) * (t * 2 - 2) + s) + 2) / 2
      }
      case "easeInElastic": return (t) => {
        if (t === 0 || t === 1) return t
        const p = elasticPeriod
        const a = Math.max(elasticAmplitude, 1)
        const s = p / (2 * Math.PI) * Math.asin(1 / a)
        return -(a * Math.pow(2, 10 * (t - 1)) * Math.sin(((t - 1) - s) * (2 * Math.PI) / p))
      }
      case "easeOutElastic": return (t) => {
        if (t === 0 || t === 1) return t
        const p = elasticPeriod
        const a = Math.max(elasticAmplitude, 1)
        const s = p / (2 * Math.PI) * Math.asin(1 / a)
        return a * Math.pow(2, -10 * t) * Math.sin((t - s) * (2 * Math.PI) / p) + 1
      }
      case "easeInBounce": return (t) => 1 - getEasingFn("easeOutBounce")(1 - t)
      case "easeOutBounce": return (t) => {
        const n1 = 7.5625
        const d1 = 2.75
        if (t < 1 / d1) return n1 * t * t
        if (t < 2 / d1) { const t1 = t - 1.5 / d1; return n1 * t1 * t1 + 0.75 }
        if (t < 2.5 / d1) { const t1 = t - 2.25 / d1; return n1 * t1 * t1 + 0.9375 }
        const t1 = t - 2.625 / d1; return n1 * t1 * t1 + 0.984375
      }
      default: return (t) => t
    }
  }

  const colorPalette = [
    "#60A5FA", "#F472B6", "#34D399", "#FBBF24", "#A78BFA",
    "#FB923C", "#10B981", "#8B5CF6", "#F87171", "#38BDF8"
  ]

  const easingGroups: Record<string, string[]> = {
    "Quad": ["easeInQuad", "easeOutQuad", "easeInOutQuad"],
    "Cubic": ["easeInCubic", "easeOutCubic", "easeInOutCubic"],
    "Quart": ["easeInQuart", "easeOutQuart", "easeInOutQuart"],
    "Back": ["easeInBack", "easeOutBack", "easeInOutBack"],
    "Elastic": ["easeInElastic", "easeOutElastic"],
    "Bounce": ["easeInBounce", "easeOutBounce"]
  }

  const SVG_W = 300
  const SVG_H = 300
  const PAD = 30
  const GW = SVG_W - PAD * 2
  const GH = SVG_H - PAD * 2
  const STEPS = 200

  let selectedEasings = $state<Set<string>>(new Set(["easeOutCubic"]))
  let animationDuration = $state(1)
  let isPlaying = $state(false)
  let animationProgress = $state(0)
  let animationFrame: number | null = null
  let hoverT = $state<number | null>(null)
  let previewMode = $state<"position" | "scale" | "rotation">("position")
  let loopMode = $state<"off" | "loop" | "yoyo">("off")
  let codeTab = $state<"dotween" | "litmotion" | "static">("dotween")

  // Start/End values for each preview mode (0~1)
  let perAxis = $state(false)
  // Uniform
  let posStart = $state(0)
  let posEnd = $state(1)
  let scaleStart = $state(0)
  let scaleEnd = $state(1)
  let rotStart = $state(0)
  let rotEnd = $state(1)
  // Per-axis
  let posStartX = $state(0), posEndX = $state(1)
  let posStartY = $state(0), posEndY = $state(0)
  let scaleStartX = $state(0), scaleEndX = $state(1)
  let scaleStartY = $state(0), scaleEndY = $state(1)
  let rotStartX = $state(0), rotEndX = $state(1)
  let rotStartY = $state(0), rotEndY = $state(0)
  let rotStartZ = $state(0), rotEndZ = $state(0)

  let copyFeedback = $state(false)

  // Whether any selected easing needs param sliders
  let showBackParams = $derived(Array.from(selectedEasings).some(id => backEasings.has(id)))
  let showElasticParams = $derived(Array.from(selectedEasings).some(id => elasticEasings.has(id)))

  interface CurveInfo {
    id: string
    color: string
    fn: EasingFunction
    path: string
  }

  let curves = $derived.by(() => {
    // read params to trigger reactivity
    void backOvershoot
    void elasticAmplitude
    void elasticPeriod

    const result: CurveInfo[] = []
    let ci = 0
    for (const id of selectedEasings) {
      const fn = getEasingFn(id)
      const color = colorPalette[ci % colorPalette.length]
      ci++

      const points: string[] = []
      for (let i = 0; i <= STEPS; i++) {
        const t = i / STEPS
        const v = fn(t)
        const x = PAD + t * GW
        const y = PAD + GH - v * GH
        points.push(`${x.toFixed(2)},${y.toFixed(2)}`)
      }
      result.push({ id, color, fn, path: `M${points.join("L")}` })
    }
    return result
  })

  let hoverValues = $derived.by(() => {
    if (hoverT === null) return []
    return curves.map(c => ({
      id: c.id,
      color: c.color,
      value: c.fn(hoverT!),
      y: PAD + GH - c.fn(hoverT!) * GH
    }))
  })

  function toggleEasing(id: string) {
    if (selectedEasings.has(id)) {
      selectedEasings.delete(id)
    } else {
      selectedEasings.add(id)
    }
    selectedEasings = new Set(selectedEasings)
  }

  function handleSvgMouseMove(e: MouseEvent) {
    const svg = e.currentTarget as SVGSVGElement
    const rect = svg.getBoundingClientRect()
    const x = (e.clientX - rect.left) / rect.width * SVG_W
    const t = (x - PAD) / GW
    hoverT = Math.max(0, Math.min(1, t))
  }

  function handleSvgMouseLeave() {
    hoverT = null
  }

  function startAnimation() {
    if (isPlaying) return
    isPlaying = true
    animationProgress = 0
    let startTime = performance.now()
    let forward = true

    function animate(now: number) {
      const elapsed = (now - startTime) / 1000
      const raw = Math.min(elapsed / animationDuration, 1)
      animationProgress = forward ? raw : 1 - raw

      if (raw < 1) {
        animationFrame = requestAnimationFrame(animate)
      } else if (loopMode === "loop") {
        startTime = performance.now()
        animationFrame = requestAnimationFrame(animate)
      } else if (loopMode === "yoyo") {
        forward = !forward
        startTime = performance.now()
        animationFrame = requestAnimationFrame(animate)
      } else {
        animationProgress = 1
        isPlaying = false
        animationFrame = null
      }
    }
    animationFrame = requestAnimationFrame(animate)
  }

  function resetAnimation() {
    if (animationFrame !== null) {
      cancelAnimationFrame(animationFrame)
      animationFrame = null
    }
    isPlaying = false
    animationProgress = 0
  }

  function getEasingColor(id: string): string | undefined {
    return curves.find(c => c.id === id)?.color
  }

  // DOTween ease mapping
  const dotweenEaseMap: Record<string, string> = {
    linear: "Ease.Linear",
    easeInQuad: "Ease.InQuad", easeOutQuad: "Ease.OutQuad", easeInOutQuad: "Ease.InOutQuad",
    easeInCubic: "Ease.InCubic", easeOutCubic: "Ease.OutCubic", easeInOutCubic: "Ease.InOutCubic",
    easeInQuart: "Ease.InQuart", easeOutQuart: "Ease.OutQuart", easeInOutQuart: "Ease.InOutQuart",
    easeInBack: "Ease.InBack", easeOutBack: "Ease.OutBack", easeInOutBack: "Ease.InOutBack",
    easeInElastic: "Ease.InElastic", easeOutElastic: "Ease.OutElastic",
    easeInBounce: "Ease.InBounce", easeOutBounce: "Ease.OutBounce"
  }

  function generateDotweenCode(id: string): string {
    const ease = dotweenEaseMap[id] ?? "Ease.Linear"
    const dur = `${animationDuration}f`
    if (backEasings.has(id)) {
      return `// ${id} (overshoot: ${backOvershoot})\ntransform.DOMove(targetPos, ${dur}).SetEase(${ease}, overshoot: ${backOvershoot}f);\ntransform.DOScale(targetScale, ${dur}).SetEase(${ease}, overshoot: ${backOvershoot}f);`
    }
    if (elasticEasings.has(id)) {
      return `// ${id} (amplitude: ${elasticAmplitude}, period: ${elasticPeriod})\ntransform.DOMove(targetPos, ${dur}).SetEase(${ease}, amplitude: ${elasticAmplitude}f, period: ${elasticPeriod}f);\ntransform.DOScale(targetScale, ${dur}).SetEase(${ease}, amplitude: ${elasticAmplitude}f, period: ${elasticPeriod}f);`
    }
    return `// ${id}\ntransform.DOMove(targetPos, ${dur}).SetEase(${ease});\ntransform.DOScale(targetScale, ${dur}).SetEase(${ease});`
  }

  function generateLitMotionCode(id: string): string {
    const ease = dotweenEaseMap[id] ?? "Ease.Linear"
    const dur = `${animationDuration}f`
    // LitMotion doesn't support per-ease params natively, add comment
    let extra = ""
    if (backEasings.has(id)) extra = ` // overshoot: ${backOvershoot}`
    if (elasticEasings.has(id)) extra = ` // amplitude: ${elasticAmplitude}, period: ${elasticPeriod}`
    return `// ${id}${extra}\nLMotion.Create(0f, 1f, ${dur})\n    .WithEase(${ease})\n    .Bind(x => transform.position = Vector3.Lerp(startPos, endPos, x));`
  }

  function getStaticCode(id: string): string {
    const s = backOvershoot
    const a = elasticAmplitude
    const p = elasticPeriod
    const map: Record<string, string> = {
      linear: "public static float Linear(float t) => t;",
      easeInQuad: "public static float EaseInQuad(float t) => t * t;",
      easeOutQuad: "public static float EaseOutQuad(float t) => t * (2f - t);",
      easeInOutQuad: "public static float EaseInOutQuad(float t)\n    => t < 0.5f ? 2f * t * t : -1f + (4f - 2f * t) * t;",
      easeInCubic: "public static float EaseInCubic(float t) => t * t * t;",
      easeOutCubic: "public static float EaseOutCubic(float t)\n{\n    t -= 1f;\n    return t * t * t + 1f;\n}",
      easeInOutCubic: "public static float EaseInOutCubic(float t)\n    => t < 0.5f ? 4f * t * t * t : (t - 1f) * (2f * t - 2f) * (2f * t - 2f) + 1f;",
      easeInQuart: "public static float EaseInQuart(float t) => t * t * t * t;",
      easeOutQuart: "public static float EaseOutQuart(float t)\n{\n    t -= 1f;\n    return 1f - t * t * t * t;\n}",
      easeInOutQuart: "public static float EaseInOutQuart(float t)\n{\n    if (t < 0.5f) return 8f * t * t * t * t;\n    t -= 1f;\n    return 1f - 8f * t * t * t * t;\n}",
      easeInBack: `public static float EaseInBack(float t, float s = ${s}f)\n{\n    return (s + 1f) * t * t * t - s * t * t;\n}`,
      easeOutBack: `public static float EaseOutBack(float t, float s = ${s}f)\n{\n    t -= 1f;\n    return 1f + (s + 1f) * t * t * t + s * t * t;\n}`,
      easeInOutBack: `public static float EaseInOutBack(float t, float s = ${s}f)\n{\n    s *= 1.525f;\n    if (t < 0.5f)\n        return (Mathf.Pow(2f * t, 2) * ((s + 1f) * 2f * t - s)) / 2f;\n    return (Mathf.Pow(2f * t - 2f, 2) * ((s + 1f) * (t * 2f - 2f) + s) + 2f) / 2f;\n}`,
      easeInElastic: `public static float EaseInElastic(float t, float amplitude = ${a}f, float period = ${p}f)\n{\n    if (t == 0f || t == 1f) return t;\n    float a = Mathf.Max(amplitude, 1f);\n    float s = period / (2f * Mathf.PI) * Mathf.Asin(1f / a);\n    return -(a * Mathf.Pow(2f, 10f * (t - 1f)) * Mathf.Sin(((t - 1f) - s) * (2f * Mathf.PI) / period));\n}`,
      easeOutElastic: `public static float EaseOutElastic(float t, float amplitude = ${a}f, float period = ${p}f)\n{\n    if (t == 0f || t == 1f) return t;\n    float a = Mathf.Max(amplitude, 1f);\n    float s = period / (2f * Mathf.PI) * Mathf.Asin(1f / a);\n    return a * Mathf.Pow(2f, -10f * t) * Mathf.Sin((t - s) * (2f * Mathf.PI) / period) + 1f;\n}`,
      easeInBounce: "public static float EaseInBounce(float t)\n    => 1f - EaseOutBounce(1f - t);",
      easeOutBounce: "public static float EaseOutBounce(float t)\n{\n    const float n = 7.5625f;\n    const float d = 2.75f;\n    if (t < 1f / d) return n * t * t;\n    if (t < 2f / d) { t -= 1.5f / d; return n * t * t + 0.75f; }\n    if (t < 2.5f / d) { t -= 2.25f / d; return n * t * t + 0.9375f; }\n    t -= 2.625f / d; return n * t * t + 0.984375f;\n}"
    }
    return map[id] ?? `// ${id}: not implemented`
  }

  function generateCode(): string {
    const ids = Array.from(selectedEasings)
    if (ids.length === 0) return "// 이징 함수를 선택하세요"

    if (codeTab === "dotween") {
      return `using DG.Tweening;\n\n${ids.map(generateDotweenCode).join("\n\n")}`
    }

    if (codeTab === "litmotion") {
      return `using LitMotion;\nusing LitMotion.Extensions;\n\n${ids.map(generateLitMotionCode).join("\n\n")}`
    }

    // static
    const lines = ids.map(getStaticCode)
    return `using UnityEngine;\n\npublic static class Easing\n{\n    ${lines.join("\n\n    ")}\n}`
  }

  async function copyCode() {
    try {
      await navigator.clipboard.writeText(generateCode())
      copyFeedback = true
      setTimeout(() => { copyFeedback = false }, 1500)
    } catch (err) {
      console.error("Failed to copy:", err)
    }
  }
</script>

<div class="p-8 max-w-[1100px] mx-auto">
  <header class="mb-6">
    <h1 class="text-2xl font-bold mb-1">Tween Visualizer</h1>
    <p class="text-sm opacity-50">이징 함수를 시각화하고 비교합니다</p>
  </header>

  <div class="flex flex-col lg:grid lg:grid-cols-[1fr_280px] gap-6 items-start">
    <!-- Left -->
    <div class="flex flex-col gap-4">
      <!-- SVG Graph -->
      <svg
        viewBox="0 0 {SVG_W} {SVG_H}"
        class="w-full max-h-[350px] rounded-lg border border-surface-500 bg-[#1a1a1a] select-none"
        onmousemove={handleSvgMouseMove}
        onmouseleave={handleSvgMouseLeave}
      >
        <!-- Grid -->
        {#each Array(11) as _, i}
          <line
            x1={PAD + (GW / 10) * i} y1={PAD}
            x2={PAD + (GW / 10) * i} y2={PAD + GH}
            stroke="#2a2a2a" stroke-width="0.5"
          />
          <line
            x1={PAD} y1={PAD + (GH / 10) * i}
            x2={PAD + GW} y2={PAD + (GH / 10) * i}
            stroke="#2a2a2a" stroke-width="0.5"
          />
        {/each}

        <!-- Axes -->
        <line x1={PAD} y1={PAD + GH} x2={PAD + GW} y2={PAD + GH} stroke="#555" stroke-width="1" />
        <line x1={PAD} y1={PAD} x2={PAD} y2={PAD + GH} stroke="#555" stroke-width="1" />

        <!-- Axis labels -->
        <text x={PAD} y={PAD + GH + 16} fill="#666" font-size="9" text-anchor="middle">0</text>
        <text x={PAD + GW / 2} y={PAD + GH + 16} fill="#666" font-size="9" text-anchor="middle">0.5</text>
        <text x={PAD + GW} y={PAD + GH + 16} fill="#666" font-size="9" text-anchor="middle">1</text>
        <text x={PAD - 8} y={PAD + GH + 3} fill="#666" font-size="9" text-anchor="end">0</text>
        <text x={PAD - 8} y={PAD + 3} fill="#666" font-size="9" text-anchor="end">1</text>

        <!-- Curves -->
        {#each curves as curve}
          <path d={curve.path} fill="none" stroke={curve.color} stroke-width="2" stroke-linecap="round" />
        {/each}

        <!-- Hover crosshair -->
        {#if hoverT !== null}
          <line
            x1={PAD + hoverT * GW} y1={PAD}
            x2={PAD + hoverT * GW} y2={PAD + GH}
            stroke="#ffffff30" stroke-width="0.5" stroke-dasharray="4,3"
          />
          {#each hoverValues as hv}
            <circle cx={PAD + hoverT * GW} cy={hv.y} r="3.5" fill={hv.color} />
          {/each}
        {/if}

        <!-- Animation progress line -->
        {#if isPlaying}
          <line
            x1={PAD + animationProgress * GW} y1={PAD}
            x2={PAD + animationProgress * GW} y2={PAD + GH}
            stroke="#fff" stroke-width="1"
          />
        {/if}
      </svg>

      <!-- Hover info -->
      {#if hoverT !== null && hoverValues.length > 0}
        <div class="flex flex-wrap gap-3 text-xs font-mono">
          <span class="opacity-50">t={hoverT.toFixed(3)}</span>
          {#each hoverValues as hv}
            <span style="color: {hv.color}">{hv.id}: {hv.value.toFixed(3)}</span>
          {/each}
        </div>
      {/if}

      <!-- Animation preview -->
      <div class="rounded-lg border border-surface-500 bg-surface-900 p-3">
        <div class="flex items-center gap-3 mb-3">
          <button
            class="btn btn-sm preset-filled-primary-500 w-16"
            onclick={startAnimation}
            disabled={isPlaying || selectedEasings.size === 0}
          >
            {isPlaying ? "Stop" : "Play"}
          </button>
          <button
            class="btn btn-sm preset-tonal"
            onclick={resetAnimation}
          >
            Reset
          </button>
          <button
            class="btn btn-sm text-[10px] {loopMode === 'loop' ? 'preset-filled-primary-500' : 'preset-tonal'}"
            onclick={() => loopMode = loopMode === "loop" ? "off" : "loop"}
          >
            Loop
          </button>
          <button
            class="btn btn-sm text-[10px] {loopMode === 'yoyo' ? 'preset-filled-primary-500' : 'preset-tonal'}"
            onclick={() => loopMode = loopMode === "yoyo" ? "off" : "yoyo"}
          >
            Yoyo
          </button>
          <div class="flex gap-1 ml-1">
            {#each ["position", "scale", "rotation"] as mode}
              <button
                class="btn btn-sm text-[10px] {previewMode === mode ? 'preset-filled-primary-500' : 'preset-tonal'}"
                onclick={() => previewMode = mode as "position" | "scale" | "rotation"}
              >
                {mode === "position" ? "Position" : mode === "scale" ? "Scale" : "Rotation"}
              </button>
            {/each}
          </div>
          <input
            type="range" min="0.3" max="3" step="0.1"
            bind:value={animationDuration}
            class="flex-1"
          />
          <span class="text-xs font-mono opacity-50 w-8">{animationDuration.toFixed(1)}s</span>
        </div>

        <!-- Start / End -->
        <div class="mt-1">
          <div class="flex items-center gap-1.5 mb-1.5">
            <span class="text-[9px] opacity-40">Range</span>
            <button
              class="px-1.5 py-0.5 rounded text-[9px] {perAxis ? 'bg-primary-500 text-white' : 'bg-surface-700 opacity-60 hover:opacity-80'}"
              onclick={() => perAxis = !perAxis}
            >
              Per Axis
            </button>
          </div>

          {#if !perAxis}
            <div class="grid grid-cols-2 gap-2">
              <div class="rounded bg-surface-800 px-2 py-1">
                <div class="flex justify-between text-[9px] mb-0.5">
                  <span class="opacity-50">Start</span>
                  <span class="font-mono opacity-60">{(previewMode === "position" ? posStart : previewMode === "scale" ? scaleStart : rotStart).toFixed(2)}</span>
                </div>
                <input
                  type="range" min="0" max="1" step="0.01"
                  value={previewMode === "position" ? posStart : previewMode === "scale" ? scaleStart : rotStart}
                  oninput={(e) => {
                    const v = parseFloat((e.target as HTMLInputElement).value)
                    if (previewMode === "position") posStart = v
                    else if (previewMode === "scale") scaleStart = v
                    else rotStart = v
                  }}
                  class="w-full"
                />
              </div>
              <div class="rounded bg-surface-800 px-2 py-1">
                <div class="flex justify-between text-[9px] mb-0.5">
                  <span class="opacity-50">End</span>
                  <span class="font-mono opacity-60">{(previewMode === "position" ? posEnd : previewMode === "scale" ? scaleEnd : rotEnd).toFixed(2)}</span>
                </div>
                <input
                  type="range" min="0" max="1" step="0.01"
                  value={previewMode === "position" ? posEnd : previewMode === "scale" ? scaleEnd : rotEnd}
                  oninput={(e) => {
                    const v = parseFloat((e.target as HTMLInputElement).value)
                    if (previewMode === "position") posEnd = v
                    else if (previewMode === "scale") scaleEnd = v
                    else rotEnd = v
                  }}
                  class="w-full"
                />
              </div>
            </div>
          {:else}
            {#if previewMode === "position"}
              <div class="grid grid-cols-2 gap-1.5">
                {#each [
                  { label: "X Start", get: posStartX, set: (v: number) => posStartX = v },
                  { label: "X End", get: posEndX, set: (v: number) => posEndX = v },
                  { label: "Y Start", get: posStartY, set: (v: number) => posStartY = v },
                  { label: "Y End", get: posEndY, set: (v: number) => posEndY = v }
                ] as item}
                  <div class="rounded bg-surface-800 px-1.5 py-1">
                    <div class="flex justify-between text-[8px] mb-0.5">
                      <span class="opacity-50">{item.label}</span>
                      <span class="font-mono opacity-60">{item.get.toFixed(2)}</span>
                    </div>
                    <input type="range" min="0" max="1" step="0.01" value={item.get} oninput={(e) => item.set(parseFloat((e.target as HTMLInputElement).value))} class="w-full" />
                  </div>
                {/each}
              </div>
            {:else if previewMode === "scale"}
              <div class="grid grid-cols-2 gap-1.5">
                {#each [
                  { label: "X Start", get: scaleStartX, set: (v: number) => scaleStartX = v },
                  { label: "X End", get: scaleEndX, set: (v: number) => scaleEndX = v },
                  { label: "Y Start", get: scaleStartY, set: (v: number) => scaleStartY = v },
                  { label: "Y End", get: scaleEndY, set: (v: number) => scaleEndY = v }
                ] as item}
                  <div class="rounded bg-surface-800 px-1.5 py-1">
                    <div class="flex justify-between text-[8px] mb-0.5">
                      <span class="opacity-50">{item.label}</span>
                      <span class="font-mono opacity-60">{item.get.toFixed(2)}</span>
                    </div>
                    <input type="range" min="0" max="1" step="0.01" value={item.get} oninput={(e) => item.set(parseFloat((e.target as HTMLInputElement).value))} class="w-full" />
                  </div>
                {/each}
              </div>
            {:else}
              <div class="grid grid-cols-2 gap-1.5">
                {#each [
                  { label: "Z Start", get: rotStartZ, set: (v: number) => rotStartZ = v },
                  { label: "Z End", get: rotEndZ, set: (v: number) => rotEndZ = v }
                ] as item}
                  <div class="rounded bg-surface-800 px-1.5 py-1">
                    <div class="flex justify-between text-[8px] mb-0.5">
                      <span class="opacity-50">{item.label}</span>
                      <span class="font-mono opacity-60">{item.get.toFixed(2)}</span>
                    </div>
                    <input type="range" min="0" max="1" step="0.01" value={item.get} oninput={(e) => item.set(parseFloat((e.target as HTMLInputElement).value))} class="w-full" />
                  </div>
                {/each}
              </div>
            {/if}
          {/if}
        </div>

        <!-- Parameters -->
        {#if showBackParams || showElasticParams}
          <div class="grid grid-cols-2 gap-3 mt-2 mb-1">
            {#if showBackParams}
              <div class="rounded-lg bg-surface-800 p-2.5">
                <div class="flex justify-between text-[10px] mb-1">
                  <span class="opacity-60">Overshoot</span>
                  <span class="font-mono opacity-70">{backOvershoot.toFixed(2)}</span>
                </div>
                <input
                  type="range" min="0" max="5" step="0.01"
                  bind:value={backOvershoot}
                  class="w-full"
                />
                <div class="flex justify-between items-center mt-1">
                  <span class="text-[10px] text-surface-300">되돌아가는 정도</span>
                  <button
                    class="text-[9px] opacity-40 hover:opacity-70"
                    onclick={() => backOvershoot = 1.70158}
                  >
                    Reset
                  </button>
                </div>
              </div>
            {/if}

            {#if showElasticParams}
              <div class="rounded-lg bg-surface-800 p-2.5">
                <div class="flex justify-between text-[10px] mb-1">
                  <span class="opacity-60">Amplitude</span>
                  <span class="font-mono opacity-70">{elasticAmplitude.toFixed(2)}</span>
                </div>
                <input
                  type="range" min="0.5" max="3" step="0.05"
                  bind:value={elasticAmplitude}
                  class="w-full"
                />
                <div class="flex justify-between items-center mt-1">
                  <span class="text-[10px] text-surface-300">진동 크기</span>
                  <button
                    class="text-[9px] opacity-40 hover:opacity-70"
                    onclick={() => elasticAmplitude = 1}
                  >
                    Reset
                  </button>
                </div>
              </div>
              <div class="rounded-lg bg-surface-800 p-2.5">
                <div class="flex justify-between text-[10px] mb-1">
                  <span class="opacity-60">Period</span>
                  <span class="font-mono opacity-70">{elasticPeriod.toFixed(2)}</span>
                </div>
                <input
                  type="range" min="0.05" max="1" step="0.01"
                  bind:value={elasticPeriod}
                  class="w-full"
                />
                <div class="flex justify-between items-center mt-1">
                  <span class="text-[10px] text-surface-300">진동 주기 (작을수록 빠름)</span>
                  <button
                    class="text-[9px] opacity-40 hover:opacity-70"
                    onclick={() => elasticPeriod = 0.3}
                  >
                    Reset
                  </button>
                </div>
              </div>
            {/if}
          </div>
        {/if}

        <div class="grid grid-cols-4 gap-2 mt-3">
          {#each curves as curve}
            {@const eased = curve.fn(animationProgress)}
            {@const pxVal = perAxis ? posStartX + eased * (posEndX - posStartX) : posStart + eased * (posEnd - posStart)}
            {@const pyVal = perAxis ? posStartY + eased * (posEndY - posStartY) : 0.5}
            {@const sxVal = perAxis ? scaleStartX + eased * (scaleEndX - scaleStartX) : scaleStart + eased * (scaleEnd - scaleStart)}
            {@const syVal = perAxis ? scaleStartY + eased * (scaleEndY - scaleStartY) : scaleStart + eased * (scaleEnd - scaleStart)}
            {@const rVal = perAxis ? rotStartX + eased * (rotEndX - rotStartX) : rotStart + eased * (rotEnd - rotStart)}
            <div class="flex flex-col items-center gap-1">
              <div class="relative w-full aspect-square bg-surface-800 rounded-lg overflow-hidden border border-surface-700">
                <!-- guide lines -->
                <div class="absolute inset-0 opacity-10">
                  <div class="absolute left-1/2 top-0 bottom-0 w-px bg-white"></div>
                  <div class="absolute top-1/2 left-0 right-0 h-px bg-white"></div>
                </div>
                <!-- animated object -->
                {#if previewMode === "position"}
                  <div
                    class="absolute w-5 h-5 rounded"
                    style="
                      background-color: {curve.color};
                      left: {pxVal * 70 + 15}%;
                      top: {(1 - pyVal) * 70 + 15}%;
                      transform: translate(-50%, -50%);
                    "
                  ></div>
                {:else if previewMode === "scale"}
                  <div
                    class="absolute rounded"
                    style="
                      background-color: {curve.color};
                      width: {8 + sxVal * 50}%;
                      height: {8 + syVal * 50}%;
                      left: 50%;
                      top: 50%;
                      transform: translate(-50%, -50%);
                      opacity: {0.4 + Math.max(sxVal, syVal) * 0.6};
                    "
                  ></div>
                {:else}
                  <div
                    class="absolute w-6 h-6 rounded"
                    style="
                      background-color: {curve.color};
                      left: 50%;
                      top: 50%;
                      transform: translate(-50%, -50%) rotate({rVal * 360}deg);
                      border-right: 3px solid rgba(255,255,255,0.5);
                    "
                  ></div>
                {/if}
              </div>
              <span class="text-[10px] font-mono opacity-50 truncate w-full text-center">{curve.id}</span>
            </div>
          {/each}
        </div>
      </div>

      <!-- C# Code Output -->
      {#if selectedEasings.size > 0}
        <div class="rounded-lg border border-surface-500 p-3">
          <div class="flex items-center justify-between mb-2">
            <div class="flex gap-1">
              {#each [["dotween", "DOTween"], ["litmotion", "LitMotion"], ["static", "Static"]] as [key, label]}
                <button
                  class="btn btn-sm text-[10px] {codeTab === key ? 'preset-filled-primary-500' : 'preset-tonal'}"
                  onclick={() => codeTab = key as "dotween" | "litmotion" | "static"}
                >
                  {label}
                </button>
              {/each}
            </div>
            <button class="btn btn-sm preset-tonal text-xs" onclick={copyCode}>
              {copyFeedback ? "Copied!" : "Copy"}
            </button>
          </div>
          <pre class="text-[11px] font-mono whitespace-pre overflow-x-auto max-h-[200px] overflow-y-auto opacity-80">{generateCode()}</pre>
        </div>
      {/if}
    </div>

    <!-- Right: Controls -->
    <div class="lg:sticky lg:top-4 flex flex-col gap-3 lg:max-h-[calc(100vh-6rem)] overflow-y-auto">
      <div class="flex gap-2">
        <button
          class="btn btn-sm preset-tonal text-xs flex-1"
          onclick={() => {
            const all = ["linear", ...Object.values(easingGroups).flat()]
            selectedEasings = new Set(all)
          }}
        >
          전체 선택
        </button>
        <button
          class="btn btn-sm preset-tonal text-xs flex-1"
          onclick={() => { selectedEasings = new Set() }}
        >
          전체 해제
        </button>
      </div>

      <!-- Linear -->
      <div class="pb-3 border-b border-surface-700">
        <label class="flex items-center gap-2 cursor-pointer hover:bg-surface-700/30 px-2 py-1 rounded text-sm">
          <input
            type="checkbox"
            checked={selectedEasings.has("linear")}
            onchange={() => toggleEasing("linear")}
          />
          <span>linear</span>
          {#if selectedEasings.has("linear")}
            <div class="ml-auto w-3 h-3 rounded-full" style="background-color: {getEasingColor('linear')}"></div>
          {/if}
        </label>
        <p class="text-[10px] text-surface-300 px-2 mt-0.5">일정한 속도로 변화</p>
      </div>

      <!-- Groups -->
      {#each Object.entries(easingGroups) as [group, ids]}
        <div class="pb-3 border-b border-surface-700 last:border-0">
          <h4 class="text-[10px] font-bold uppercase tracking-wider text-lime-400 mb-0.5 px-2">{group}</h4>
          <p class="text-[10px] text-surface-300 px-2 mb-1">
            {#if group === "Quad"}
              t² 기반, 부드러운 가감속
            {:else if group === "Cubic"}
              t³ 기반, Quad보다 강한 가감속
            {:else if group === "Quart"}
              t⁴ 기반, 더 극적인 가감속
            {:else if group === "Back"}
              목표를 살짝 넘겼다 돌아옴 · overshoot 조절 가능
            {:else if group === "Elastic"}
              스프링처럼 진동 · amplitude/period 조절 가능
            {:else if group === "Bounce"}
              바닥에 튀는 공처럼 반복 감쇠
            {/if}
          </p>
          {#each ids as id}
            <label class="flex items-center gap-2 cursor-pointer hover:bg-surface-700/30 px-2 py-1 rounded text-sm">
              <input
                type="checkbox"
                checked={selectedEasings.has(id)}
                onchange={() => toggleEasing(id)}
              />
              <span class="text-xs">{id}</span>
              {#if selectedEasings.has(id)}
                <div class="ml-auto w-3 h-3 rounded-full" style="background-color: {getEasingColor(id)}"></div>
              {/if}
            </label>
          {/each}
        </div>
      {/each}

    </div>
  </div>
</div>
