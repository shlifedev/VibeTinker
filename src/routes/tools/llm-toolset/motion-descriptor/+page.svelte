<script lang="ts">
  import { commands } from "$lib/bindings"

  interface MotionPoint {
    x: number
    y: number
    timestamp: number
  }

  let isRecording = $state(false)
  let motionData = $state<MotionPoint[]>([])
  let descriptorEn = $state("")
  let descriptorKo = $state("")
  let canvas = $state<HTMLCanvasElement | null>(null)
  let startTime = 0
  let sampleCount = $state(10)
  let isPlaying = $state(false)
  let playbackSpeed = $state(1)
  let playbackProgress = $state(0)
  let animationFrameId = $state(0)
  let playbackStartTime = $state(0)

  function getCanvasCoords(e: MouseEvent): { x: number, y: number } {
    if (!canvas) return { x: 0, y: 0 }
    const rect = canvas.getBoundingClientRect()
    const scaleX = canvas.width / rect.width
    const scaleY = canvas.height / rect.height
    return {
      x: (e.clientX - rect.left) * scaleX,
      y: (e.clientY - rect.top) * scaleY
    }
  }

  function startRecording(e: MouseEvent) {
    if (!canvas) return

    isRecording = true
    motionData = []
    startTime = Date.now()

    const { x, y } = getCanvasCoords(e)
    motionData.push({
      x,
      y,
      timestamp: 0
    })
    
    drawCanvas()
  }

  function recordMotion(e: MouseEvent) {
    if (!isRecording || !canvas) return

    const { x, y } = getCanvasCoords(e)
    motionData.push({
      x,
      y,
      timestamp: Date.now() - startTime
    })
    
    drawCanvas()
  }

  async function stopRecording() {
    if (!isRecording) return
    
    isRecording = false
    
    if (motionData.length > 1) {
      try {
        const result = await commands.generateMotionDescriptor(JSON.stringify(motionData), sampleCount)
        if (result.status === "ok") {
          descriptorEn = result.data.en
          descriptorKo = result.data.ko
        } else {
          descriptorEn = `Error: ${result.error}`
          descriptorKo = ""
        }
      } catch (e) {
        descriptorEn = `Error: ${e}`
        descriptorKo = ""
      }
    }
  }

  function getSpeedColor(normalized: number): string {
    const hue = 240 - (normalized * 240)
    return `hsl(${hue}, 100%, 50%)`
  }

  function drawCanvas() {
    if (!canvas) return
    
    const ctx = canvas.getContext("2d")
    if (!ctx) return
    
    ctx.clearRect(0, 0, canvas.width, canvas.height)
    
    if (motionData.length > 1) {
      const speeds: number[] = []
      for (let i = 1; i < motionData.length; i++) {
        const dx = motionData[i].x - motionData[i - 1].x
        const dy = motionData[i].y - motionData[i - 1].y
        const dt = motionData[i].timestamp - motionData[i - 1].timestamp
        const distance = Math.sqrt(dx * dx + dy * dy)
        const speed = dt > 0 ? distance / dt : 0
        speeds.push(speed)
      }
      
      const logSpeeds = speeds.map(s => Math.log(s + 1))
      const logMax = Math.max(...logSpeeds, 0.1)
      
      ctx.lineWidth = 6
      ctx.lineCap = "round"
      ctx.lineJoin = "round"
      
      for (let i = 1; i < motionData.length; i++) {
        const x1 = motionData[i - 1].x
        const y1 = motionData[i - 1].y
        const x2 = motionData[i].x
        const y2 = motionData[i].y
        
        const normalized1 = i > 1 ? logSpeeds[i - 2] / logMax : 0
        const normalized2 = logSpeeds[i - 1] / logMax
        
        const gradient = ctx.createLinearGradient(x1, y1, x2, y2)
        gradient.addColorStop(0, getSpeedColor(normalized1))
        gradient.addColorStop(1, getSpeedColor(normalized2))
        
        ctx.strokeStyle = gradient
        ctx.beginPath()
        ctx.moveTo(x1, y1)
        ctx.lineTo(x2, y2)
        ctx.stroke()
      }
      
      ctx.fillStyle = "#4CAF50"
      ctx.beginPath()
      ctx.arc(motionData[0].x, motionData[0].y, 8, 0, Math.PI * 2)
      ctx.fill()
      
      ctx.fillStyle = "#f44336"
      ctx.beginPath()
      const last = motionData[motionData.length - 1]
      ctx.arc(last.x, last.y, 8, 0, Math.PI * 2)
      ctx.fill()
      
      ctx.fillStyle = "#fff"
      ctx.font = "bold 10px sans-serif"
      ctx.textAlign = "center"
      ctx.textBaseline = "middle"
      ctx.fillText("P1", motionData[0].x, motionData[0].y)
      ctx.fillText("P2", last.x, last.y)
    }
  }

  function clearCanvas() {
    stopPlayback()
    motionData = []
    descriptorEn = ""
    descriptorKo = ""
    if (canvas) {
      const ctx = canvas.getContext("2d")
      if (ctx) {
        ctx.clearRect(0, 0, canvas.width, canvas.height)
      }
    }
  }

  async function copyDescriptor(text: string) {
    if (text) {
      await navigator.clipboard.writeText(text)
    }
  }

  async function copyImage() {
    if (!canvas || motionData.length < 2) return

    try {
      // Create a temp canvas with background baked in
      const tmp = document.createElement("canvas")
      tmp.width = canvas.width
      tmp.height = canvas.height
      const tmpCtx = tmp.getContext("2d")!
      tmpCtx.fillStyle = "#1a1a1a"
      tmpCtx.fillRect(0, 0, tmp.width, tmp.height)
      tmpCtx.drawImage(canvas, 0, 0)

      const blob = await new Promise<Blob | null>((resolve) => {
        tmp.toBlob(resolve, "image/png")
      })

      if (blob) {
        await navigator.clipboard.write([
          new ClipboardItem({ "image/png": blob })
        ])
      }
    } catch (e) {
      console.error("Failed to copy image:", e)
    }
  }

  function getSampledPoints(): { nx: number, ny: number, nt: number, speed: number }[] {
    if (motionData.length < 2) return []

    const minX = Math.min(...motionData.map(p => p.x))
    const maxX = Math.max(...motionData.map(p => p.x))
    const minY = Math.min(...motionData.map(p => p.y))
    const maxY = Math.max(...motionData.map(p => p.y))
    const totalDuration = motionData[motionData.length - 1].timestamp

    const step = Math.max(1, Math.floor(motionData.length / sampleCount))
    const sampled: { nx: number, ny: number, nt: number, speed: number }[] = []

    for (let i = 0; i < motionData.length; i += step) {
      const p = motionData[i]
      const nx = maxX > minX ? (p.x - minX) / (maxX - minX) : 0.5
      const ny = maxY > minY ? (p.y - minY) / (maxY - minY) : 0.5
      const nt = totalDuration > 0 ? p.timestamp / totalDuration : 0

      let speed = 0
      if (i > 0) {
        const prev = motionData[Math.max(0, i - step)]
        const dx = p.x - prev.x
        const dy = p.y - prev.y
        const dt = p.timestamp - prev.timestamp
        speed = dt > 0 ? Math.sqrt(dx * dx + dy * dy) / dt : 0
      }

      sampled.push({ nx, ny, nt, speed })
    }

    return sampled
  }

  function drawSimulation(progress: number = 1) {
    if (!canvas) return
    const ctx = canvas.getContext("2d")
    if (!ctx) return

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    const points = getSampledPoints()
    if (points.length === 0) return

    const padding = 40
    const w = canvas.width - padding * 2
    const h = canvas.height - padding * 2

    // Draw grid
    ctx.strokeStyle = "#333"
    ctx.lineWidth = 0.5
    for (let i = 0; i <= 10; i++) {
      const x = padding + (w * i / 10)
      const y = padding + (h * i / 10)
      ctx.beginPath()
      ctx.moveTo(x, padding)
      ctx.lineTo(x, padding + h)
      ctx.stroke()
      ctx.beginPath()
      ctx.moveTo(padding, y)
      ctx.lineTo(padding + w, y)
      ctx.stroke()
    }

    // Axis labels
    ctx.fillStyle = "#666"
    ctx.font = "10px monospace"
    ctx.textAlign = "center"
    ctx.fillText("0", padding, padding + h + 15)
    ctx.fillText("1", padding + w, padding + h + 15)
    ctx.textAlign = "right"
    ctx.fillText("0", padding - 5, padding + h)
    ctx.fillText("1", padding - 5, padding)

    const visibleCount = Math.floor(points.length * progress)
    if (visibleCount < 1) return

    // Calculate max speed for normalization
    const speeds = points.map(p => Math.log(p.speed + 1))
    const maxLogSpeed = Math.max(...speeds, 0.1)

    // Draw path
    ctx.lineWidth = 3
    ctx.lineCap = "round"
    ctx.lineJoin = "round"

    for (let i = 1; i < visibleCount; i++) {
      const x1 = padding + points[i - 1].nx * w
      const y1 = padding + points[i - 1].ny * h
      const x2 = padding + points[i].nx * w
      const y2 = padding + points[i].ny * h

      const norm = speeds[i] / maxLogSpeed
      const hue = 240 - norm * 240
      ctx.strokeStyle = `hsl(${hue}, 100%, 50%)`
      ctx.beginPath()
      ctx.moveTo(x1, y1)
      ctx.lineTo(x2, y2)
      ctx.stroke()
    }

    // Draw point labels
    ctx.font = "9px monospace"
    ctx.textAlign = "left"
    for (let i = 0; i < visibleCount; i++) {
      const x = padding + points[i].nx * w
      const y = padding + points[i].ny * h

      // Point dot
      ctx.fillStyle = i === 0 ? "#4CAF50" : i === visibleCount - 1 ? "#f44336" : "#fff"
      ctx.beginPath()
      ctx.arc(x, y, i === 0 || i === visibleCount - 1 ? 6 : 3, 0, Math.PI * 2)
      ctx.fill()

      // Label
      ctx.fillStyle = "rgba(255,255,255,0.7)"
      ctx.fillText(
        `pos:(${points[i].nx.toFixed(2)},${points[i].ny.toFixed(2)}) t:${points[i].nt.toFixed(2)}`,
        x + 8, y - 4
      )
    }

    // Current cursor highlight during playback
    if (progress < 1 && visibleCount > 0) {
      const current = points[visibleCount - 1]
      const cx = padding + current.nx * w
      const cy = padding + current.ny * h

      ctx.strokeStyle = "#fff"
      ctx.lineWidth = 2
      ctx.beginPath()
      ctx.arc(cx, cy, 10, 0, Math.PI * 2)
      ctx.stroke()
    }
  }

  function startPlayback() {
    if (isPlaying) {
      stopPlayback()
      return
    }

    isPlaying = true
    playbackProgress = 0
    playbackStartTime = Date.now()

    const points = getSampledPoints()
    if (points.length === 0) return

    // Total playback duration based on actual motion duration scaled by speed
    const totalDuration = motionData[motionData.length - 1].timestamp
    const playDuration = totalDuration / playbackSpeed

    function animate() {
      const elapsed = Date.now() - playbackStartTime
      playbackProgress = Math.min(1, elapsed / playDuration)

      drawSimulation(playbackProgress)

      if (playbackProgress < 1) {
        animationFrameId = requestAnimationFrame(animate)
      } else {
        isPlaying = false
      }
    }

    animationFrameId = requestAnimationFrame(animate)
  }

  function stopPlayback() {
    isPlaying = false
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId)
      animationFrameId = 0
    }
  }

</script>

<div class="tool-page">
  <header class="tool-header">
    <h1>Motion Descriptor</h1>
    <p>마우스 움직임을 기록하고 LLM 프롬프트로 변환합니다.</p>
  </header>

  <div class="main-grid">
    <div class="left-column">
      <div class="canvas-container">
        <canvas
          bind:this={canvas}
          width="800"
          height="400"
          class:recording={isRecording}
          onmousedown={startRecording}
          onmousemove={recordMotion}
          onmouseup={stopRecording}
          onmouseleave={stopRecording}
        >
        </canvas>

        <div class="legend">
          <div class="legend-bar"></div>
          <div class="legend-labels">
            <span>느림</span>
            <span>중간</span>
            <span>빠름</span>
          </div>
        </div>

        <div class="canvas-info">
          {#if isRecording}
            <span class="status recording">🔴 기록 중... ({motionData.length} 포인트)</span>
          {:else if motionData.length > 0}
            <span class="status">✅ 기록 완료 ({motionData.length} 포인트)</span>
          {:else}
            <span class="status">클릭하여 기록 시작</span>
          {/if}

          <div class="canvas-actions">
            <button onclick={copyImage} disabled={isRecording || motionData.length < 2}>
              Copy Image
            </button>
            <button onclick={clearCanvas} disabled={isRecording}>
              Clear
            </button>
          </div>
        </div>

        {#if !isRecording && motionData.length > 1}
          <div class="controls-row">
            <div class="control-group">
              <label>샘플 수: {sampleCount}</label>
              <input type="range" min="5" max="150" step="1" bind:value={sampleCount} />
            </div>
          </div>
          <div class="simulation-controls">
            <button onclick={startPlayback}>
              {isPlaying ? "⏹ Stop" : "▶ Play"}
            </button>
            <button onclick={() => drawCanvas()}>
              원본 보기
            </button>
            <div class="speed-controls">
              <span>속도:</span>
              {#each [0.5, 1, 2] as speed}
                <button class:active={playbackSpeed === speed} onclick={() => { playbackSpeed = speed }}>
                  {speed}x
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>

    <div class="right-column">
      {#if !descriptorEn}
        <div class="empty-state">캔버스에 모션을 그리면 표시됩니다.</div>
      {/if}
      {#if descriptorEn}
        <div class="descriptor-output">
          <div class="output-header">
            <h3>Motion Descriptor (EN)</h3>
            <button onclick={() => copyDescriptor(descriptorEn)}>Copy</button>
          </div>
          <pre>{descriptorEn}</pre>
        </div>
        <div class="descriptor-output">
          <div class="output-header">
            <h3>Motion Descriptor (KO)</h3>
            <button onclick={() => copyDescriptor(descriptorKo)}>Copy</button>
          </div>
          <pre>{descriptorKo}</pre>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .tool-page {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }

  .tool-header {
    margin-bottom: 1.5rem;
  }

  .tool-header h1 {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
  }

  .tool-header p {
    margin: 0;
    color: #888;
  }

  .main-grid {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 1.5rem;
    align-items: start;
  }

  .left-column {
    min-width: 0;
  }

  .right-column {
    position: sticky;
    top: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .canvas-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  canvas {
    width: 100%;
    height: 400px;
    background: #1a1a1a;
    border: 2px solid #333;
    border-radius: 8px;
    cursor: crosshair;
    transition: border-color 0.2s;
  }

  canvas.recording {
    border-color: #0e639c;
    box-shadow: 0 0 20px rgba(14, 99, 156, 0.3);
  }

  canvas:hover {
    border-color: #444;
  }

  .legend {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .legend-bar {
    height: 12px;
    background: linear-gradient(to right,
      hsl(240, 100%, 50%),
      hsl(180, 100%, 50%),
      hsl(120, 100%, 50%),
      hsl(60, 100%, 50%),
      hsl(0, 100%, 50%)
    );
    border-radius: 4px;
    border: 1px solid #444;
  }

  .legend-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: #888;
    padding: 0 0.5rem;
  }

  .canvas-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .status {
    color: #888;
    font-size: 0.9rem;
  }

  .status.recording {
    color: #f44336;
    font-weight: 600;
  }

  .canvas-actions {
    display: flex;
    gap: 0.5rem;
  }

  .canvas-info button {
    padding: 0.5rem 1rem;
    background: #333;
    border: 1px solid #444;
    color: #ccc;
    cursor: pointer;
    border-radius: 4px;
  }

  .canvas-info button:hover:not(:disabled) {
    background: #444;
  }

  .canvas-info button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .controls-row {
    display: flex;
    align-items: center;
    gap: 2rem;
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
  }

  .control-group label {
    font-size: 0.9rem;
    color: #ccc;
    white-space: nowrap;
  }

  .control-group input[type="range"] {
    width: 100%;
    accent-color: #0e639c;
  }

  .descriptor-output {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1rem;
  }

  .output-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .output-header h3 {
    margin: 0;
    font-size: 0.95rem;
    color: #0e639c;
  }

  .output-header button {
    padding: 0.35rem 0.75rem;
    background: #0e639c;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.8rem;
  }

  .output-header button:hover {
    background: #1177bb;
  }

  pre {
    margin: 0;
    padding: 0.75rem;
    background: #0d0d0d;
    border: 1px solid #222;
    border-radius: 4px;
    color: #e0e0e0;
    font-family: monospace;
    font-size: 0.8rem;
    white-space: pre-wrap;
    word-wrap: break-word;
    overflow-x: auto;
    max-height: 300px;
    overflow-y: auto;
  }

  .simulation-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem;
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 4px;
  }

  .simulation-controls button {
    padding: 0.4rem 1rem;
    background: #0e639c;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .simulation-controls button:hover {
    background: #1177bb;
  }

  .speed-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #888;
    font-size: 0.85rem;
  }

  .speed-controls button {
    padding: 0.25rem 0.6rem;
    background: #333;
    border: 1px solid #444;
    color: #ccc;
    cursor: pointer;
    border-radius: 3px;
    font-size: 0.8rem;
  }

  .speed-controls button.active {
    background: #0e639c;
    border-color: #0e639c;
    color: #fff;
  }

  .speed-controls button:hover:not(.active) {
    background: #444;
  }

  .empty-state {
    color: #555;
    font-size: 0.9rem;
    text-align: center;
    padding: 2rem 1rem;
    border: 1px dashed #333;
    border-radius: 8px;
  }
</style>
