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
        output = await commands.base64Decode(input)
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

<div class="tool-page">
  <header class="tool-header">
    <h1>Base64</h1>
    <p>문자열을 Base64로 인코딩/디코딩합니다.</p>
  </header>

  <div class="tool-controls">
    <div class="mode-toggle">
      <button class:active={mode === "encode"} onclick={() => mode = "encode"}>
        Encode
      </button>
      <button class:active={mode === "decode"} onclick={() => mode = "decode"}>
        Decode
      </button>
    </div>
    <div class="actions">
      <button onclick={handleSwap}>↕ Swap</button>
      <button onclick={handleClear}>Clear</button>
    </div>
  </div>

  <div class="tool-content">
    <div class="panel">
      <label>Input</label>
      <textarea
        bind:value={input}
        placeholder={mode === "encode" ? "인코딩할 텍스트를 입력하세요..." : "디코딩할 Base64를 입력하세요..."}
      ></textarea>
    </div>

    <div class="convert-button">
      <button onclick={handleConvert}>
        {mode === "encode" ? "Encode →" : "Decode →"}
      </button>
    </div>

    <div class="panel">
      <div class="output-header">
        <label>Output</label>
        <button class="copy-btn" onclick={handleCopy} disabled={!output}>Copy</button>
      </div>
      <textarea
        readonly
        value={output}
        placeholder="결과가 여기에 표시됩니다..."
      ></textarea>
    </div>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}
</div>

<style>
  .tool-page {
    padding: 2rem;
    max-width: 1200px;
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

  .tool-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
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
  }

  .mode-toggle button.active {
    background: #0e639c;
    border-color: #0e639c;
    color: #fff;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .actions button {
    padding: 0.5rem 1rem;
    background: #333;
    border: 1px solid #444;
    color: #ccc;
    cursor: pointer;
    border-radius: 4px;
  }

  .actions button:hover {
    background: #444;
  }

  .tool-content {
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 1rem;
    align-items: stretch;
  }

  .panel {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .panel label {
    font-size: 0.9rem;
    color: #888;
  }

  .panel textarea {
    flex: 1;
    min-height: 300px;
    padding: 1rem;
    background: #1e1e1e;
    border: 1px solid #333;
    border-radius: 4px;
    color: #e0e0e0;
    font-family: monospace;
    font-size: 0.9rem;
    resize: vertical;
  }

  .panel textarea:focus {
    outline: none;
    border-color: #0e639c;
  }

  .output-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .copy-btn {
    padding: 0.25rem 0.5rem;
    background: #333;
    border: 1px solid #444;
    color: #ccc;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.8rem;
  }

  .copy-btn:hover:not(:disabled) {
    background: #444;
  }

  .copy-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .convert-button {
    display: flex;
    align-items: center;
  }

  .convert-button button {
    padding: 1rem;
    background: #0e639c;
    border: none;
    color: #fff;
    cursor: pointer;
    border-radius: 4px;
    font-weight: 600;
  }

  .convert-button button:hover {
    background: #1177bb;
  }

  .error {
    margin-top: 1rem;
    padding: 1rem;
    background: #5a1d1d;
    border: 1px solid #8b2d2d;
    border-radius: 4px;
    color: #f88;
  }
</style>
