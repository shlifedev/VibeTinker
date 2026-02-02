<script lang="ts">
  import { commands } from "$lib/bindings"

  let input = $state("")
  let output = $state("")
  let mode = $state<"encode" | "decode">("encode")
  let error = $state("")

  async function handleConvert() {
    error = ""
    try {
      if (mode === "encode") {
        output = await commands.base64Encode(input)
      } else {
        const result = await commands.base64Decode(input)
        if (result.status === "ok") {
          output = result.data
        } else {
          error = JSON.stringify(result.error)
          output = ""
        }
      }
    } catch (e) {
      error = String(e)
      output = ""
    }
  }

  function handleSwap() {
    mode = mode === "encode" ? "decode" : "encode"
    const temp = input
    input = output
    output = temp
    error = ""
  }

  function handleClear() {
    input = ""
    output = ""
    error = ""
  }

  async function handleCopy() {
    if (output) {
      await navigator.clipboard.writeText(output)
    }
  }
</script>

<div class="p-8 max-w-[1200px] mx-auto">
  <header class="mb-6">
    <h1 class="text-2xl font-bold mb-2">Base64</h1>
    <p class="opacity-50">문자열을 Base64로 인코딩/디코딩합니다.</p>
  </header>

  <div class="flex justify-between items-center mb-4">
    <div class="flex gap-2">
      <button
        class="btn {mode === 'encode' ? 'preset-filled-primary-500' : 'preset-tonal'}"
        onclick={() => mode = "encode"}
      >
        Encode
      </button>
      <button
        class="btn {mode === 'decode' ? 'preset-filled-primary-500' : 'preset-tonal'}"
        onclick={() => mode = "decode"}
      >
        Decode
      </button>
    </div>
    <div class="flex gap-2">
      <button class="btn preset-tonal" onclick={handleSwap}>↕ Swap</button>
      <button class="btn preset-tonal" onclick={handleClear}>Clear</button>
    </div>
  </div>

  <div class="grid grid-cols-[1fr_auto_1fr] gap-4 items-stretch">
    <div class="flex flex-col gap-2">
      <label class="text-sm opacity-50">Input</label>
      <textarea
        bind:value={input}
        placeholder={mode === "encode" ? "인코딩할 텍스트를 입력하세요..." : "디코딩할 Base64를 입력하세요..."}
        class="textarea min-h-[300px] resize-y font-mono text-sm"
      ></textarea>
    </div>

    <div class="flex items-center">
      <button class="btn preset-filled-primary-500 font-semibold" onclick={handleConvert}>
        {mode === "encode" ? "Encode →" : "Decode →"}
      </button>
    </div>

    <div class="flex flex-col gap-2">
      <div class="flex justify-between items-center">
        <label class="text-sm opacity-50">Output</label>
        <button class="btn btn-sm preset-tonal" onclick={handleCopy} disabled={!output}>Copy</button>
      </div>
      <textarea
        readonly
        value={output}
        placeholder="결과가 여기에 표시됩니다..."
        class="textarea min-h-[300px] resize-y font-mono text-sm"
      ></textarea>
    </div>
  </div>

  {#if error}
    <div class="card preset-filled-error-500 p-4 mt-4">{error}</div>
  {/if}
</div>
