<script lang="ts">
  import { onMount } from "svelte"

  type HitboxType = "body" | "attack" | "hurt" | "trigger" | "custom"

  interface Hitbox {
    id: string
    name: string
    type: HitboxType
    shape: "rect" | "circle"
    x: number
    y: number
    width?: number
    height?: number
    radius?: number
    color: string
    visible: boolean
  }

  const typeColors: Record<HitboxType, string> = {
    body: "#22c55e",
    attack: "#ef4444",
    hurt: "#f97316",
    trigger: "#3b82f6",
    custom: "#a855f7"
  }

  let canvas: HTMLCanvasElement
  let ctx: CanvasRenderingContext2D | null = null
  let image: HTMLImageElement | null = null
  let imageLoaded = $state(false)

  let hitboxes = $state<Hitbox[]>([])
  let selectedHitboxId = $state<string | null>(null)
  let drawMode = $state<"rect" | "circle">("rect")
  let currentType = $state<HitboxType>("body")
  let isDrawing = $state(false)
  let isPanning = $state(false)
  let isResizing = $state(false)
  let resizeHandle = $state<string | null>(null)

  let zoom = $state(1)
  let panX = $state(0)
  let panY = $state(0)
  let startX = $state(0)
  let startY = $state(0)
  let mouseX = $state(0)
  let mouseY = $state(0)
  let tempHitbox = $state<Hitbox | null>(null)

  let jsonOutput = $derived(JSON.stringify({
    imageWidth: image?.width || 0,
    imageHeight: image?.height || 0,
    hitboxes: hitboxes.map(h => ({
      name: h.name,
      type: h.type,
      shape: h.shape,
      x: Math.round(h.x),
      y: Math.round(h.y),
      ...(h.shape === "rect" ? {
        width: Math.round(h.width!),
        height: Math.round(h.height!)
      } : {
        radius: Math.round(h.radius!)
      })
    }))
  }, null, 2))

  let selectedHitbox = $derived(hitboxes.find(h => h.id === selectedHitboxId))

  onMount(() => {
    ctx = canvas.getContext("2d")
    render()
  })

  function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement
    const file = target.files?.[0]
    if (file) loadImage(file)
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault()
    const file = event.dataTransfer?.files[0]
    if (file?.type.startsWith("image/")) loadImage(file)
  }

  function loadImage(file: File) {
    const reader = new FileReader()
    reader.onload = (e) => {
      const img = new Image()
      img.onload = () => {
        image = img
        imageLoaded = true
        resetView()
        render()
      }
      img.src = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }

  function resetView() {
    if (!image) return
    const padding = 50
    const scaleX = (canvas.width - padding * 2) / image.width
    const scaleY = (canvas.height - padding * 2) / image.height
    zoom = Math.min(scaleX, scaleY, 4)
    panX = (canvas.width - image.width * zoom) / 2
    panY = (canvas.height - image.height * zoom) / 2
  }

  function toCanvasCoords(clientX: number, clientY: number): [number, number] {
    const rect = canvas.getBoundingClientRect()
    const x = (clientX - rect.left - panX) / zoom
    const y = (clientY - rect.top - panY) / zoom
    return [x, y]
  }

  function handleMouseDown(event: MouseEvent) {
    const [x, y] = toCanvasCoords(event.clientX, event.clientY)

    if (event.button === 1 || event.button === 2 || event.ctrlKey) {
      isPanning = true
      startX = event.clientX - panX
      startY = event.clientY - panY
      return
    }

    const handle = getResizeHandle(x, y)
    if (handle) {
      isResizing = true
      resizeHandle = handle
      startX = x
      startY = y
      return
    }

    const clickedHitbox = getHitboxAt(x, y)
    if (clickedHitbox) {
      selectedHitboxId = clickedHitbox.id
      startX = x
      startY = y
      render()
      return
    }

    selectedHitboxId = null
    isDrawing = true
    startX = x
    startY = y
    tempHitbox = {
      id: crypto.randomUUID(),
      name: `${currentType}_${hitboxes.length + 1}`,
      type: currentType,
      shape: drawMode,
      x,
      y,
      width: 0,
      height: 0,
      radius: 0,
      color: typeColors[currentType],
      visible: true
    }
  }

  function handleMouseMove(event: MouseEvent) {
    const [x, y] = toCanvasCoords(event.clientX, event.clientY)
    mouseX = Math.round(x)
    mouseY = Math.round(y)

    if (isPanning) {
      panX = event.clientX - startX
      panY = event.clientY - startY
      render()
      return
    }

    if (isResizing && selectedHitbox && resizeHandle) {
      const dx = x - startX
      const dy = y - startY

      if (selectedHitbox.shape === "rect") {
        if (resizeHandle.includes("n")) {
          selectedHitbox.y = startY + dy
          selectedHitbox.height = (selectedHitbox.height || 0) - dy
        }
        if (resizeHandle.includes("s")) {
          selectedHitbox.height = (selectedHitbox.height || 0) + dy
        }
        if (resizeHandle.includes("w")) {
          selectedHitbox.x = startX + dx
          selectedHitbox.width = (selectedHitbox.width || 0) - dx
        }
        if (resizeHandle.includes("e")) {
          selectedHitbox.width = (selectedHitbox.width || 0) + dx
        }

        if (selectedHitbox.width && selectedHitbox.width < 0) {
          selectedHitbox.x += selectedHitbox.width
          selectedHitbox.width = -selectedHitbox.width
        }
        if (selectedHitbox.height && selectedHitbox.height < 0) {
          selectedHitbox.y += selectedHitbox.height
          selectedHitbox.height = -selectedHitbox.height
        }
      } else {
        const dist = Math.sqrt(Math.pow(x - selectedHitbox.x, 2) + Math.pow(y - selectedHitbox.y, 2))
        selectedHitbox.radius = Math.max(1, dist)
      }

      startX = x
      startY = y
      render()
      return
    }

    if (isDrawing && tempHitbox) {
      if (drawMode === "rect") {
        tempHitbox.width = x - startX
        tempHitbox.height = y - startY
      } else {
        const dx = x - startX
        const dy = y - startY
        tempHitbox.radius = Math.sqrt(dx * dx + dy * dy)
      }
      render()
      return
    }

    if (selectedHitbox && !isResizing) {
      const handle = getResizeHandle(x, y)
      canvas.style.cursor = handle ? getCursorForHandle(handle) : getHitboxAt(x, y) ? "move" : "crosshair"
    } else {
      canvas.style.cursor = getHitboxAt(x, y) ? "pointer" : "crosshair"
    }
  }

  function handleMouseUp(event: MouseEvent) {
    if (isPanning) {
      isPanning = false
      return
    }

    if (isResizing) {
      isResizing = false
      resizeHandle = null
      return
    }

    if (isDrawing && tempHitbox) {
      if (drawMode === "rect" && Math.abs(tempHitbox.width || 0) > 2 && Math.abs(tempHitbox.height || 0) > 2) {
        if ((tempHitbox.width || 0) < 0) {
          tempHitbox.x += tempHitbox.width || 0
          tempHitbox.width = -(tempHitbox.width || 0)
        }
        if ((tempHitbox.height || 0) < 0) {
          tempHitbox.y += tempHitbox.height || 0
          tempHitbox.height = -(tempHitbox.height || 0)
        }
        hitboxes.push(tempHitbox)
        selectedHitboxId = tempHitbox.id
      } else if (drawMode === "circle" && (tempHitbox.radius || 0) > 2) {
        hitboxes.push(tempHitbox)
        selectedHitboxId = tempHitbox.id
      }
      tempHitbox = null
      isDrawing = false
      render()
    }
  }

  function handleWheel(event: WheelEvent) {
    event.preventDefault()
    const [x, y] = toCanvasCoords(event.clientX, event.clientY)
    const oldZoom = zoom
    zoom *= event.deltaY > 0 ? 0.9 : 1.1
    zoom = Math.max(0.1, Math.min(10, zoom))
    panX = event.clientX - (event.clientX - panX) * (zoom / oldZoom)
    panY = event.clientY - (event.clientY - panY) * (zoom / oldZoom)
    render()
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Delete" && selectedHitboxId) {
      deleteHitbox(selectedHitboxId)
      event.preventDefault()
    }
  }

  function getHitboxAt(x: number, y: number): Hitbox | null {
    for (let i = hitboxes.length - 1; i >= 0; i--) {
      const h = hitboxes[i]
      if (!h.visible) continue

      if (h.shape === "rect") {
        if (x >= h.x && x <= h.x + (h.width || 0) && y >= h.y && y <= h.y + (h.height || 0)) {
          return h
        }
      } else {
        const dx = x - h.x
        const dy = y - h.y
        if (Math.sqrt(dx * dx + dy * dy) <= (h.radius || 0)) {
          return h
        }
      }
    }
    return null
  }

  function getResizeHandle(x: number, y: number): string | null {
    if (!selectedHitbox || !selectedHitbox.visible) return null

    const handleSize = 8 / zoom

    if (selectedHitbox.shape === "rect") {
      const left = selectedHitbox.x
      const right = selectedHitbox.x + (selectedHitbox.width || 0)
      const top = selectedHitbox.y
      const bottom = selectedHitbox.y + (selectedHitbox.height || 0)

      const handles = [
        { name: "nw", x: left, y: top },
        { name: "n", x: (left + right) / 2, y: top },
        { name: "ne", x: right, y: top },
        { name: "e", x: right, y: (top + bottom) / 2 },
        { name: "se", x: right, y: bottom },
        { name: "s", x: (left + right) / 2, y: bottom },
        { name: "sw", x: left, y: bottom },
        { name: "w", x: left, y: (top + bottom) / 2 }
      ]

      for (const handle of handles) {
        if (Math.abs(x - handle.x) < handleSize && Math.abs(y - handle.y) < handleSize) {
          return handle.name
        }
      }
    } else {
      const dist = Math.sqrt(Math.pow(x - selectedHitbox.x, 2) + Math.pow(y - selectedHitbox.y, 2))
      if (Math.abs(dist - (selectedHitbox.radius || 0)) < handleSize) {
        return "radius"
      }
    }

    return null
  }

  function getCursorForHandle(handle: string): string {
    const cursors: Record<string, string> = {
      nw: "nw-resize",
      n: "n-resize",
      ne: "ne-resize",
      e: "e-resize",
      se: "se-resize",
      s: "s-resize",
      sw: "sw-resize",
      w: "w-resize",
      radius: "ew-resize"
    }
    return cursors[handle] || "default"
  }

  function deleteHitbox(id: string) {
    hitboxes = hitboxes.filter(h => h.id !== id)
    if (selectedHitboxId === id) selectedHitboxId = null
    render()
  }

  function render() {
    if (!ctx) return

    ctx.fillStyle = "#1a1a1a"
    ctx.fillRect(0, 0, canvas.width, canvas.height)

    if (!image || !imageLoaded) return

    ctx.save()
    ctx.translate(panX, panY)
    ctx.scale(zoom, zoom)

    drawCheckerboard()

    ctx.drawImage(image, 0, 0)

    hitboxes.filter(h => h.visible).forEach(h => drawHitbox(h, false))

    if (tempHitbox) drawHitbox(tempHitbox, false)

    if (selectedHitbox && selectedHitbox.visible) {
      drawHitbox(selectedHitbox, true)
      drawResizeHandles(selectedHitbox)
    }

    ctx.restore()
  }

  function drawCheckerboard() {
    if (!ctx || !image) return

    const size = 8
    ctx.fillStyle = "#2a2a2a"
    ctx.fillRect(0, 0, image.width, image.height)

    ctx.fillStyle = "#3a3a3a"
    for (let y = 0; y < image.height; y += size) {
      for (let x = 0; x < image.width; x += size) {
        if ((x / size + y / size) % 2 === 0) {
          ctx.fillRect(x, y, size, size)
        }
      }
    }
  }

  function drawHitbox(h: Hitbox, isSelected: boolean) {
    if (!ctx) return

    ctx.strokeStyle = h.color
    ctx.fillStyle = h.color + "40"
    ctx.lineWidth = (isSelected ? 2.5 : 1.5) / zoom

    if (h.shape === "rect") {
      ctx.fillRect(h.x, h.y, h.width || 0, h.height || 0)
      ctx.strokeRect(h.x, h.y, h.width || 0, h.height || 0)
    } else {
      ctx.beginPath()
      ctx.arc(h.x, h.y, h.radius || 0, 0, Math.PI * 2)
      ctx.fill()
      ctx.stroke()
    }

    if (isSelected) {
      ctx.strokeStyle = "#ffffff"
      ctx.lineWidth = 1 / zoom
      ctx.setLineDash([4 / zoom, 4 / zoom])
      if (h.shape === "rect") {
        ctx.strokeRect(h.x, h.y, h.width || 0, h.height || 0)
      } else {
        ctx.beginPath()
        ctx.arc(h.x, h.y, h.radius || 0, 0, Math.PI * 2)
        ctx.stroke()
      }
      ctx.setLineDash([])
    }
  }

  function drawResizeHandles(h: Hitbox) {
    if (!ctx) return

    const handleSize = 6 / zoom
    ctx.fillStyle = "#ffffff"
    ctx.strokeStyle = "#000000"
    ctx.lineWidth = 1 / zoom

    if (h.shape === "rect") {
      const left = h.x
      const right = h.x + (h.width || 0)
      const top = h.y
      const bottom = h.y + (h.height || 0)

      const handles = [
        [left, top],
        [(left + right) / 2, top],
        [right, top],
        [right, (top + bottom) / 2],
        [right, bottom],
        [(left + right) / 2, bottom],
        [left, bottom],
        [left, (top + bottom) / 2]
      ]

      handles.forEach(([x, y]) => {
        ctx!.fillRect(x - handleSize, y - handleSize, handleSize * 2, handleSize * 2)
        ctx!.strokeRect(x - handleSize, y - handleSize, handleSize * 2, handleSize * 2)
      })
    } else {
      const angle = Math.PI / 4
      for (let i = 0; i < 8; i++) {
        const a = angle * i
        const x = h.x + Math.cos(a) * (h.radius || 0)
        const y = h.y + Math.sin(a) * (h.radius || 0)
        ctx!.fillRect(x - handleSize, y - handleSize, handleSize * 2, handleSize * 2)
        ctx!.strokeRect(x - handleSize, y - handleSize, handleSize * 2, handleSize * 2)
      }
    }
  }

  function copyJson() {
    navigator.clipboard.writeText(jsonOutput)
  }

  function clearAll() {
    if (confirm("Clear all hitboxes?")) {
      hitboxes = []
      selectedHitboxId = null
      render()
    }
  }

  $effect(() => {
    render()
  })
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="p-6 h-screen flex flex-col">
  <h1 class="text-3xl font-bold mb-6">Hitbox Editor</h1>

  {#if !imageLoaded}
    <div class="flex items-center justify-center h-[calc(100vh-12rem)]">
      <div
        class="card p-12 text-center"
        ondrop={handleDrop}
        ondragover={(e) => e.preventDefault()}
      >
        <h2 class="text-xl font-semibold mb-4">Load a sprite image</h2>
        <p class="text-surface-400 mb-6">Drag & drop or click to browse</p>
        <label class="btn preset-filled-primary-500 cursor-pointer">
          Choose File
          <input type="file" accept="image/*" class="hidden" onchange={handleFileSelect} />
        </label>
      </div>
    </div>
  {:else}
    <div class="grid grid-cols-[1fr_350px] gap-4 flex-1 overflow-hidden">
      <!-- Canvas -->
      <div class="flex flex-col">
        <div class="card p-4 flex-1 flex flex-col">
          <div class="flex items-center justify-between mb-2">
            <div class="text-sm text-surface-400">
              Mouse: ({mouseX}, {mouseY}) | Zoom: {(zoom * 100).toFixed(0)}%
            </div>
            <button class="btn btn-sm preset-tonal" onclick={resetView}>Reset View</button>
          </div>

          <canvas
            bind:this={canvas}
            width="1200"
            height="800"
            class="bg-surface-900 rounded-lg flex-1 cursor-crosshair"
            onmousedown={handleMouseDown}
            onmousemove={handleMouseMove}
            onmouseup={handleMouseUp}
            onwheel={handleWheel}
            oncontextmenu={(e) => e.preventDefault()}
          ></canvas>
        </div>
      </div>

      <!-- Right Panel -->
      <div class="flex flex-col gap-4 overflow-y-auto">
        <!-- Controls -->
        <div class="card p-4">
          <h3 class="text-lg font-semibold mb-3">Draw Mode</h3>
          <div class="flex gap-2 mb-4">
            <button
              class="btn flex-1 {drawMode === 'rect' ? 'preset-filled-primary-500' : 'preset-tonal'}"
              onclick={() => drawMode = "rect"}
            >
              Rectangle
            </button>
            <button
              class="btn flex-1 {drawMode === 'circle' ? 'preset-filled-primary-500' : 'preset-tonal'}"
              onclick={() => drawMode = "circle"}
            >
              Circle
            </button>
          </div>

          <h3 class="text-lg font-semibold mb-3">Hitbox Type</h3>
          <div class="flex flex-col gap-2">
            {#each Object.keys(typeColors) as type}
              <button
                class="btn justify-start {currentType === type ? 'preset-filled-primary-500' : 'preset-filled-surface-200-800'}"
                onclick={() => currentType = type as HitboxType}
              >
                <div class="w-4 h-4 rounded" style="background-color: {typeColors[type as HitboxType]}"></div>
                <span class="capitalize">{type}</span>
              </button>
            {/each}
          </div>

          <div class="mt-4 pt-4 border-t border-surface-700">
            <button class="btn preset-filled-error-500 w-full" onclick={clearAll}>Clear All</button>
          </div>
        </div>

        <!-- Hitbox List -->
        <div class="card p-4 flex-1 overflow-y-auto">
          <h3 class="text-lg font-semibold mb-3">Hitboxes ({hitboxes.length})</h3>

          {#if hitboxes.length === 0}
            <p class="text-surface-400 text-sm">No hitboxes yet. Draw on the canvas to add one.</p>
          {:else}
            <div class="flex flex-col gap-2">
              {#each hitboxes as hitbox (hitbox.id)}
                <div
                  class="card p-3 cursor-pointer hover:preset-filled-surface-200-800 transition-colors {selectedHitboxId === hitbox.id ? 'ring-2 ring-primary-500' : ''}"
                  onclick={() => {
                    selectedHitboxId = hitbox.id
                    render()
                  }}
                >
                  <div class="flex items-center justify-between mb-2">
                    <input
                      type="text"
                      bind:value={hitbox.name}
                      class="bg-transparent border-none p-0 text-sm font-semibold flex-1"
                      onclick={(e) => e.stopPropagation()}
                    />
                    <button
                      class="btn btn-sm preset-filled-error-500"
                      onclick={(e) => {
                        e.stopPropagation()
                        deleteHitbox(hitbox.id)
                      }}
                    >
                      ×
                    </button>
                  </div>

                  <div class="flex items-center gap-2 text-xs text-surface-400">
                    <span class="capitalize">{hitbox.type}</span>
                    <span>•</span>
                    <span class="capitalize">{hitbox.shape}</span>
                    <span>•</span>
                    {#if hitbox.shape === "rect"}
                      <span>{Math.round(hitbox.width || 0)}×{Math.round(hitbox.height || 0)}</span>
                    {:else}
                      <span>r={Math.round(hitbox.radius || 0)}</span>
                    {/if}
                  </div>

                  <div class="flex items-center gap-2 mt-2">
                    <input
                      type="color"
                      bind:value={hitbox.color}
                      class="w-8 h-8 rounded cursor-pointer"
                      onclick={(e) => e.stopPropagation()}
                      onchange={() => render()}
                    />
                    <label class="flex items-center gap-2 cursor-pointer">
                      <input
                        type="checkbox"
                        bind:checked={hitbox.visible}
                        onclick={(e) => e.stopPropagation()}
                        onchange={() => render()}
                      />
                      <span class="text-xs">Visible</span>
                    </label>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <!-- JSON Output -->
        <div class="card p-4">
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-lg font-semibold">JSON Output</h3>
            <button class="btn btn-sm preset-filled-primary-500" onclick={copyJson}>Copy</button>
          </div>
          <textarea
            readonly
            value={jsonOutput}
            class="textarea w-full h-48 font-mono text-xs"
          ></textarea>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  input[type="text"] {
    outline: none;
  }
  input[type="text"]:focus {
    outline: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 2px;
    padding: 0 4px;
  }
</style>
