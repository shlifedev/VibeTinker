<script lang="ts">
  import { encodeBase64, decodeBase64 } from '$lib/api';
  
  let input = $state('');
  let output = $state('');
  let error = $state('');
  let mode = $state<'encode' | 'decode'>('encode');
  let loading = $state(false);
  
  async function handleConvert() {
    if (!input.trim()) {
      error = 'Please enter some text';
      return;
    }
    
    error = '';
    loading = true;
    
    try {
      if (mode === 'encode') {
        output = await encodeBase64(input);
      } else {
        output = await decodeBase64(input);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'An error occurred';
      output = '';
    } finally {
      loading = false;
    }
  }
  
  function handleSwap() {
    const temp = input;
    input = output;
    output = temp;
    mode = mode === 'encode' ? 'decode' : 'encode';
  }
  
  function handleCopy() {
    if (output) {
      navigator.clipboard.writeText(output);
    }
  }
  
  function handleClear() {
    input = '';
    output = '';
    error = '';
  }
</script>

<svelte:head>
  <title>Base64 Encoder/Decoder - Dev Tools</title>
</svelte:head>

<div class="tool-page">
  <h1>Base64 Encoder/Decoder</h1>
  
  <div class="mode-selector">
    <button 
      class:active={mode === 'encode'} 
      onclick={() => mode = 'encode'}
    >
      Encode
    </button>
    <button 
      class:active={mode === 'decode'} 
      onclick={() => mode = 'decode'}
    >
      Decode
    </button>
  </div>
  
  <div class="converter">
    <div class="input-section">
      <label for="input">
        {mode === 'encode' ? 'Plain Text' : 'Base64 String'}
      </label>
      <textarea 
        id="input" 
        bind:value={input} 
        placeholder={mode === 'encode' ? 'Enter text to encode...' : 'Enter Base64 to decode...'}
        rows="8"
      ></textarea>
    </div>
    
    <div class="actions">
      <button class="primary" onclick={handleConvert} disabled={loading}>
        {loading ? 'Converting...' : (mode === 'encode' ? 'Encode' : 'Decode')}
      </button>
      <button onclick={handleSwap} disabled={!output}>Swap</button>
      <button onclick={handleClear}>Clear</button>
    </div>
    
    {#if error}
      <div class="error">{error}</div>
    {/if}
    
    <div class="output-section">
      <label for="output">
        {mode === 'encode' ? 'Base64 Result' : 'Decoded Text'}
      </label>
      <textarea 
        id="output" 
        value={output} 
        readonly 
        placeholder="Result will appear here..."
        rows="8"
      ></textarea>
      {#if output}
        <button class="copy-btn" onclick={handleCopy}>Copy</button>
      {/if}
    </div>
  </div>
</div>

<style>
  .tool-page {
    max-width: 800px;
  }

  h1 {
    margin-bottom: 1.5rem;
  }

  .mode-selector {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .mode-selector button {
    padding: 0.5rem 1.5rem;
    border: 1px solid #333;
    background: transparent;
    color: #aaa;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .mode-selector button.active {
    background: #0f3460;
    border-color: #0f3460;
    color: #fff;
  }

  .converter {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .input-section,
  .output-section {
    position: relative;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #aaa;
    font-size: 0.9rem;
  }

  textarea {
    width: 100%;
    padding: 1rem;
    background: #16213e;
    border: 1px solid #333;
    border-radius: 4px;
    color: #eee;
    font-family: 'SF Mono', Monaco, monospace;
    font-size: 0.9rem;
    resize: vertical;
  }

  textarea:focus {
    outline: none;
    border-color: #0f3460;
  }

  textarea[readonly] {
    background: #0d1b2a;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    border: 1px solid #333;
    background: #16213e;
    color: #eee;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  button:hover:not(:disabled) {
    background: #1a2744;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.primary {
    background: #0f3460;
    border-color: #0f3460;
  }

  button.primary:hover:not(:disabled) {
    background: #1a4a8a;
  }

  .copy-btn {
    position: absolute;
    top: 2.5rem;
    right: 0.5rem;
    padding: 0.25rem 0.75rem;
    font-size: 0.8rem;
  }

  .error {
    padding: 0.75rem 1rem;
    background: rgba(255, 0, 0, 0.1);
    border: 1px solid rgba(255, 0, 0, 0.3);
    border-radius: 4px;
    color: #ff6b6b;
  }
</style>
