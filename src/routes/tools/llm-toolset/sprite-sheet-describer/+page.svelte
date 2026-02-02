<script lang="ts">
  // State will be added in next task
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
</style>
