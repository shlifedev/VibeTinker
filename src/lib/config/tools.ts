export interface Tool {
  id: string
  name: string
  description: string
  path: string
}

export interface ToolCategory {
  id: string
  name: string
  tools: Tool[]
}

export const toolCategories: ToolCategory[] = [
  {
    id: "encoding",
    name: "인코딩",
    tools: [
      { id: "base64", name: "Base64", description: "Base64 인코딩/디코딩 도구", path: "/tools/encoding/base64" }
    ]
  },
  {
    id: "llm-toolset",
    name: "LLM 도구",
    tools: [
      { id: "motion-descriptor", name: "Motion Descriptor", description: "마우스 모션을 LLM 프롬프트로 변환", path: "/tools/llm-toolset/motion-descriptor" },
      { id: "sprite-sheet-describer", name: "Sprite Sheet Describer", description: "스프라이트 시트 그리드 주석 도구", path: "/tools/llm-toolset/sprite-sheet-describer" }
    ]
  },
  {
    id: "gamedev",
    name: "게임 개발",
    tools: [
      { id: "tween-visualizer", name: "Tween Visualizer", description: "이징 함수 시각화 및 비교 도구", path: "/tools/gamedev/tween-visualizer" },
      { id: "hitbox-editor", name: "Hitbox Editor", description: "스프라이트 충돌 영역 편집 + JSON 출력", path: "/tools/gamedev/hitbox-editor" }
    ]
  }
]

export function findToolByPath(path: string): Tool | undefined {
  for (const category of toolCategories) {
    const tool = category.tools.find((t: Tool) => t.path === path)
    if (tool) return tool
  }
  return undefined
}

export function findCategoryById(id: string): ToolCategory | undefined {
  return toolCategories.find((c: ToolCategory) => c.id === id)
}
