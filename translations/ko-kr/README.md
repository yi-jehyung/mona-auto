# mona-auto: 

**mona-auto** 는 화면에 표시되는 이미지를 인식하여 사전 정의된 작업을 자동으로 수행하는 강력한 Windows 자동화 도구입니다.
복잡한 스크립트 없이도 클릭, 키 입력, 텍스트 입력 등의 작업을 손쉽게 설정할 수 있습니다.

---

<a href="https://discord.gg/esYQbEQGXG"><img src="https://img.shields.io/badge/discord-%237289DA.svg?&style=for-the-badge&logo=discord&logoColor=white" alt="Discord Link"/></a>

<div align="center">
  <h3>
    <span> | </span>
    <a href="https://github.com/yi-jehyung/mona-auto/blob/main/translations/ko-kr/README.md"> 한국어 </a>
  </h3>
</div>

## 주요 기능

- **이미지 기반 자동화** (템플릿 매칭)
- 마우스 클릭, 드래그, 스크롤, 키보드 입력 지원
- 텍스트 입력 기능
- 다국어(i18n) 지원
- 디스코드 메시지/스크린샷 전송 기능
- [`egui`](https://github.com/emilk/egui)기반의 직관적인 GUI

---

## 설치 방법

### 다운로드 (권장)

최신 빌드 버전은 [Releases](https://github.com/yi-jehyung/mona-auto/releases) 페이지에서 다운로드할 수 있습니다:

1. [Releases](https://github.com/yi-jehyung/mona-auto/releases) 페이지로 이동
2. `mona-auto.zip` 파일 다운로드
3. 압축 해제
4. `mona-auto.exe` 실행

---

### 소스에서 빌드 (개발자용)

직접 빌드하고 싶다면 아래 절차를 따라 주세요:

[Rust](https://www.rust-lang.org/tools/install) 및 Cargo 설치 후:

```bash
git clone https://github.com/yi-jehyung/mona-auto
cd mona-auto
cargo build --release
```

실행 파일은 다음 위치에 생성됩니다: `target/release/mona-auto.exe`

---

## 사용법
1. `mona-auto.exe` 실행
2. 자동화를 적용할 윈도우 선택
3. 이미지를 추가
4. 화면 영역을 캡처하여 템플릿 등록
5. 원하는 작업(클릭, 입력, 대기 등) 추가
6. 시작 버튼 클릭 → 이미지가 감지되면 자동으로 행동 실행

---

## 스크린샷

| UI | 액션 설정 화면 | 스크린샷 화면 |
|----|----------------------|------------|
| ![Main UI](./screenshots/ui_main.png) | ![Action Setup](./screenshots/ui_action.png) | ![Screenshot](./screenshots/ui_screen.png) |

---

## 로드맵
- [x] 기본 이미지 매칭 및 자동화
- [ ] OCR(문자인식) 지원
- [ ] 스크립트 지원
- [ ] 변수 사용 기능 지원

---

## 기여하기
모든 종류의 기여를 환영합니다!
버그 제보, 기능 제안, UI 개선, 번역 개선 등 다양한 방식으로 참여할 수 있습니다.

[이슈 등록](https://github.com/yi-jehyung/mona-auto/issues) 또는 PR을 제출해 주세요.

**디스코드 커뮤니티**에 참여하여 질문하거나 아이디어를 나눠보세요
<a href="https://discord.gg/esYQbEQGXG"><img src="https://img.shields.io/badge/discord-%237289DA.svg?&style=for-the-badge&logo=discord&logoColor=white" alt="Discord Link"/></a>

---
> ⚠ **주의 사항**  
> 이 애플리케이션은 현재 개발 중인 **베타 버전**이며, 일부 기능이 불안정할 수 있습니다.  
> **Windows 전용**으로 제작되었으며, 다른 운영체제에서는 정상적으로 작동하지 않을 수 있습니다.  
> 사용 중 발생한 문제나 손해에 대해 **개발자는 어떠한 책임도 지지 않습니다**.