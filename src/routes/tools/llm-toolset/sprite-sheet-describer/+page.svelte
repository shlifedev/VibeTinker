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
    "벽(위)", "벽(아래)", "벽(좌)", "벽(우)", "바닥", "천장",
    "문", "창문", "계단", "사다리",
    "풀", "나무", "물", "돌",
    "상자", "스위치", "레버",
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
        return { index: cellToIndex(row, col), description }
      } else {
        return { row, col, description }
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

        const img = new Image()
        img.onload = () => {
          spriteImage = img
          imageFile = new File([projectData.imageDataUrl], projectData.imageName, { type: "image/png" })
          tileWidth = projectData.gridSize.tileWidth || 32
          tileHeight = projectData.gridSize.tileHeight || 32

          cellDescriptions.clear()
          Object.entries(projectData.descriptions).forEach(([key, value]) => {
            cellDescriptions.set(key, value)
          })
          cellDescriptions = new Map(cellDescriptions)

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

    ctx.clearRect(0, 0, canvas.width, canvas.height)
    ctx.drawImage(spriteImage, 0, 0, canvas.width, canvas.height)

    const cellWidth = canvas.width / gridCols
    const cellHeight = canvas.height / gridRows

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

    cellDescriptions.forEach((desc, key) => {
      const { row, col } = parseKey(key)

      ctx.strokeStyle = "#4CAF50"
      ctx.lineWidth = 2
      ctx.strokeRect(col * cellWidth + 1, row * cellHeight + 1, cellWidth - 2, cellHeight - 2)

      const fontSize = Math.min(12, cellHeight * 0.25)
      ctx.font = `${fontSize}px sans-serif`
      ctx.textAlign = "center"
      ctx.textBaseline = "middle"

      const maxChars = Math.max(2, Math.floor(cellWidth / 7))
      const displayText = desc.length > maxChars ? desc.slice(0, maxChars - 1) + "…" : desc
      const textWidth = ctx.measureText(displayText).width
      const cx = col * cellWidth + cellWidth / 2
      const cy = row * cellHeight + cellHeight / 2

      const padX = 4
      const padY = 2
      ctx.fillStyle = "rgba(0, 0, 0, 0.7)"
      ctx.beginPath()
      ctx.roundRect(cx - textWidth / 2 - padX, cy - fontSize / 2 - padY, textWidth + padX * 2, fontSize + padY * 2, 3)
      ctx.fill()

      ctx.fillStyle = "#fff"
      ctx.fillText(displayText, cx, cy)
      ctx.textAlign = "start"
      ctx.textBaseline = "alphabetic"
    })

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
      if (e.shiftKey && lastSelectedCell) {
        selectRectangle(lastSelectedCell, { row, col })
      } else if (e.ctrlKey || e.metaKey) {
        if (selectedCells.has(key)) {
          selectedCells.delete(key)
        } else {
          selectedCells.add(key)
        }
        selectedCells = new Set(selectedCells)
      } else {
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

<div class="p-8 max-w-[1400px] mx-auto">
  <header class="mb-6">
    <h1 class="text-2xl font-bold mb-2">Sprite Sheet Describer</h1>
    <p class="opacity-50">스프라이트 시트를 그리드로 나누고 각 셀에 설명을 추가합니다.</p>
  </header>

  {#if errorMessage}
    <div class="card preset-filled-error-500 p-4 fixed top-8 right-8 z-[2000] shadow-xl animate-pulse">
      ⚠️ {errorMessage}
    </div>
  {/if}

  <div class="flex gap-8 mb-8 flex-wrap">
    <div class="card preset-filled-surface-200-800 p-4">
      <h3 class="text-sm font-semibold text-primary-500 mb-3">이미지 불러오기</h3>
      <input
        type="file"
        accept="image/*"
        onchange={handleFileSelect}
        class="text-sm"
      />
      {#if imageFile}
        <span class="block mt-2 text-xs opacity-50">{imageFile.name}</span>
      {/if}
    </div>

    <div class="card preset-filled-surface-200-800 p-4">
      <h3 class="text-sm font-semibold text-primary-500 mb-3">타일 크기 (px)</h3>
      <div class="flex gap-4">
        <label class="flex items-center gap-2 text-sm">
          가로:
          <input
            type="number"
            min="1"
            bind:value={tileWidth}
            onchange={() => {
              tileWidth = validateTileSize(tileWidth, "타일 가로")
            }}
            class="input w-16 text-sm"
          />
        </label>
        <label class="flex items-center gap-2 text-sm">
          세로:
          <input
            type="number"
            min="1"
            bind:value={tileHeight}
            onchange={() => {
              tileHeight = validateTileSize(tileHeight, "타일 세로")
            }}
            class="input w-16 text-sm"
          />
        </label>
      </div>
      {#if spriteImage}
        <span class="block mt-2 text-xs opacity-50">{gridCols}×{gridRows} 그리드 ({spriteImage.width}×{spriteImage.height}px)</span>
      {/if}
    </div>

    <div class="card preset-filled-surface-200-800 p-4">
      <h3 class="text-sm font-semibold text-primary-500 mb-3">선택 모드</h3>
      <div class="flex gap-2">
        <button
          class="btn btn-sm {selectionMode === 'single' ? 'preset-filled-primary-500' : 'preset-tonal'}"
          onclick={() => { selectionMode = "single"; selectedCells = new Set() }}
        >
          단일 선택
        </button>
        <button
          class="btn btn-sm {selectionMode === 'multi' ? 'preset-filled-primary-500' : 'preset-tonal'}"
          onclick={() => selectionMode = "multi"}
        >
          멀티 선택
        </button>
      </div>
      {#if selectionMode === "multi"}
        <div class="mt-3 flex items-center gap-3 flex-wrap">
          {#if selectedCells.size > 0}
            <span class="text-xs text-success-500">{selectedCells.size}개 셀 선택됨</span>
            <button class="btn btn-sm preset-filled-error-500" onclick={() => { selectedCells = new Set(); drawGrid() }}>
              선택 해제
            </button>
          {:else}
            <span class="text-xs opacity-40">클릭: 단일 선택 | Ctrl+클릭: 추가/제거 | Shift+클릭: 영역 선택</span>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  {#if spriteImage}
    <div class="card preset-filled-surface-200-800 p-6 mb-8">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-sm font-semibold text-primary-500">프리셋 태그</h3>
        <button class="btn btn-sm preset-filled-success-500" onclick={() => showAddTagModal = true}>
          + 커스텀 태그 추가
        </button>
      </div>

      <div class="flex flex-wrap gap-2">
        {#each PRESET_TAGS as tag}
          <button
            class="btn btn-sm preset-filled-primary-500"
            onclick={() => applyTag(tag)}
            disabled={selectionMode === "single" || selectedCells.size === 0}
          >
            {tag}
          </button>
        {/each}
      </div>

      {#if customTags.length > 0}
        <div class="mt-4 pt-4 border-t border-surface-500">
          <h4 class="text-sm font-semibold text-success-500 mb-3">커스텀 태그</h4>
          <div class="flex flex-wrap gap-2">
            {#each customTags as tag}
              <button
                class="btn btn-sm preset-filled-success-500 flex items-center gap-2"
                onclick={() => applyTag(tag)}
                disabled={selectionMode === "single" || selectedCells.size === 0}
              >
                {tag}
                <span class="opacity-70 hover:opacity-100 text-lg leading-none" onclick={(e) => { e.stopPropagation(); removeCustomTag(tag) }}>×</span>
              </button>
            {/each}
          </div>
        </div>
      {/if}

      {#if selectionMode === "multi" && selectedCells.size === 0}
        <p class="mt-4 text-xs opacity-40">멀티 선택 모드에서 셀을 선택한 후 태그를 클릭하세요</p>
      {/if}
    </div>

    <div class="mb-8">
      <canvas
        bind:this={canvas}
        width="600"
        height="450"
        class="w-full max-w-[600px] h-auto bg-[#1a1a1a] border-2 border-surface-500 rounded-lg
          {selectionMode === 'multi' ? 'cursor-crosshair' : 'cursor-pointer'}"
        onclick={handleCanvasClick}
      >
      </canvas>
    </div>
  {:else}
    <div class="card preset-outlined-surface-200-800 p-16 text-center mb-8">
      <p class="opacity-40 text-lg">이미지를 불러와서 시작하세요</p>
    </div>
  {/if}

  {#if showModal && currentCell}
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-[1000]" onclick={closeModal}>
      <div class="card preset-filled-surface-200-800 w-[90%] max-w-[500px] shadow-2xl" onclick={(e) => e.stopPropagation()}>
        <div class="flex justify-between items-start p-6 border-b border-surface-500">
          <div>
            <h3 class="text-base font-semibold text-primary-500">셀 설명 편집</h3>
            <span class="text-xs opacity-50 mt-1 block">
              (행: {currentCell.row}, 열: {currentCell.col},
              인덱스: {cellToIndex(currentCell.row, currentCell.col)})
            </span>
          </div>
          <button class="btn-icon preset-tonal" onclick={closeModal}>×</button>
        </div>

        <div class="p-6">
          <textarea
            bind:value={modalDescription}
            placeholder="셀 설명을 입력하세요..."
            rows="4"
            class="textarea w-full text-sm"
          ></textarea>

          <div class="mt-4 pt-4 border-t border-surface-500">
            <span class="text-xs opacity-50 block mb-2">빠른 선택:</span>
            <div class="flex flex-wrap gap-2">
              {#each PRESET_TAGS as tag}
                <button
                  class="btn btn-sm preset-filled-primary-500"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
              {#each customTags as tag}
                <button
                  class="btn btn-sm preset-filled-success-500"
                  onclick={() => { modalDescription = tag }}
                >
                  {tag}
                </button>
              {/each}
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 p-6 border-t border-surface-500">
          <button class="btn preset-tonal" onclick={closeModal}>취소</button>
          <button class="btn preset-filled-error-500" onclick={deleteDescription}>삭제</button>
          <button class="btn preset-filled-primary-500" onclick={saveDescription}>저장</button>
        </div>
      </div>
    </div>
  {/if}

  {#if cellDescriptions.size > 0}
    <div class="mt-8">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-lg font-semibold text-primary-500">출력</h3>
        <div class="flex gap-4">
          <label class="flex items-center gap-2 text-sm cursor-pointer">
            <input
              type="radio"
              value="rowcol"
              bind:group={outputFormat}
              class="accent-primary-500"
            />
            Row/Col
          </label>
          <label class="flex items-center gap-2 text-sm cursor-pointer">
            <input
              type="radio"
              value="index"
              bind:group={outputFormat}
              class="accent-primary-500"
            />
            Index
          </label>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="card preset-filled-surface-200-800 overflow-hidden">
          <div class="flex justify-between items-center p-4 border-b border-surface-500">
            <h4 class="text-sm font-semibold text-primary-500">JSON</h4>
            <button class="btn btn-sm preset-filled-primary-500" onclick={() => copyToClipboard(jsonOutput, "JSON")}>
              {copyFeedback === "JSON" ? "복사됨!" : "Copy"}
            </button>
          </div>
          <pre class="p-4 bg-surface-900 text-xs font-mono whitespace-pre-wrap break-words overflow-auto max-h-[400px]">{jsonOutput}</pre>
        </div>

        <div class="card preset-filled-surface-200-800 overflow-hidden">
          <div class="flex justify-between items-center p-4 border-b border-surface-500">
            <h4 class="text-sm font-semibold text-primary-500">텍스트 리스트</h4>
            <button class="btn btn-sm preset-filled-primary-500" onclick={() => copyToClipboard(textOutput, "텍스트")}>
              {copyFeedback === "텍스트" ? "복사됨!" : "Copy"}
            </button>
          </div>
          <pre class="p-4 bg-surface-900 text-xs font-mono whitespace-pre-wrap break-words overflow-auto max-h-[400px]">{textOutput}</pre>
        </div>
      </div>
    </div>
  {/if}

  <div class="mt-8 pt-8 border-t-2 border-surface-500">
    <h3 class="text-base opacity-50 mb-4">프로젝트 관리</h3>
    <div class="flex gap-4 flex-wrap">
      <button class="btn preset-filled-primary-500" onclick={saveProject} disabled={!spriteImage}>
        💾 프로젝트 저장
      </button>
      <label class="btn preset-filled-success-500 cursor-pointer">
        📂 프로젝트 불러오기
        <input
          type="file"
          accept="application/json"
          onchange={handleProjectLoad}
          class="hidden"
        />
      </label>
      <button class="btn preset-filled-warning-500" onclick={clearDescriptions} disabled={cellDescriptions.size === 0}>
        🗑️ 설명만 지우기
      </button>
      <button class="btn preset-filled-error-500" onclick={resetAll} disabled={!spriteImage}>
        ♻️ 전체 초기화
      </button>
    </div>
  </div>

  {#if showAddTagModal}
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-[1000]" onclick={() => showAddTagModal = false}>
      <div class="card preset-filled-surface-200-800 w-[90%] max-w-[400px] shadow-2xl" onclick={(e) => e.stopPropagation()}>
        <div class="flex justify-between items-center p-6 border-b border-surface-500">
          <h3 class="text-base font-semibold text-primary-500">커스텀 태그 추가</h3>
          <button class="btn-icon preset-tonal" onclick={() => showAddTagModal = false}>×</button>
        </div>

        <div class="p-6">
          <input
            type="text"
            bind:value={newTagName}
            placeholder="태그 이름을 입력하세요..."
            onkeydown={(e) => e.key === "Enter" && addCustomTag()}
            class="input w-full text-sm"
          />
        </div>

        <div class="flex justify-end gap-2 p-6 border-t border-surface-500">
          <button class="btn preset-tonal" onclick={() => showAddTagModal = false}>취소</button>
          <button class="btn preset-filled-primary-500" onclick={addCustomTag}>추가</button>
        </div>
      </div>
    </div>
  {/if}
</div>
