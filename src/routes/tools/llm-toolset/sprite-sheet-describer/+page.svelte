<script lang="ts">
  interface CellKey {
    row: number
    col: number
  }

  let spriteImage = $state<HTMLImageElement | null>(null)
  let imageFile = $state<File | null>(null)
  let gridRows = $state(4)
  let gridCols = $state(4)
  let cellDescriptions = $state<Map<string, string>>(new Map())
  let selectedCells = $state<Set<string>>(new Set())
  let selectionMode = $state<"single" | "multi">("single")
  let showModal = $state(false)
  let currentCell = $state<CellKey | null>(null)
  let customTags = $state<string[]>([])
  let outputFormat = $state<"rowcol" | "index">("rowcol")
  let canvas = $state<HTMLCanvasElement | null>(null)

  // Auto-redraw when grid dimensions change
  $effect(() => {
    if (gridRows || gridCols || canvas) {
      drawGrid()
    }
  })

  function cellKey(row: number, col: number): string {
    return `${row}-${col}`
  }

  function parseKey(key: string): CellKey {
    const [row, col] = key.split("-").map(Number)
    return { row, col }
  }

  function cellToIndex(row: number, col: number): number {
    return row * gridCols + col
  }

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    imageFile = file
    const reader = new FileReader()

    reader.onload = (evt) => {
      const img = new Image()
      img.onload = () => {
        spriteImage = img
        cellDescriptions = new Map()
        selectedCells = new Set()
        drawGrid()
      }
      img.src = evt.target?.result as string
    }

    reader.readAsDataURL(file)
  }

  function drawGrid() {
    if (!canvas || !spriteImage) return

    const ctx = canvas.getContext("2d")
    if (!ctx) return

    // Preserve aspect ratio
    const aspectRatio = spriteImage.width / spriteImage.height
    const maxWidth = 800
    const maxHeight = 600

    let canvasWidth = maxWidth
    let canvasHeight = maxWidth / aspectRatio

    if (canvasHeight > maxHeight) {
      canvasHeight = maxHeight
      canvasWidth = maxHeight * aspectRatio
    }

    canvas.width = canvasWidth
    canvas.height = canvasHeight

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    // Draw sprite sheet image (now properly sized)
    ctx.drawImage(spriteImage, 0, 0, canvas.width, canvas.height)

    const cellWidth = canvas.width / gridCols
    const cellHeight = canvas.height / gridRows

    // Draw grid lines
    ctx.strokeStyle = "#ffffff40"
    ctx.lineWidth = 1

    for (let i = 0; i <= gridRows; i++) {
      ctx.beginPath()
      ctx.moveTo(0, i * cellHeight)
      ctx.lineTo(canvas.width, i * cellHeight)
      ctx.stroke()
    }

    for (let i = 0; i <= gridCols; i++) {
      ctx.beginPath()
      ctx.moveTo(i * cellWidth, 0)
      ctx.lineTo(i * cellWidth, canvas.height)
      ctx.stroke()
    }

    // Highlight cells with descriptions
    cellDescriptions.forEach((desc, key) => {
      const { row, col } = parseKey(key)
      ctx.fillStyle = "rgba(14, 99, 156, 0.3)"
      ctx.fillRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)

      // Draw checkmark for cells with descriptions
      ctx.fillStyle = "#4CAF50"
      ctx.font = "16px sans-serif"
      ctx.fillText("✓", col * cellWidth + 5, row * cellHeight + 20)
    })

    // Highlight selected cells (multi mode)
    selectedCells.forEach(key => {
      const { row, col } = parseKey(key)
      ctx.strokeStyle = "#0e639c"
      ctx.lineWidth = 3
      ctx.strokeRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
    })
  }
</script>

<div class="tool-page">
  <header class="tool-header">
    <h1>Sprite Sheet Describer</h1>
    <p>스프라이트 시트를 그리드로 나누고 각 셀에 설명을 추가합니다.</p>
  </header>

  <div class="instructions">
    <h3>사용 방법</h3>
    <ol>
      <li>스프라이트 시트 이미지를 불러옵니다</li>
      <li>그리드 크기를 설정합니다 (행/열)</li>
      <li>셀을 클릭하여 설명을 입력하거나 프리셋 태그를 사용합니다</li>
      <li>JSON 또는 텍스트 리스트 형식으로 내보냅니다</li>
    </ol>
  </div>

  <div class="control-panel">
    <div class="control-section">
      <h3>이미지 불러오기</h3>
      <input
        type="file"
        accept="image/*"
        onchange={handleFileSelect}
        class="file-input"
      />
      {#if imageFile}
        <span class="file-name">{imageFile.name}</span>
      {/if}
    </div>

    <div class="control-section">
      <h3>그리드 설정</h3>
      <div class="grid-inputs">
        <label>
          행:
          <input
            type="number"
            min="1"
            max="50"
            bind:value={gridRows}
          />
        </label>
        <label>
          열:
          <input
            type="number"
            min="1"
            max="50"
            bind:value={gridCols}
          />
        </label>
      </div>
    </div>

    <div class="control-section">
      <h3>선택 모드</h3>
      <div class="mode-toggle">
        <button
          class:active={selectionMode === "single"}
          onclick={() => { selectionMode = "single"; selectedCells = new Set() }}
        >
          단일 선택
        </button>
        <button
          class:active={selectionMode === "multi"}
          onclick={() => selectionMode = "multi"}
        >
          멀티 선택
        </button>
      </div>
      {#if selectionMode === "multi" && selectedCells.size > 0}
        <span class="selection-count">{selectedCells.size}개 셀 선택됨</span>
      {/if}
    </div>
  </div>

  {#if spriteImage}
    <div class="canvas-container">
      <canvas
        bind:this={canvas}
        width="800"
        height="600"
        class:multi-mode={selectionMode === "multi"}
      >
      </canvas>
    </div>
  {:else}
    <div class="empty-state">
      <p>이미지를 불러와서 시작하세요</p>
    </div>
  {/if}
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

  .instructions {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .instructions h3 {
    margin-top: 0;
    font-size: 1rem;
    color: #0e639c;
  }

  .instructions ol {
    margin: 0.5rem 0 0 1.5rem;
    padding: 0;
  }

  .instructions li {
    margin: 0.5rem 0;
    color: #ccc;
  }

  .control-panel {
    display: flex;
    gap: 2rem;
    margin-bottom: 2rem;
    flex-wrap: wrap;
  }

  .control-section {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1rem;
  }

  .control-section h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.9rem;
    color: #0e639c;
  }

  .file-input {
    display: block;
    color: #ccc;
    font-size: 0.9rem;
  }

  .file-name {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.85rem;
    color: #888;
  }

  .grid-inputs {
    display: flex;
    gap: 1rem;
  }

  .grid-inputs label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: #ccc;
  }

  .grid-inputs input {
    width: 60px;
    padding: 0.4rem;
    background: #0d0d0d;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
  }

  .mode-toggle {
    display: flex;
    gap: 0.5rem;
  }

  .mode-toggle button {
    padding: 0.5rem 1rem;
    background: #333;
    border: 1px solid #444;
    color: #ccc;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .mode-toggle button:hover {
    background: #444;
  }

  .mode-toggle button.active {
    background: #0e639c;
    border-color: #0e639c;
    color: #fff;
  }

  .selection-count {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.85rem;
    color: #4CAF50;
  }

  .canvas-container {
    margin-bottom: 2rem;
  }

  canvas {
    width: 100%;
    max-width: 800px;
    height: auto;
    background: #1a1a1a;
    border: 2px solid #333;
    border-radius: 8px;
    cursor: pointer;
  }

  canvas.multi-mode {
    cursor: crosshair;
  }

  .empty-state {
    padding: 4rem 2rem;
    text-align: center;
    background: #1a1a1a;
    border: 2px dashed #333;
    border-radius: 8px;
    margin-bottom: 2rem;
  }

  .empty-state p {
    margin: 0;
    color: #666;
    font-size: 1.1rem;
  }
</style>
