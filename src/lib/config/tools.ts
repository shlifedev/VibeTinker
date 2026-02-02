export interface Tool {
  id: string
  name: string
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
      { id: "base64", name: "Base64", path: "/tools/encoding/base64" }
    ]
  },
  {
    id: "llm-toolset",
    name: "LLM 도구",
    tools: [
      { id: "motion-descriptor", name: "Motion Descriptor", path: "/tools/llm-toolset/motion-descriptor" }
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
