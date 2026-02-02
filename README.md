# VibeTinker

LLM 지원을 통해 확장 가능한 개발자 도구 모음

## 소개

VibeTinker는 사이드바 인터페이스로 다양한 개발 도구를 관리할 수 있는 데스크톱 애플리케이션입니다. LLM의 도움을 받아 새로운 도구를 추가하고, 기존 도구를 커스터마이징하여 나만의 워크벤치를 만들 수 있습니다.

## 주요 기능

- 사이드바 네비게이션으로 도구 구성
- LLM 보조로 새 도구 빠르게 추가
- Tauri + Rust 백엔드로 네이티브 성능
- 카테고리별 도구 분류

## 내장 도구

### 인코딩
- **Base64** - 텍스트 Base64 인코딩/디코딩

### LLM 도구
- **Motion Descriptor** - 마우스 제스처를 자연어로 변환

### 게임 개발
- **Sprite Sheet Describer** - 스프라이트 시트에 설명 추가, JSON 출력
- **Tween Visualizer** - 30+ 이징 함수 시각화 및 비교

## 시작하기

### 필수 요구사항
- [Bun](https://bun.sh)
- [Rust](https://rustup.rs)

### 개발 모드

```bash
bun install
bun run tauri dev
```

### 프로덕션 빌드

```bash
bun run tauri build
```

## 기술 스택

- Frontend: SvelteKit 2.x + Svelte 5, Skeleton UI
- Backend: Rust + Tauri 2.0
- Type Safety: tauri-specta
- Styling: Tailwind CSS 4

## 프로젝트 철학

VibeTinker는 정해진 도구 모음이 아닌, 도구를 만드는 프레임워크입니다. LLM을 활용해 필요한 유틸리티를 빠르게 구축하고, 자신의 워크플로우에 맞는 워크벤치를 만들어보세요.

## 새 도구 추가

1. `src/lib/config/tools.ts`에 도구 정의
2. `src/routes/tools/{category}/{tool-id}/+page.svelte` 생성
3. (선택) `src-tauri/src/tools/`에 백엔드 커맨드 추가

자세한 개발 가이드는 [CLAUDE.md](./Claude.md)를 참고하세요.

## 프로젝트 구조

```
VibeTinker/
├── src/                    # Frontend (SvelteKit)
│   ├── routes/tools/      # 도구 구현
│   ├── lib/components/    # UI 컴포넌트
│   └── lib/config/        # 도구 메타데이터
├── src-tauri/             # Backend (Rust)
│   ├── src/tools/         # 백엔드 커맨드
│   └── src/modules/       # 유틸리티
└── .github/workflows/     # CI/CD
```

## 릴리스

버전 태그 푸시 시 GitHub Actions가 자동으로 빌드합니다:

```bash
git tag v1.0.0
git push origin v1.0.0
```

빌드 결과물:
- Windows: `.exe` 설치 파일
- macOS: Universal `.dmg` + `.app` (Intel + Apple Silicon)
- Linux: `.AppImage` + `.deb`

## 기여

새로운 도구와 개선 사항에 대한 기여를 환영합니다.

1. 저장소 포크
2. 기능 브랜치 생성
3. 프로젝트 구조에 맞춰 도구 추가
4. Pull Request 제출

## 라이선스

MIT

## 감사

- [Tauri](https://tauri.app)
- [Skeleton UI](https://skeleton.dev)
