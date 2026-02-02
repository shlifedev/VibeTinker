<script lang="ts">
  interface CellKey {
    row: number
    col: number
  }

  let spriteImage = $state<HTMLImageElement | null>(null)
  let imageFile = $state<File | null>(null)
  let tileWidth = $state(32)
  let tileHeight = $state(32)
  let gridRows = $derived(spriteImage ? Math.max(1, Math.floor(spriteImage.height / tileHeight)) : 1)
  let gridCols = $derived(spriteImage ? Math.max(1, Math.floor(spriteImage.width / tileWidth)) : 1)
  let cellDescriptions = $state<Map<string, string>>(new Map())
  let selectedCells = $state<Set<string>>(new Set())
  let selectionMode = $state<"single" | "multi">("single")
  let showModal = $state(false)
  let currentCell = $state<CellKey | null>(null)
  let customTags = $state<string[]>([])
  let outputFormat = $state<"rowcol" | "index">("rowcol")
  let canvas = $state<HTMLCanvasElement | null>(null)
  let errorMessage = $state<string | null>(null)

  function showError(message: string) {
    errorMessage = message
    setTimeout(() => { errorMessage = null }, 4000)
  }

  function validateTileSize(value: number, name: string): number {
    if (value < 1) {
      showError(`${name}은(는) 1 이상이어야 합니다.`)
      return 1
    }
    if (spriteImage) {
      const maxSize = name === "타일 가로" ? spriteImage.width : spriteImage.height
      if (value > maxSize) {
        showError(`${name}은(는) ${maxSize} 이하여야 합니다.`)
        return maxSize
      }
    }
    return value
  }

  // Auto-redraw when tile dimensions change
  $effect(() => {
    if ((tileWidth || tileHeight) && canvas) {
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

  const PRESET_TAGS = [
    // 지형
    "벽(위)", "벽(아래)", "벽(좌)", "벽(우)", "바닥", "천장",
    // 구조물
    "문", "창문", "계단", "사다리",
    // 자연
    "풀", "나무", "물", "돌",
    // 상호작용
    "상자", "스위치", "레버",
    // 캐릭터
    "idle", "walk", "run", "jump", "attack"
  ]

  const CUSTOM_TAGS_KEY = "sprite-sheet-describer-custom-tags"

  function loadCustomTags() {
    const stored = localStorage.getItem(CUSTOM_TAGS_KEY)
    if (stored) {
      try {
        customTags = JSON.parse(stored)
      } catch {
        customTags = []
      }
    }
  }

  function saveCustomTags() {
    localStorage.setItem(CUSTOM_TAGS_KEY, JSON.stringify(customTags))
  }

  // Load on mount
  $effect(() => {
    loadCustomTags()
  })

  let showAddTagModal = $state(false)
  let newTagName = $state("")

  function addCustomTag() {
    const tag = newTagName.trim()
    if (tag && !customTags.includes(tag) && !PRESET_TAGS.includes(tag)) {
      customTags = [...customTags, tag]
      saveCustomTags()
      newTagName = ""
      showAddTagModal = false
    }
  }

  function removeCustomTag(tag: string) {
    customTags = customTags.filter(t => t !== tag)
    saveCustomTags()
  }

  function applyTag(tag: string) {
    if (selectionMode === "single" && currentCell) {
      const key = cellKey(currentCell.row, currentCell.col)
      cellDescriptions.set(key, tag)
      cellDescriptions = new Map(cellDescriptions)
      drawGrid()
    } else if (selectionMode === "multi" && selectedCells.size > 0) {
      selectedCells.forEach(key => {
        cellDescriptions.set(key, tag)
      })
      cellDescriptions = new Map(cellDescriptions)
      selectedCells = new Set()
      drawGrid()
    }
  }

  function generateJSON(): string {
    const cells = Array.from(cellDescriptions.entries()).map(([key, description]) => {
      const { row, col } = parseKey(key)

      if (outputFormat === "index") {
        return {
          index: cellToIndex(row, col),
          description
        }
      } else {
        return {
          row,
          col,
          description
        }
      }
    })

    const output = {
      gridSize: { rows: gridRows, cols: gridCols },
      format: outputFormat,
      cells
    }

    return JSON.stringify(output, null, 2)
  }

  function generateTextList(): string {
    const lines = Array.from(cellDescriptions.entries())
      .map(([key, description]) => {
        const { row, col } = parseKey(key)

        if (outputFormat === "index") {
          const index = cellToIndex(row, col)
          return `[${index}] ${description}`
        } else {
          return `[${row},${col}] ${description}`
        }
      })
      .sort()

    return lines.join("\n")
  }

  let jsonOutput = $derived(cellDescriptions.size > 0 ? generateJSON() : "")
  let textOutput = $derived(cellDescriptions.size > 0 ? generateTextList() : "")

  let copyFeedback = $state<string | null>(null)

  async function copyToClipboard(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text)
      copyFeedback = label
      setTimeout(() => { copyFeedback = null }, 2000)
    } catch (err) {
      console.error("Failed to copy:", err)
    }
  }

  interface ProjectData {
    imageName: string
    imageDataUrl: string
    gridSize: { rows: number, cols: number, tileWidth: number, tileHeight: number }
    descriptions: Record<string, string>
    customTags: string[]
  }

  async function saveProject() {
    if (!spriteImage || !imageFile) return

    const descriptions: Record<string, string> = {}
    cellDescriptions.forEach((value, key) => {
      descriptions[key] = value
    })

    // Convert image to data URL
    const canvas = document.createElement("canvas")
    canvas.width = spriteImage.width
    canvas.height = spriteImage.height
    const ctx = canvas.getContext("2d")
    if (!ctx) return
    ctx.drawImage(spriteImage, 0, 0)
    const imageDataUrl = canvas.toDataURL("image/png")

    const projectData: ProjectData = {
      imageName: imageFile.name,
      imageDataUrl,
      gridSize: { rows: gridRows, cols: gridCols, tileWidth, tileHeight },
      descriptions,
      customTags
    }

    const blob = new Blob([JSON.stringify(projectData, null, 2)], {
      type: "application/json"
    })

    const url = URL.createObjectURL(blob)
    const a = document.createElement("a")
    a.href = url
    a.download = `sprite-sheet-project-${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(url)
  }

  function handleProjectLoad(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    const reader = new FileReader()

    reader.onload = (evt) => {
      try {
        const projectData: ProjectData = JSON.parse(evt.target?.result as string)

        // Load image
        const img = new Image()
        img.onload = () => {
          spriteImage = img
          imageFile = new File([projectData.imageDataUrl], projectData.imageName, { type: "image/png" })
          tileWidth = projectData.gridSize.tileWidth || 32
          tileHeight = projectData.gridSize.tileHeight || 32

          // Load descriptions
          cellDescriptions.clear()
          Object.entries(projectData.descriptions).forEach(([key, value]) => {
            cellDescriptions.set(key, value)
          })
          cellDescriptions = new Map(cellDescriptions)

          // Load custom tags
          customTags = projectData.customTags || []
          saveCustomTags()

          drawGrid()
        }
        img.src = projectData.imageDataUrl
      } catch (err) {
        console.error("Failed to load project:", err)
        alert("프로젝트 파일을 불러올 수 없습니다.")
      }
    }

    reader.readAsText(file)
  }

  function resetAll() {
    if (!confirm("모든 데이터를 초기화하시겠습니까?")) return

    spriteImage = null
    imageFile = null
    cellDescriptions = new Map()
    selectedCells = new Set()
    tileWidth = 32
    tileHeight = 32
    if (canvas) {
      const ctx = canvas.getContext("2d")
      if (ctx) {
        ctx.clearRect(0, 0, canvas.width, canvas.height)
      }
    }
  }

  function clearDescriptions() {
    if (!confirm("모든 설명을 삭제하시겠습니까?")) return

    cellDescriptions = new Map()
    selectedCells = new Set()
    drawGrid()
  }

  function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement
    const file = input.files?.[0]
    if (!file) return

    if (!file.type.startsWith("image/")) {
      showError("이미지 파일만 선택할 수 있습니다.")
      return
    }

    if (file.size > 10 * 1024 * 1024) {
      showError("파일 크기는 10MB 이하여야 합니다.")
      return
    }

    imageFile = file
    const reader = new FileReader()

    reader.onerror = () => {
      showError("이미지를 불러올 수 없습니다.")
    }

    reader.onload = (evt) => {
      const img = new Image()

      img.onerror = () => {
        showError("이미지를 로드할 수 없습니다.")
      }

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
    const maxWidth = 600
    const maxHeight = 450

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

      // Green border instead of blue overlay
      ctx.strokeStyle = "#4CAF50"
      ctx.lineWidth = 2
      ctx.strokeRect(col * cellWidth + 1, row * cellHeight + 1, cellWidth - 2, cellHeight - 2)

      // Text with dark background pill
      const fontSize = Math.min(12, cellHeight * 0.25)
      ctx.font = `${fontSize}px sans-serif`
      ctx.textAlign = "center"
      ctx.textBaseline = "middle"

      const maxChars = Math.max(2, Math.floor(cellWidth / 7))
      const displayText = desc.length > maxChars ? desc.slice(0, maxChars - 1) + "…" : desc
      const textWidth = ctx.measureText(displayText).width
      const cx = col * cellWidth + cellWidth / 2
      const cy = row * cellHeight + cellHeight / 2

      // Dark background behind text
      const padX = 4
      const padY = 2
      ctx.fillStyle = "rgba(0, 0, 0, 0.7)"
      ctx.beginPath()
      ctx.roundRect(cx - textWidth / 2 - padX, cy - fontSize / 2 - padY, textWidth + padX * 2, fontSize + padY * 2, 3)
      ctx.fill()

      // White text
      ctx.fillStyle = "#fff"
      ctx.fillText(displayText, cx, cy)
      ctx.textAlign = "start"
      ctx.textBaseline = "alphabetic"
    })

    // Highlight selected cells (multi mode)
    selectedCells.forEach(key => {
      const { row, col } = parseKey(key)
      ctx.strokeStyle = "#0e639c"
      ctx.lineWidth = 3
      ctx.strokeRect(col * cellWidth, row * cellHeight, cellWidth, cellHeight)
    })
  }

  let lastSelectedCell = $state<CellKey | null>(null)

  function selectRectangle(start: CellKey | null, end: CellKey) {
    if (!start) {
      const key = cellKey(end.row, end.col)
      selectedCells.add(key)
      selectedCells = new Set(selectedCells)
      return
    }

    const minRow = Math.min(start.row, end.row)
    const maxRow = Math.max(start.row, end.row)
    const minCol = Math.min(start.col, end.col)
    const maxCol = Math.max(start.col, end.col)

    for (let r = minRow; r <= maxRow; r++) {
      for (let c = minCol; c <= maxCol; c++) {
        selectedCells.add(cellKey(r, c))
      }
    }

    selectedCells = new Set(selectedCells)
  }

  function handleCanvasClick(e: MouseEvent) {
    if (!canvas || !spriteImage) return

    const rect = canvas.getBoundingClientRect()
    const scaleX = canvas.width / rect.width
    const scaleY = canvas.height / rect.height
    const x = (e.clientX - rect.left) * scaleX
    const y = (e.clientY - rect.top) * scaleY

    const cellWidth = canvas.width / gridCols
    const cellHeight = canvas.height / gridRows

    const col = Math.floor(x / cellWidth)
    const row = Math.floor(y / cellHeight)

    if (row < 0 || row >= gridRows || col < 0 || col >= gridCols) return

    const key = cellKey(row, col)

    if (selectionMode === "single") {
      currentCell = { row, col }
      showModal = true
    } else {
      // Multi mode
      if (e.shiftKey && lastSelectedCell) {
        // Rectangle selection
        selectRectangle(lastSelectedCell, { row, col })
      } else if (e.ctrlKey || e.metaKey) {
        // Toggle selection
        if (selectedCells.has(key)) {
          selectedCells.delete(key)
        } else {
          selectedCells.add(key)
        }
        selectedCells = new Set(selectedCells)
      } else {
        // Single click in multi mode - clear and select one
        selectedCells.clear()
        selectedCells.add(key)
        selectedCells = new Set(selectedCells)
      }
      lastSelectedCell = { row, col }
      drawGrid()
    }
  }

  let modalDescription = $state("")

  function openModal() {
    if (!currentCell) return
    const key = cellKey(currentCell.row, currentCell.col)
    modalDescription = cellDescriptions.get(key) || ""
  }

  function saveDescription() {
    if (!currentCell) return
    const key = cellKey(currentCell.row, currentCell.col)

    if (modalDescription.trim()) {
      cellDescriptions.set(key, modalDescription.trim())
    } else {
      cellDescriptions.delete(key)
    }

    cellDescriptions = new Map(cellDescriptions)
    showModal = false
    modalDescription = ""
    drawGrid()
  }

  function deleteDescription() {
    if (!currentCell) return
    const key = cellKey(currentCell.row, currentCell.col)
    cellDescriptions.delete(key)
    cellDescriptions = new Map(cellDescriptions)
    showModal = false
    modalDescription = ""
    drawGrid()
  }

  function closeModal() {
    showModal = false
    modalDescription = ""
    currentCell = null
  }

  $effect(() => {
    if (showModal && currentCell) {
      openModal()
    }
  })
</script>

<div class="tool-page">
  <header class="tool-header">
    <h1>Sprite Sheet Describer</h1>
    <p>스프라이트 시트를 그리드로 나누고 각 셀에 설명을 추가합니다.</p>
  </header>

  {#if errorMessage}
    <div class="error-toast">
      ⚠️ {errorMessage}
    </div>
  {/if}

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
      <h3>타일 크기 (px)</h3>
      <div class="grid-inputs">
        <label>
          가로:
          <input
            type="number"
            min="1"
            bind:value={tileWidth}
            onchange={() => {
              tileWidth = validateTileSize(tileWidth, "타일 가로")
            }}
          />
        </label>
        <label>
          세로:
          <input
            type="number"
            min="1"
            bind:value={tileHeight}
            onchange={() => {
              tileHeight = validateTileSize(tileHeight, "타일 세로")
            }}
          />
        </label>
      </div>
      {#if spriteImage}
        <span class="grid-info">{gridCols}×{gridRows} 그리드 ({spriteImage.width}×{spriteImage.height}px)</span>
      {/if}
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
      {#if selectionMode === "multi"}
        <div class="multi-info">
          {#if selectedCells.size > 0}
            <span class="selection-count">{selectedCells.size}개 셀 선택됨</span>
            <button class="clear-btn" onclick={() => { selectedCells = new Set(); drawGrid() }}>
              선택 해제
            </button>
          {:else}
            <span class="hint">클릭: 단일 선택 | Ctrl+클릭: 추가/제거 | Shift+클릭: 영역 선택</span>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  {#if spriteImage}
    <div class="tags-toolbar">
      <div class="toolbar-header">
        <h3>프리셋 태그</h3>
        <button class="add-tag-btn" onclick={() => showAddTagModal = true}>
          + 커스텀 태그 추가
        </button>
      </div>

      <div class="tags-toolbar-content">
        {#each PRESET_TAGS as tag}
          <button
            class="tag-btn preset-tag"
            onclick={() => applyTag(tag)}
            disabled={selectionMode === "single" || selectedCells.size === 0}
          >
            {tag}
          </button>
        {/each}
      </div>

      {#if customTags.length > 0}
        <div class="custom-tags-section">
          <h4>커스텀 태그</h4>
          <div class="tags-toolbar-content">
            {#each customTags as tag}
              <button
                class="tag-btn custom-tag"
                onclick={() => applyTag(tag)}
                disabled={selectionMode === "single" || selectedCells.size === 0}
              >
                {tag}
                <span class="remove-tag" onclick={(e) => { e.stopPropagation(); removeCustomTag(tag) }}>×</span>
              </button>
            {/each}
          </div>
        </div>
      {/if}

      {#if selectionMode === "multi" && selectedCells.size === 0}
        <p class="toolbar-hint">멀티 선택 모드에서 셀을 선택한 후 태그를 클릭하세요</p>
      {/if}
    </div>

    <div class="canvas-container">
      <canvas
        bind:this={canvas}
        width="600"
        height="450"
        class:multi-mode={selectionMode === "multi"}
        onclick={handleCanvasClick}
      >
      </canvas>
    </div>
  {:else}
    <div class="empty-state">
      <p>이미지를 불러와서 시작하세요</p>
    </div>
  {/if}

  {#if showModal && currentCell}
    <div class="modal-backdrop" onclick={closeModal}>
      <div class="modal" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h3>
            셀 설명 편집
            <span class="cell-info">
              (행: {currentCell.row}, 열: {currentCell.col},
              인덱스: {cellToIndex(currentCell.row, currentCell.col)})
            </span>
          </h3>
          <button class="close-btn" onclick={closeModal}>×</button>
        </div>

        <div class="modal-body">
          <textarea
            bind:value={modalDescription}
            placeholder="셀 설명을 입력하세요..."
            rows="4"
          ></textarea>

          <div class="quick-tags">
            <span class="tags-label">빠른 선택:</span>
            <div class="tags-grid">
              {#each PRESET_TAGS as tag}
                <button
                  class="tag-btn preset-tag"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
              {#each customTags as tag}
                <button
                  class="tag-btn custom-tag"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" onclick={closeModal}>취소</button>
          <button class="btn-danger" onclick={deleteDescription}>삭제</button>
          <button class="btn-primary" onclick={saveDescription}>저장</button>
        </div>
      </div>
    </div>
  {/if}

  {#if cellDescriptions.size > 0}
    <div class="output-section">
      <div class="output-header">
        <h3>출력</h3>
        <div class="format-toggle">
          <label>
            <input
              type="radio"
              value="rowcol"
              bind:group={outputFormat}
            />
            Row/Col
          </label>
          <label>
            <input
              type="radio"
              value="index"
              bind:group={outputFormat}
            />
            Index
          </label>
        </div>
      </div>

      <div class="output-panels">
        <div class="output-panel">
          <div class="panel-header">
            <h4>JSON</h4>
            <button onclick={() => copyToClipboard(jsonOutput, "JSON")}>
              {copyFeedback === "JSON" ? "복사됨!" : "Copy"}
            </button>
          </div>
          <pre>{jsonOutput}</pre>
        </div>

        <div class="output-panel">
          <div class="panel-header">
            <h4>텍스트 리스트</h4>
            <button onclick={() => copyToClipboard(textOutput, "텍스트")}>
              {copyFeedback === "텍스트" ? "복사됨!" : "Copy"}
            </button>
          </div>
          <pre>{textOutput}</pre>
        </div>
      </div>
    </div>
  {/if}

  <div class="project-management">
    <h3>프로젝트 관리</h3>
    <div class="management-actions">
      <button class="mgmt-btn save" onclick={saveProject} disabled={!spriteImage}>
        💾 프로젝트 저장
      </button>
      <label class="mgmt-btn load">
        📂 프로젝트 불러오기
        <input
          type="file"
          accept="application/json"
          onchange={handleProjectLoad}
          style="display: none;"
        />
      </label>
      <button class="mgmt-btn clear" onclick={clearDescriptions} disabled={cellDescriptions.size === 0}>
        🗑️ 설명만 지우기
      </button>
      <button class="mgmt-btn reset" onclick={resetAll} disabled={!spriteImage}>
        ♻️ 전체 초기화
      </button>
    </div>
  </div>

  {#if showAddTagModal}
    <div class="modal-backdrop" onclick={() => showAddTagModal = false}>
      <div class="modal small-modal" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h3>커스텀 태그 추가</h3>
          <button class="close-btn" onclick={() => showAddTagModal = false}>×</button>
        </div>

        <div class="modal-body">
          <input
            type="text"
            bind:value={newTagName}
            placeholder="태그 이름을 입력하세요..."
            onkeydown={(e) => e.key === "Enter" && addCustomTag()}
          />
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" onclick={() => showAddTagModal = false}>취소</button>
          <button class="btn-primary" onclick={addCustomTag}>추가</button>
        </div>
      </div>
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

  .grid-info {
    display: block;
    margin-top: 0.5rem;
    font-size: 0.8rem;
    color: #888;
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

  .multi-info {
    margin-top: 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .hint {
    font-size: 0.8rem;
    color: #666;
  }

  .clear-btn {
    padding: 0.25rem 0.75rem;
    background: #d32f2f;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.8rem;
  }

  .clear-btn:hover {
    background: #f44336;
  }

  .canvas-container {
    margin-bottom: 2rem;
  }

  canvas {
    width: 100%;
    max-width: 600px;
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

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: #1e1e1e;
    border: 1px solid #444;
    border-radius: 8px;
    width: 90%;
    max-width: 500px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1.5rem;
    border-bottom: 1px solid #333;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #0e639c;
  }

  .cell-info {
    display: block;
    font-size: 0.85rem;
    color: #888;
    font-weight: normal;
    margin-top: 0.25rem;
  }

  .close-btn {
    background: none;
    border: none;
    color: #888;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 30px;
    height: 30px;
    line-height: 1;
  }

  .close-btn:hover {
    color: #fff;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .modal-body textarea {
    width: 100%;
    padding: 0.75rem;
    background: #0d0d0d;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
    font-family: inherit;
    font-size: 0.9rem;
    resize: vertical;
  }

  .modal-body textarea:focus {
    outline: none;
    border-color: #0e639c;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 1.5rem;
    border-top: 1px solid #333;
  }

  .modal-footer button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .btn-primary {
    background: #0e639c;
    color: #fff;
  }

  .btn-primary:hover {
    background: #1177bb;
  }

  .btn-secondary {
    background: #333;
    color: #ccc;
  }

  .btn-secondary:hover {
    background: #444;
  }

  .btn-danger {
    background: #d32f2f;
    color: #fff;
  }

  .btn-danger:hover {
    background: #f44336;
  }

  .quick-tags {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #333;
  }

  .tags-label {
    display: block;
    font-size: 0.85rem;
    color: #888;
    margin-bottom: 0.5rem;
  }

  .tags-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag-btn {
    padding: 0.4rem 0.8rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.2s;
  }

  .preset-tag {
    background: #0e639c;
    color: #fff;
  }

  .preset-tag:hover:not(:disabled) {
    background: #1177bb;
    transform: translateY(-1px);
  }

  .tag-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .tags-toolbar {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .tags-toolbar h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: #0e639c;
  }

  .tags-toolbar-content {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .toolbar-hint {
    margin: 1rem 0 0 0;
    font-size: 0.85rem;
    color: #666;
  }

  .toolbar-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .add-tag-btn {
    padding: 0.4rem 0.8rem;
    background: #4CAF50;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .add-tag-btn:hover {
    background: #66BB6A;
  }

  .custom-tags-section {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid #333;
  }

  .custom-tags-section h4 {
    margin: 0 0 1rem 0;
    font-size: 0.9rem;
    color: #4CAF50;
  }

  .custom-tag {
    background: #4CAF50;
    color: #fff;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .custom-tag:hover:not(:disabled) {
    background: #66BB6A;
  }

  .remove-tag {
    font-size: 1.2rem;
    line-height: 1;
    opacity: 0.7;
  }

  .remove-tag:hover {
    opacity: 1;
  }

  .small-modal {
    max-width: 400px;
  }

  .modal-body input[type="text"] {
    width: 100%;
    padding: 0.75rem;
    background: #0d0d0d;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .modal-body input[type="text"]:focus {
    outline: none;
    border-color: #4CAF50;
  }

  .output-section {
    margin-top: 2rem;
  }

  .output-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .output-header h3 {
    margin: 0;
    font-size: 1.2rem;
    color: #0e639c;
  }

  .format-toggle {
    display: flex;
    gap: 1rem;
  }

  .format-toggle label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: #ccc;
    cursor: pointer;
  }

  .format-toggle input[type="radio"] {
    cursor: pointer;
    accent-color: #0e639c;
  }

  .output-panels {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .output-panel {
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: #252526;
    border-bottom: 1px solid #333;
  }

  .panel-header h4 {
    margin: 0;
    font-size: 0.9rem;
    color: #0e639c;
  }

  .panel-header button {
    padding: 0.4rem 0.8rem;
    background: #0e639c;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.85rem;
  }

  .panel-header button:hover {
    background: #1177bb;
  }

  .output-panel pre {
    margin: 0;
    padding: 1rem;
    background: #0d0d0d;
    color: #e0e0e0;
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.85rem;
    white-space: pre-wrap;
    word-wrap: break-word;
    overflow-x: auto;
    max-height: 400px;
    overflow-y: auto;
  }

  .project-management {
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 2px solid #333;
  }

  .project-management h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: #888;
  }

  .management-actions {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .mgmt-btn {
    padding: 0.75rem 1.25rem;
    border: 1px solid #444;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .mgmt-btn.save {
    background: #0e639c;
    color: #fff;
  }

  .mgmt-btn.save:hover:not(:disabled) {
    background: #1177bb;
  }

  .mgmt-btn.load {
    background: #4CAF50;
    color: #fff;
  }

  .mgmt-btn.load:hover {
    background: #66BB6A;
  }

  .mgmt-btn.clear {
    background: #ff9800;
    color: #fff;
  }

  .mgmt-btn.clear:hover:not(:disabled) {
    background: #ffa726;
  }

  .mgmt-btn.reset {
    background: #d32f2f;
    color: #fff;
  }

  .mgmt-btn.reset:hover:not(:disabled) {
    background: #f44336;
  }

  .mgmt-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .error-toast {
    position: fixed;
    top: 2rem;
    right: 2rem;
    background: #d32f2f;
    color: #fff;
    padding: 1rem 1.5rem;
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    z-index: 2000;
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
</style>
